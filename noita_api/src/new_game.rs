use crate::{GameGlobal, PAUSE_SIMULATE, get_fast_call};
use std::sync::atomic::{AtomicU8, Ordering};
const PAUSE_FRAME: u8 = 8;
static DO_RESTART: AtomicU8 = AtomicU8::new(0);
#[inline]
unsafe fn new_game() {
    let fun = unsafe { get_fast_call!(0x009a_2d70, fn()) };
    fun();
}
#[inline]
pub fn delay_new_game() {
    DO_RESTART.store(PAUSE_FRAME, Ordering::Relaxed);
    PAUSE_SIMULATE.store(false, Ordering::Relaxed);
    GameGlobal::global().pause();
}
#[inline]
pub fn new_game_pause_update() {
    match DO_RESTART.load(Ordering::Relaxed) {
        0 => {}
        1 => {
            let mut game_global = GameGlobal::global();
            if game_global.is_paused() {
                DO_RESTART.store(0, Ordering::Relaxed);
                unsafe {
                    new_game();
                }
            } else {
                DO_RESTART.store(PAUSE_FRAME, Ordering::Relaxed);
                game_global.pause();
            }
        }
        s => DO_RESTART.store(s - 1, Ordering::Relaxed),
    }
}
