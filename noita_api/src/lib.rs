#![feature(try_trait_v2)]
#![feature(debug_closure_helpers)]
#![cfg_attr(
    not(all(target_os = "windows", target_pointer_width = "32")),
    feature(allocator_api)
)]
#![feature(const_default)]
#![feature(const_trait_impl)]
pub mod alloc;
pub mod damage;
pub mod explosion;
pub mod fire_wand;
pub use nxml_rs;
mod funs;
mod globals;
pub mod lua;
pub mod lua_bindings;
pub mod lua_global;
pub mod new_game;
pub mod pause;
pub mod print;
pub(crate) mod search;
pub mod types;
pub use alloc::*;
pub use damage::*;
pub use explosion::*;
pub use fire_wand::*;
pub use libloading;
pub use lua::{LuaRawFn, LuaState};
pub use lua_global::*;
pub use new_game::*;
use noita_api_macros::search_fun;
pub use noita_api_macros::{
    damage_hook, exit_hook, explosion_hook, fire_hook, lua_function, lua_module, open_hook,
};
pub use pause::*;
pub use print::*;
pub use types::*;
#[inline]
pub fn dump_mem(s: &str) {
    unsafe {
        if let Ok(lib) = libloading::Library::new("malloc_probe.dll")
            && let Ok(func) =
                lib.get::<libloading::Symbol<unsafe extern "C" fn(*const u8, usize)>>("put_data")
        {
            func(s.as_ptr(), s.len());
        }
    }
}
#[inline]
#[must_use]
#[allow(clippy::as_conversions)]
pub fn get_construct_cell()
-> this_call!(fn(StdBox<GridWorld>, isize, isize, StdBox<CellData>, *mut ()) -> Option<Cell<()>>) {
    let ptr = search_fun![0x8b, 0x46, 0x38, 0x33, 0xc9, 0x83, 0xf8, 0x01];
    unsafe {
        get_this_call!(
            ptr as usize,
            fn(StdBox<GridWorld>, isize, isize, StdBox<CellData>, *mut ()) -> Option<Cell<()>>
        )
    }
}
