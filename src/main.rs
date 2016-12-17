extern crate luajit_sys;

use std::path::Path;
use std::ffi::CString;

use luajit_sys::*;
use luajit_sys::consts::*;

fn main() {
    let mut state = unsafe { luaL_newstate() };

    unsafe {
        luaL_openlibs(state);
    }

    let file = CString::new("./examples/basic.lua").unwrap();

    let res = unsafe { luaL_loadfile(state, file.as_ptr()) };

    if res != 0 {
        panic!("Could not load file");
    }

    let res = unsafe { lua_pcall(state, 0, LUA_MULTRET, 0) };

    if res != 0 {
        panic!("Could not execute file");
    }

    unsafe {
        lua_close(state);
    }
}