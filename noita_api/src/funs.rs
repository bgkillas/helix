#[macro_export]
macro_rules! get_this_call {
    ($addr:expr, $($tt:tt)*) => {
        std::mem::transmute::<usize, $crate::this_call!($($tt)*)>($addr)
    };
}
#[macro_export]
macro_rules! get_fast_call {
    ($addr:expr, $($tt:tt)*) => {
        std::mem::transmute::<usize, $crate::fast_call!($($tt)*)>($addr)
    };
}
#[macro_export]
macro_rules! get_cdecl {
    ($addr:expr, $($tt:tt)*) => {
        std::mem::transmute::<usize, $crate::cdecl!($($tt)*)>($addr)
    };
}
#[macro_export]
macro_rules! get_std_call {
    ($addr:expr, $($tt:tt)*) => {
        std::mem::transmute::<usize, $crate::std_call!($($tt)*)>($addr)
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
#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! cdecl {
    ($($tt:tt)*) => {extern "cdecl" $($tt)*};
}
#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! cdecl {
    ($($tt:tt)*) => {extern "C" $($tt)*};
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
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! static_detour_this_call {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "thiscall" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "thiscall" $($tt)*
        }
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! static_detour_this_call {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "C" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "C" $($tt)*
        }
    };
}
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! static_detour_fast_call {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "fastcall" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "fastcall" $($tt)*
        }
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! static_detour_fast_call {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "C" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "C" $($tt)*
        }
    };
}
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! static_detour_std_call {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "stdcall" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "stdcall" $($tt)*
        }
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! static_detour_std_call {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "C" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "C" $($tt)*
        }
    };
}
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! static_detour_cdecl {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "cdecl" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "cdecl" $($tt)*
        }
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! static_detour_cdecl {
    (static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            static $ident: extern "C" $($tt)*
        }
    };
    (pub static $ident:ident: $($tt:tt)*) => {
        retour::static_detour! {
            pub static $ident: extern "C" $($tt)*
        }
    };
}
