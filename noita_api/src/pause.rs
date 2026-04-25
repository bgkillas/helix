use crate::alloc::StdBox;
use crate::types::death_match::DeathMatch;
use crate::types::game_global::GameGlobal;
use retour::static_detour;
use std::mem;
pub static mut PAUSE_SIMULATE: bool = false;
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
    if unsafe { !PAUSE_SIMULATE } {
        let mut game_global = GameGlobal::global();
        if game_global.is_paused() {
            let state = *game_global.pause_state;
            game_global.unpause();
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
