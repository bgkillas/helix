#![cfg_attr(
    not(all(target_os = "windows", target_pointer_width = "32")),
    feature(allocator_api)
)]
pub mod alloc;
pub mod funs;
pub mod globals;
pub mod lua;
pub mod lua_bindings;
pub mod pause;
pub mod print;
pub mod types;
pub use libloading;
pub use noita_api_macros::{lua_function, lua_module};
use std::mem;
pub fn dump_mem(s: &str) {
    let malloc_probe = format!(
        "{}/malloc_probe.dll",
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
    );
    unsafe {
        if let Ok(lib) = libloading::Library::new(malloc_probe)
            && let Ok(func) =
                lib.get::<libloading::Symbol<unsafe extern "C" fn(*const u8, usize)>>("put_data")
        {
            func(s.as_ptr(), s.len());
        }
    }
}
pub fn new_game() {
    let fun = unsafe { get_this_call!(0x009a2d70, fn()) };
    fun();
}
