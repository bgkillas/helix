use crate::lua::LuaState;
use eyre::Context;
pub fn print(value: &str) {
    let lua = LuaState::current().unwrap();
    lua.get_global(c"print");
    lua.push_string(value);
    lua.call(1, 0).wrap_err("Failed to call print").unwrap();
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::print(&format!($($arg)*))
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
