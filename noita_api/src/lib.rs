pub mod alloc;
pub mod globals;
pub mod lua;
pub mod lua_bindings;
pub mod types;
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
pub fn dump_mem(s: &str) {
    unsafe {
        let lib = libloading::Library::new(format!(
            "{}/malloc_probe.dll",
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        ))
        .unwrap();
        let func: libloading::Symbol<unsafe extern "C" fn(*const u8, usize)> =
            lib.get("put_data").unwrap();
        func(s.as_ptr(), s.len());
    }
}
