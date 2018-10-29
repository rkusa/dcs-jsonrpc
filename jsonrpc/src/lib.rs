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

#[macro_use]
mod macros;
mod error;
mod module;
mod server;

use std::ffi::CString;
use std::ptr;

use hlua51::Lua;
use libc::c_int;
use lua51_sys as ffi;

#[no_mangle]
pub extern "C" fn start(state: *mut ffi::lua_State) -> c_int {
    let mut lua = unsafe { Lua::from_existing_state(state, false) };
    if let Err(err) = module::start(&mut lua) {
        return report_error(state, &err.to_string());
    }

    0
}

#[no_mangle]
pub extern "C" fn try_next(state: *mut ffi::lua_State) -> c_int {
    let mut lua = unsafe { Lua::from_existing_state(state, false) };
    match module::try_next(&mut lua) {
        Ok(had_next) => {
            unsafe { ffi::lua_pushboolean(state, had_next as c_int) }
            1
        }
        Err(err) => report_error(state, &err.to_string()),
    }
}

#[no_mangle]
pub extern "C" fn broadcast(state: *mut ffi::lua_State) -> c_int {
    let mut lua = unsafe { Lua::from_existing_state(state, false) };
    if let Err(err) = module::broadcast(&mut lua) {
        return report_error(state, &err.to_string());
    }

    0
}

fn report_error(state: *mut ffi::lua_State, msg: &str) -> c_int {
    let msg = CString::new(msg).unwrap();

    unsafe {
        ffi::lua_pushstring(state, msg.as_ptr());
        ffi::lua_error(state);
    }

    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn luaopen_dcsjsonrpc(state: *mut ffi::lua_State) -> c_int {
    let registration = &[
        ffi::luaL_Reg {
            name: cstr!("start"),
            func: Some(start),
        },
        ffi::luaL_Reg {
            name: cstr!("next"),
            func: Some(try_next),
        },
        ffi::luaL_Reg {
            name: cstr!("broadcast"),
            func: Some(broadcast),
        },
        ffi::luaL_Reg {
            name: ptr::null(),
            func: None,
        },
    ];

    ffi::luaL_openlib(state, cstr!("dcsjsonrpc"), registration.as_ptr(), 0);

    1
}
