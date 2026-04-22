pub mod alloc;
pub mod lua;
pub mod lua_bindings;
use crate::lua::LuaState;
use eyre::Context;
pub use libloading;
pub use noita_api_macros::{lua_function, lua_module};
pub fn print(value: &str) -> eyre::Result<()> {
    let lua = LuaState::current()?;
    lua.get_global(c"print");
    lua.push_string(value);
    lua.call(1, 0).wrap_err("Failed to call print")?;
    Ok(())
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = $crate::print(&format!($($arg)*));
    };
}
pub fn game_print(value: &str) -> eyre::Result<()> {
    let lua = LuaState::current()?;
    lua.get_global(c"GamePrint");
    lua.push_string(value);
    lua.call(1, 0).wrap_err("Failed to call GamePrint")?;
    Ok(())
}
#[macro_export]
macro_rules! game_print {
    ($($arg:tt)*) => {
        let _ = $crate::game_print(&format!($($arg)*));
    };
}
