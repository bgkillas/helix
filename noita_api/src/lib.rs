pub mod alloc;
pub mod globals;
pub mod lua;
pub mod lua_bindings;
pub mod print;
pub mod types;
use crate::alloc::StdBox;
use crate::types::death_match::DeathMatch;
use crate::types::funs::FastCall;
use crate::types::game_global::GameGlobal;
pub use libloading;
pub use noita_api_macros::{lua_function, lua_module};
use retour::static_detour;
use std::mem;
pub static mut PAUSE_SIMULATE: bool = true;
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
    let fun = unsafe { mem::transmute::<usize, FastCall<(), ()>>(0x009a2d70) };
    fun.call(());
}
#[cfg(target_os = "windows")]
static_detour! {
  static PAUSE: extern "thiscall" fn(StdBox<DeathMatch>, f32);
}
#[cfg(not(target_os = "windows"))]
static_detour! {
  static PAUSE: fn(StdBox<DeathMatch>, f32);
}
fn pause(this: StdBox<DeathMatch>, dt: f32) {
    PAUSE.call(this, dt);
    if unsafe { PAUSE_SIMULATE } {
        let mut game_global = GameGlobal::global();
        let state = *game_global.pause_state;
        if state > 0 {
            *game_global.pause_state = 0;
            PAUSE.call(this, dt);
            *game_global.pause_state = state;
        }
    }
}
pub fn disable_pause() {
    unsafe {
        #[allow(clippy::missing_transmute_annotations)]
        let old_pause = mem::transmute::<usize, _>(0x006b26f0);
        PAUSE.initialize(old_pause, pause).unwrap();
        PAUSE.enable().unwrap();
    }
}
