use crate::lua_bindings::{lua_State, luaL_newstate};
use retour::static_detour;
static_detour! {
    pub static NEW_STATE: unsafe extern "C" fn() -> *mut lua_State;
}
#[inline]
pub fn install_global(f: impl Fn() -> *mut lua_State + Send + 'static) {
    unsafe {
        NEW_STATE.initialize(luaL_newstate, f).unwrap();
        NEW_STATE.enable().unwrap();
    }
}
