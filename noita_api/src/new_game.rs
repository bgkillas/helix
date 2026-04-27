use crate::{GameGlobal, PAUSE_SIMULATE, get_fast_call};
const PAUSE_FRAME: u8 = 8;
static mut DO_RESTART: u8 = 0;
pub fn new_game() {
    let fun = unsafe { get_fast_call!(0x009a2d70, fn()) };
    fun();
}
pub fn delay_new_game() {
    unsafe {
        DO_RESTART = PAUSE_FRAME;
        PAUSE_SIMULATE = true;
    }
    GameGlobal::global().pause();
}
pub fn new_game_pause_update() {
    if unsafe { DO_RESTART == 1 } {
        let mut game_global = GameGlobal::global();
        if game_global.is_paused() {
            unsafe {
                DO_RESTART = 0;
            }
            new_game();
        } else {
            unsafe {
                DO_RESTART = PAUSE_FRAME;
            }
            game_global.pause();
        }
    } else if unsafe { DO_RESTART > 1 } {
        unsafe { DO_RESTART -= 1 }
    }
}
