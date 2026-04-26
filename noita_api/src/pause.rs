use crate::alloc::StdBox;
use crate::get_this_call;
use crate::types::death_match::DeathMatch;
use crate::types::game_global::GameGlobal;
use retour::static_detour;
pub static mut PAUSE_SIMULATE: bool = false;
#[cfg(target_os = "windows")]
static_detour! {
  static PAUSE: extern "thiscall" fn(StdBox<DeathMatch>, f32);
}
#[cfg(not(target_os = "windows"))]
static_detour! {
  static PAUSE: extern "C" fn(StdBox<DeathMatch>, f32);
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
        let old_pause = get_this_call!(0x006b26f0, fn(StdBox<DeathMatch>, f32));
        PAUSE.initialize(old_pause, pause).unwrap();
        PAUSE.enable().unwrap();
    }
}
