use crate::{Entity, StdBox, Vec2, fast_call, get_fast_call, search_fun};
static RAW: std::sync::atomic::AtomicPtr<retour::RawDetour> = std::sync::atomic::AtomicPtr::null();
pub type FireWandFun = fast_call!(
    fn(
        Option<StdBox<Entity>>,
        Option<StdBox<Entity>>,
        StdBox<Vec2>,
        Option<StdBox<Entity>>,
        isize,
        isize,
        u8,
        bool,
        f32,
        f32,
    )
);
#[allow(clippy::as_conversions)]
#[inline]
pub fn install_fire_wand_manual(fire_fun_hook: FireWandFun) {
    if !RAW.load(std::sync::atomic::Ordering::Relaxed).is_null() {
        return;
    }
    // 0xc0d290
    let fun_addr = search_fun![0x80, 0xbf, ???2, 0x00, 0x00, 0x00, 0x0f, 0x84, ???4, 0x69, 0x0d, ???4, 0xfd, 0x43, 0x03, 0x00];
    unsafe {
        let fun = get_fast_call!(
            fun_addr as usize,
            fn(
                Option<StdBox<Entity>>,
                Option<StdBox<Entity>>,
                StdBox<Vec2>,
                Option<StdBox<Entity>>,
                isize,
                isize,
                u8,
                bool,
                f32,
                f32,
            )
        );
        let raw = retour::RawDetour::new(fun as *const (), fire_fun_hook as *const ()).unwrap();
        raw.enable().unwrap();
        RAW.store(
            Box::leak(Box::new(raw)),
            std::sync::atomic::Ordering::Relaxed,
        );
    }
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
fn get_ptr() -> *const () {
    unsafe {
        RAW.load(std::sync::atomic::Ordering::Relaxed)
            .as_ref()
            .unwrap()
    }
    .trampoline() as *const ()
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
#[unsafe(naked)]
pub extern "fastcall" fn call_orig(
    _entity: Option<StdBox<Entity>>,
    _varlet_parent: Option<StdBox<Entity>>,
    _position: StdBox<Vec2>,
    _projectile: Option<StdBox<Entity>>,
    _unk1: isize,
    _unk2: isize,
    _unk3: u8,
    _send_message: bool,
    _target_x: f32,
    _target_y: f32,
) {
    std::arch::naked_asm!(
        "push ebp",
        "mov ebp,esp",
        "push [ebp+0x24]",
        "push [ebp+0x20]",
        "push [ebp+0x1c]",
        "push [ebp+0x18]",
        "push [ebp+0x14]",
        "push [ebp+0x10]",
        "push [ebp+0x0c]",
        "push [ebp+0x08]",
        "call {get_ptr}",
        "call eax",
        "mov esp,ebp",
        "pop ebp",
        "ret 0x20",
        get_ptr = sym get_ptr,
    )
}
#[macro_export]
macro_rules! install_fire_wand {
    ($fun:path) => {
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        #[allow(clippy::too_many_arguments)]
        extern "fastcall" fn on_fire_inner(
            entity: Option<$crate::StdBox<$crate::Entity>>,
            verlet_parent: Option<$crate::StdBox<$crate::Entity>>,
            position: $crate::StdBox<$crate::Vec2>,
            projectile: Option<$crate::StdBox<$crate::Entity>>,
            unk1: isize,
            unk2: isize,
            unk3: u8,
            send_message: bool,
            target_x: f32,
            target_y: f32,
        ) {
            $fun(
                $crate::call_orig,
                entity,
                verlet_parent,
                position,
                projectile,
                unk1,
                unk2,
                unk3,
                send_message,
                target_x,
                target_y,
            );
        }
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        #[unsafe(naked)]
        pub extern "fastcall" fn fire_fun_hook(
            _entity: Option<$crate::StdBox<$crate::Entity>>,
            _verlet_parent: Option<$crate::StdBox<$crate::Entity>>,
            _position: $crate::StdBox<$crate::Vec2>,
            _projectile: Option<$crate::StdBox<$crate::Entity>>,
            _unk1: isize,
            _unk2: isize,
            _unk3: u8,
            _send_message: bool,
            _target_x: f32,
            _target_y: f32,
        ) {
            std::arch::naked_asm!(
                "push ebp",
                "mov ebp,esp",
                "push [ebp+0x24]",
                "push [ebp+0x20]",
                "push [ebp+0x1c]",
                "push [ebp+0x18]",
                "push [ebp+0x14]",
                "push [ebp+0x10]",
                "push [ebp+0x0c]",
                "push [ebp+0x08]",
                "call {on_fire_inner}",
                "mov esp,ebp",
                "pop ebp",
                "ret",
                on_fire_inner = sym on_fire_inner,
            )
        }
        #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
        {
            _ = $fun;
        }
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        $crate::install_fire_wand_manual(fire_fun_hook)
    };
}
