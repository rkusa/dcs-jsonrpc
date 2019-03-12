#![feature(try_trait, await_macro, async_await, futures_api)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate tokio;
#[macro_use]
extern crate log;
#[macro_use]
extern crate const_cstr;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod error;
mod module;
mod server;

use std::ptr;

use crate::error::assert_stack_size;
use crate::module::push_string;
use libc::c_int;
use lua51 as ffi;

#[no_mangle]
pub extern "C" fn start(l: *mut ffi::lua_State) -> c_int {
    if let Err(err) = module::start(l).and_then(|_| assert_stack_size(l, 0)) {
        return report_error(l, &err.to_string());
    }

    0
}

#[no_mangle]
pub extern "C" fn stop(_state: *mut ffi::lua_State) -> c_int {
    module::stop();

    0
}

#[no_mangle]
pub extern "C" fn try_next(l: *mut ffi::lua_State) -> c_int {
    //    let mem = unsafe { ffi::lua_gc(l, ffi::LUA_GCCOUNT as i32, 0) };
    //    warn!("Mem in use: {} kB", mem);

    unsafe { ffi::lua_gc(l, ffi::LUA_GCSTOP as i32, 0) };

    let result =
        match unsafe { module::try_next(l).and_then(|r| assert_stack_size(l, 0).map(|_| r)) } {
            Ok(had_next) => {
                unsafe { ffi::lua_pushboolean(l, had_next as c_int) }
                1
            }
            Err(err) => {
                report_error(l, &err.to_string())
            }
        };

    unsafe { ffi::lua_gc(l, ffi::LUA_GCRESTART as i32, 0) };

    result
}

#[no_mangle]
pub extern "C" fn broadcast(l: *mut ffi::lua_State) -> c_int {
    unsafe { ffi::lua_gc(l, ffi::LUA_GCSTOP as i32, 0) };

    let result = if let Err(err) = module::broadcast(l).and_then(|_| assert_stack_size(l, 0)) {
        report_error(l, &err.to_string())
    } else {
        0
    };

    unsafe { ffi::lua_gc(l, ffi::LUA_GCRESTART as i32, 0) };

    result
}

fn report_error(l: *mut ffi::lua_State, msg: &str) -> c_int {
    error!("{}", msg);

    unsafe {
        push_string(l, msg);
        ffi::lua_error(l);
    }

    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn luaopen_dcsjsonrpc(state: *mut ffi::lua_State) -> c_int {
    let registration = &[
        ffi::luaL_Reg {
            name: const_cstr!("start").as_ptr(),
            func: Some(start),
        },
        ffi::luaL_Reg {
            name: const_cstr!("stop").as_ptr(),
            func: Some(stop),
        },
        ffi::luaL_Reg {
            name: const_cstr!("next").as_ptr(),
            func: Some(try_next),
        },
        ffi::luaL_Reg {
            name: const_cstr!("broadcast").as_ptr(),
            func: Some(broadcast),
        },
        ffi::luaL_Reg {
            name: ptr::null(),
            func: None,
        },
    ];

    ffi::luaL_openlib(
        state,
        const_cstr!("dcsjsonrpc").as_ptr(),
        registration.as_ptr(),
        0,
    );

    1
}
