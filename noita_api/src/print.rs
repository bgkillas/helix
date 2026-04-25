use crate::alloc::StdBox;
use crate::types::game_global::GameGlobal;
use crate::types::string::StdString;
use noita_api_macros::this_call;
use std::ffi::c_void;
use std::mem;
pub fn print(value: &str) {
    let ptr = 0x01155538 as *mut c_void;
    let print =
        unsafe { mem::transmute::<usize, this_call!(fn(*mut c_void, *const u8))>(0x00903930) };
    print(ptr, value.as_ptr())
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::print(&format!("{}\0", format_args!($($arg)*)))
    };
}
pub fn game_print(value: &str) {
    let game_global = GameGlobal::global();
    if let Some(ptr) = game_global.game_print {
        let game_print = unsafe {
            mem::transmute::<usize, this_call!(fn(StdBox<c_void>, &StdString, usize))>(0x006c4ad0)
        };
        let string = StdString::from(value);
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
