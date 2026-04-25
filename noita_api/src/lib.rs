pub mod alloc;
pub mod globals;
pub mod lua;
pub mod lua_bindings;
pub mod pause;
pub mod print;
pub mod types;
pub use libloading;
pub use noita_api_macros::{fast_call, lua_function, lua_module, std_call, this_call};
use std::mem;
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
pub fn new_game() {
    let fun = unsafe { mem::transmute::<usize, fast_call!(fn())>(0x009a2d70) };
    fun();
}
