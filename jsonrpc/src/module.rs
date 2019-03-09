use crate::error::Error;
use crate::server::Server;
use lua51 as ffi;
use lua51::lua_pop;
use serde_json::Value;
use std::ffi::{CStr, CString};

static mut INITIALIZED: bool = false;
static mut SERVER: Option<Server> = None;

pub fn init(l: *mut ffi::lua_State) -> Result<(), Error> {
    unsafe {
        if INITIALIZED {
            return Ok(());
        }
        INITIALIZED = true;
    }

    // init logging
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};

    // get lfs.writedir()
    let writedir = unsafe {
        ffi::lua_getfield(l, ffi::LUA_GLOBALSINDEX, const_cstr!("lfs").as_ptr());
        if ffi::lua_istable(l, -1) {
            ffi::lua_getfield(l, -1, const_cstr!("writedir").as_ptr());

            // call writedir with 0 args and 1 expected result; this removes writedir from the stack
            ffi::lua_call(l, 0, 1);
            assert_eq!(ffi::lua_isstring(l, -1), 1);
            let p_writedir = ffi::lua_tostring(l, -1);

            let writedir = match CStr::from_ptr(p_writedir).to_str() {
                Ok(s) => Some(s.to_string()),
                Err(_) => {
                    error!("Failed to read lfs.writedir()");
                    None
                }
            };

            // pop: fn call result and lfs
            ffi::lua_pop(l, 2);

            writedir
        } else {
            // pop: lfs
            ffi::lua_pop(l, 1);

            None
        }
    };

    let config = if let Some(writedir) = writedir {
        let log_file = writedir + "Logs/dcsjsonrpc.log";

        let requests = FileAppender::builder()
            .append(false)
            .build(log_file)
            .unwrap();

        Config::builder()
            .appender(Appender::builder().build("file", Box::new(requests)))
            .logger(Logger::builder().build("dcsjsonrpc", LevelFilter::Info))
            .build(Root::builder().appender("file").build(LevelFilter::Off))
            .unwrap()
    } else {
        let stdout = ConsoleAppender::builder().build();
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .logger(Logger::builder().build("dcsjsonrpc", LevelFilter::Debug))
            .build(Root::builder().appender("stdout").build(LevelFilter::Off))
            .unwrap()
    };

    log4rs::init_config(config).unwrap();

    Ok(())
}

pub fn start(l: *mut ffi::lua_State) -> Result<(), Error> {
    if unsafe { SERVER.is_some() } {
        return Ok(());
    }

    init(l)?;

    info!("Starting ...");

    let server = Server::start()?;
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

pub unsafe fn try_next(l: *mut ffi::lua_State) -> Result<bool, Error> {
    // expect 1 argument, ignore other ones
    ffi::lua_settop(l, 1);

    // read callback argument
    if !ffi::lua_isfunction(l, -1) {
        ffi::lua_settop(l, 0);
        return Err(Error::ArgumentType(
            "callback".to_string(),
            "function".to_string(),
        ));
    }

    if let Some(server) = &SERVER {
        if let Some(mut next) = server.try_next() {
            let method = next.req.method();
            warn!("Pushing String: {}", method);
            push_string(l, method);
            match next.req.take_params() {
                Some(p) => {
                    let p = serde_json::to_string(&p).unwrap();
                    warn!("Pushing String: {}", p);
                    push_string(l, p);
                }
                None => ffi::lua_pushnil(l),
            }

            warn!("Done Pushing");

            ffi::lua_call(l, 2, 1); // 2 args, 1 result

            if !ffi::lua_istable(l, -1) {
                ffi::lua_settop(l, 0);
                return Err(Error::ArgumentType(
                    "result".to_string(),
                    "table".to_string(),
                ));
            }

            // check whether we've received an error
            ffi::lua_getfield(l, -1, const_cstr!("error").as_ptr());
            if ffi::lua_isstring(l, -1) == 1 {
                let error = CStr::from_ptr(ffi::lua_tostring(l, -1))
                    .to_str()?
                    .to_string();
                next.error(error);

                ffi::lua_settop(l, 0);
                return Ok(true);
            }

            // pop error
            lua_pop(l, 1);

            // check whether we've received a result
            ffi::lua_getfield(l, -1, const_cstr!("result").as_ptr());
            if ffi::lua_isstring(l, -1) == 1 {
                let res = CStr::from_ptr(ffi::lua_tostring(l, -1))
                    .to_str()?
                    .to_string();
                let res: Value = serde_json::from_str(&res)
                    .map_err(|err| Error::SerializeResult(err, res.to_string()))?;
                next.success(res);
            }

            ffi::lua_settop(l, 0);
            return Ok(true);
        }
    }

    ffi::lua_settop(l, 0);
    Ok(false)
}

pub fn broadcast(l: *mut ffi::lua_State) -> Result<(), Error> {
    let (payload, channel) = unsafe {
        // expect 2 arguments, ignore other ones
        ffi::lua_settop(l, 2);

        // read payload argument
        if ffi::lua_isstring(l, -1) == 0 {
            ffi::lua_settop(l, 0);
            return Err(Error::ArgumentType(
                "channel".to_string(),
                "string".to_string(),
            ));
        }
        let payload = CStr::from_ptr(ffi::lua_tostring(l, -1))
            .to_str()?
            .to_string();

        // read channel argument
        if ffi::lua_isstring(l, -2) == 0 {
            ffi::lua_settop(l, 0);
            return Err(Error::ArgumentType(
                "payload".to_string(),
                "string".to_string(),
            ));
        }
        let channel = CStr::from_ptr(ffi::lua_tostring(l, -2))
            .to_str()?
            .to_string();

        ffi::lua_settop(l, 0);
        (payload, channel)
    };

    let payload: Option<Value> =
        serde_json::from_str(&payload).map_err(|err| Error::SerializeBroadcast(err, payload))?;

    if let Some(server) = unsafe { &SERVER } {
        server.broadcast(&channel, payload);
    }

    Ok(())
}

//pub extern "C" fn cstr<T: Into<Vec<u8>>>(t: T) -> *const libc::c_char {
//    let s = CString::new(t).unwrap();
//    s.as_ptr()
//}

pub fn push_string<T: Into<Vec<u8>>>(l: *mut ffi::lua_State, t: T) {
    let cs = CString::new(t).unwrap();
    let ptr = cs.into_raw();

    unsafe {
        ffi::lua_pushstring(l, ptr);

        // retake pointer to free memory
        let _ = CString::from_raw(ptr);
    }
}
