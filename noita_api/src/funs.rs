#[macro_export]
macro_rules! get_this_call {
    ($addr:expr, $($tt:tt)*) => {
        mem::transmute::<usize, $crate::this_call!($($tt)*)>($addr)
    };
}
#[macro_export]
macro_rules! get_fast_call {
    ($addr:expr, $($tt:tt)*) => {
        mem::transmute::<usize, $crate::fast_call!($($tt)*)>($addr)
    };
}
#[macro_export]
macro_rules! get_std_call {
    ($addr:expr, $($tt:tt)*) => {
        mem::transmute::<usize, $crate::std_call!($($tt)*)>($addr)
    };
}
#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! this_call {
    ($($tt:tt)*) => {extern "thiscall" $($tt)*};
}
#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! std_call {
    ($($tt:tt)*) => {extern "stdcall" $($tt)*};
}
#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! fast_call {
    ($($tt:tt)*) => {extern "fastcall" $($tt)*};
}
#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! this_call {
    ($($tt:tt)*) => {extern "C" $($tt)*};
}
#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! std_call {
    ($($tt:tt)*) => {extern "C" $($tt)*};
}
#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! fast_call {
    ($($tt:tt)*) => {extern "C" $($tt)*};
}
