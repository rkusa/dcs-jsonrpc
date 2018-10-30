use crate::error::Error;
use crate::server::Server;
use hlua51::{Lua, LuaFunction, LuaTable};
use serde_json::Value;

static mut INITIALIZED: bool = false;
static mut SERVER: Option<Server> = None;

pub fn init(lua: &mut Lua<'_>) -> Result<(), Error> {
    unsafe {
        if INITIALIZED {
            return Ok(());
        }
        INITIALIZED = true;
    }

    // init logging
    use log::LevelFilter;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};

    #[cfg(not(test))]
    let writedir = {
        let mut lfs: LuaTable<_> = get!(lua, "lfs")?;
        let mut writedir: LuaFunction<_> = get!(lfs, "writedir")?;
        let writedir: String = writedir.call()?;
        writedir
    };
    #[cfg(test)]
    let writedir = String::from("./");
    let log_file = writedir + "Logs/dcsjsonrpc.log";

    let requests = FileAppender::builder()
        .append(false)
        .build(log_file)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(requests)))
        .logger(Logger::builder().build("dcsjsonrpc", LevelFilter::Debug))
        .build(Root::builder().appender("file").build(LevelFilter::Off))
        .unwrap();

    log4rs::init_config(config).unwrap();

    Ok(())
}

pub fn start(mut lua: &mut Lua<'_>) -> Result<(), Error> {
    if unsafe { SERVER.is_some() } {
        return Ok(());
    }

    init(&mut lua)?;

    info!("Starting ...");

    let server = Server::new();
    unsafe { SERVER = Some(server) }

    info!("Started ...");

    Ok(())
}

pub fn stop() {
    info!("Stopping ...");

    if let Some(server) = unsafe { SERVER.take() } {
        server.stop();
    }
}

pub fn try_next(lua: &mut Lua<'_>) -> Result<bool, Error> {
    let mut callback: LuaFunction<_> = pop!(lua, "callback")?;

    if let Some(server) = unsafe { &SERVER } {
        if let Some(mut next) = server.try_next() {
            let params = next.req.take_params().map(|params| params.to_string());
            let mut result: LuaTable<_> =
                callback.call_with_args((next.req.method().clone(), params))?;
            if let Some(err) = result.get("error") {
                next.error(err);
            } else if let Some(res) = result.get::<String, _, _>("result") {
                let res: Value = serde_json::from_str(&res)
                    .map_err(|err| Error::SerializeResult(err, res.to_string()))?;
                next.success(res);
            }

            return Ok(true);
        }
    }

    Ok(false)
}

pub fn broadcast(lua: &mut Lua<'_>) -> Result<(), Error> {
    let payload: String = pop!(lua, "payload")?;
    let channel: String = pop!(lua, "channel")?;
    let payload: Option<Value> =
        serde_json::from_str(&payload).map_err(|err| Error::SerializeBroadcast(err, payload))?;

    if let Some(server) = unsafe { &SERVER } {
        server.broadcast(&channel, payload);
    }

    Ok(())
}
