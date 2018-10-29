use crate::error::Error;
use crate::server::Server;
use hlua51::{Lua, LuaFunction, LuaTable};
use serde_json::Value;

static mut INITIALIZED: bool = false;
static mut SERVER: Option<Server> = None;

pub fn init(_lua: &mut Lua<'_>) -> Result<(), Error> {
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

    // let mut lfs: LuaTable<_> = get!(lua, "lfs")?;
    // let mut writedir: LuaFunction<_> = get!(lfs, "writedir")?;
    // let writedir: String = writedir.call()?;
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

pub fn try_next(lua: &mut Lua<'_>) -> Result<bool, Error> {
    let mut callback: LuaFunction<_> = pop!(lua, "callback")?;

    if let Some(server) = unsafe { &SERVER } {
        if let Some(mut next) = server.try_next() {
            // TODO: unwrap
            let params = next
                .req
                .params
                .take()
                .map(|params| serde_json::to_string(&params).unwrap());
            let mut result: LuaTable<_> = callback
                .call_with_args((next.req.method.clone(), params))
                .unwrap();
            if let Some(err) = result.get("error") {
                next.error(err);
            } else if let Some(res) = result.get::<String, _, _>("result") {
                // TODO: unwrap
                let res: Value = serde_json::from_str(&res).unwrap();
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
    // TODO: unwrap
    let payload: Option<Value> = serde_json::from_str(&payload).unwrap();

    if let Some(server) = unsafe { &SERVER } {
        server.broadcast(&channel, payload);
    }

    Ok(())
}
