use crate::{GameGlobal, LogFlush, StdBox, StdStringRef, get_this_call};
use std::ffi::{CStr, c_char, c_void};
use std::ptr;
#[inline]
pub fn log_print(value: &CStr) {
    let orig = LogFlush::global().flush;
    LogFlush::global().flush = true;
    let ptr = ptr::with_exposed_provenance_mut(0x0115_5538);
    let print = unsafe { get_this_call!(0x0090_3930, fn(*mut c_void, *const c_char)) };
    print(ptr, value.as_ptr());
    LogFlush::global().flush = orig;
}
#[macro_export]
macro_rules! log_print {
    ($($arg:tt)*) => {
        $crate::log_print(std::ffi::CStr::from_bytes_with_nul(format!("{}\0", format_args!($($arg)*)).as_bytes()).unwrap())
    };
}
#[macro_export]
macro_rules! log_println {
    ($($arg:tt)*) => {
        $crate::log_print!("{}\n", format_args!($($arg)*))
    };
}
#[inline]
pub fn game_print(value: &str) {
    let game_global = GameGlobal::global();
    if let Some(ptr) = game_global.game_print {
        let game_print =
            unsafe { get_this_call!(0x006c_4ad0, fn(StdBox<c_void>, &StdStringRef, usize)) };
        unsafe {
            let string = StdStringRef::no_alloc(value);
            game_print(ptr, &string, 1);
        }
    }
}
#[macro_export]
macro_rules! game_print {
    ($($arg:tt)*) => {
        $crate::print::game_print(&format!($($arg)*))
    };
}
