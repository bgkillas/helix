use crate::lua::LuaState;
use eyre::Context;
use noita_api_macros::this_call;
use std::ffi::c_void;
use std::mem;
pub fn print(value: &str) {
    let ptr = 0x01155538 as *mut c_void;
    let fun =
        unsafe { mem::transmute::<usize, this_call!(fn(*mut c_void, *const u8))>(0x00903930) };
    fun(ptr, value.as_ptr())
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::print(&format!("{}\0", format_args!($($arg)*)))
    };
}
pub fn game_print(value: &str) {
    let lua = LuaState::current().unwrap();
    lua.get_global(c"GamePrint");
    lua.push_string(value);
    lua.call(1, 0).wrap_err("Failed to call GamePrint").unwrap();
}
#[macro_export]
macro_rules! game_print {
    ($($arg:tt)*) => {
        $crate::print::game_print(&format!($($arg)*))
    };
}
