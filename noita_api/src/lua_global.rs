use crate::lua::LUA;
use crate::lua_bindings::{lua_Alloc, lua_State};
use retour::static_detour;
use std::os::raw::c_void;
static_detour! {
    static NEW_STATE: unsafe extern "C" fn(lua_Alloc, *mut c_void) -> *mut lua_State;
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
fn newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State {
    unsafe { NEW_STATE.call(f, ud) }
}
#[inline]
pub fn install_global() {
    unsafe {
        NEW_STATE.initialize(LUA.lua_newstate, newstate).unwrap();
        NEW_STATE.enable().unwrap();
    }
}
