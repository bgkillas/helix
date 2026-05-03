#![feature(try_trait_v2)]
#![cfg_attr(
    not(all(target_os = "windows", target_pointer_width = "32")),
    feature(allocator_api)
)]
pub mod alloc;
mod funs;
mod globals;
pub mod lua;
pub mod lua_bindings;
pub mod lua_global;
pub mod new_game;
pub mod pause;
pub mod print;
pub mod types;
pub use alloc::*;
pub use libloading;
pub use lua_global::*;
pub use new_game::*;
pub(crate) use noita_api_macros::{assert_size, assert_size_with};
pub use noita_api_macros::{lua_function, lua_module};
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
