use crate::lua::LUA;
use crate::lua_bindings::{LUA_GLOBALSINDEX, lua_State};
use retour::static_detour;
use std::mem;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

static_detour! {
    static NEW_STATE: unsafe extern "C" fn() -> *mut lua_State;
}
static LUA_FUN: AtomicUsize = AtomicUsize::new(0);
static LUA_NAME: AtomicPtr<u8> = AtomicPtr::null();
#[allow(clippy::not_unsafe_ptr_arg_deref)]
fn newstate() -> *mut lua_State {
    let lua = unsafe { NEW_STATE.call() };
    let fun_addr = LUA_FUN.load(Ordering::Relaxed);
    let fun = unsafe { mem::transmute::<usize, fn(*mut lua_State)>(fun_addr) };
    fun(lua);
    unsafe {
        (LUA.lua_setfield)(
            lua,
            LUA_GLOBALSINDEX,
            LUA_NAME.load(Ordering::Relaxed).cast_const().cast(),
        );
    }
    lua
}
#[allow(clippy::as_conversions)]
#[inline]
pub fn install_global(f: fn(*mut lua_State), name: &'static str) {
    unsafe {
        LUA_FUN.store(f as usize, Ordering::Relaxed);
        LUA_NAME.store(name.as_ptr().cast_mut(), Ordering::Relaxed);
        NEW_STATE.initialize(LUA.luaL_newstate, newstate).unwrap();
        NEW_STATE.enable().unwrap();
    }
}
