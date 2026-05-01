use crate::{GameGlobal, LogFlush, StdBox, StdString, get_this_call};
use std::ffi::c_void;
use std::ptr;
#[inline]
pub fn log_print(value: &str) {
    let orig = LogFlush::global().flush;
    LogFlush::global().flush = true;
    let ptr = ptr::with_exposed_provenance_mut(0x0115_5538);
    let print = unsafe { get_this_call!(0x0090_3930, fn(*mut c_void, *const u8)) };
    print(ptr, value.as_ptr());
    LogFlush::global().flush = orig;
}
#[macro_export]
macro_rules! log_print {
    ($($arg:tt)*) => {
        $crate::log_print(&format!("{}\0", format_args!($($arg)*)))
    };
}
#[macro_export]
macro_rules! log_println {
    ($($arg:tt)*) => {
        $crate::log_print(&format!("{}\n\0", format_args!($($arg)*)))
    };
}
#[inline]
pub fn game_print(value: &str) {
    let game_global = GameGlobal::global();
    if let Some(ptr) = game_global.game_print {
        let game_print =
            unsafe { get_this_call!(0x006c_4ad0, fn(StdBox<c_void>, &StdString, usize)) };
        let string = StdString::no_alloc(value);
        game_print(ptr, &string, 1);
        string.free();
    }
}
#[macro_export]
macro_rules! game_print {
    ($($arg:tt)*) => {
        $crate::print::game_print(&format!($($arg)*))
    };
}
