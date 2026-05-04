use crate::{Entity, StdBox, Vec2, get_cdecl, search_fun};
use retour::static_detour;
#[cfg(target_os = "windows")]
static_detour! {
    pub static FIRE: extern "cdecl" fn(
        StdBox<Vec2>,
        *const Entity,
        isize,
        isize,
        u8,
        bool,
        f32,
        f32,
    );
}
#[cfg(not(target_os = "windows"))]
static_detour! {
    pub static FIRE: extern "C" fn(
        StdBox<Vec2>,
        *const Entity,
        isize,
        isize,
        u8,
        bool,
        f32,
        f32,
    );
}
#[allow(clippy::as_conversions)]
#[inline]
pub fn install_fire_wand_manual(
    fire: impl Fn(StdBox<Vec2>, *const Entity, isize, isize, u8, bool, f32, f32) + Send + 'static,
) {
    let fun_addr = search_fun![0x80, 0xbf, ???2, 0x00, 0x00, 0x00, 0x0f, 0x84, ???4, 0x69, 0x0d, ???4, 0xfd, 0x43, 0x03, 0x00];
    unsafe {
        let fun = get_cdecl!(
            fun_addr as usize,
            fn(StdBox<Vec2>, *const Entity, isize, isize, u8, bool, f32, f32)
        );
        FIRE.initialize(fun, fire).unwrap();
        FIRE.enable().unwrap();
    }
}
#[macro_export]
macro_rules! install_fire_wand {
    ($fun:path) => {
        #[allow(clippy::too_many_arguments)]
        fn on_fire_inner(
            position: StdBox<Vec2>,
            projectile: *const Entity,
            unk1: isize,
            unk2: isize,
            unk3: u8,
            send_message: bool,
            target_x: f32,
            target_y: f32,
        ) {
            let entity: *const Entity;
            let verlet_parent: *const Entity;
            unsafe {
                std::arch::asm!(
                    "",
                    out("ecx") entity,
                    out("edx") verlet_parent,
                );
            }
            #[allow(clippy::too_many_arguments)]
            fn inner_fun(
                entity: *const Entity,
                verlet_parent: *const Entity,
                position: StdBox<Vec2>,
                projectile: *const Entity,
                unk1: isize,
                unk2: isize,
                unk3: u8,
                send_message: bool,
                target_x: f32,
                target_y: f32,
            ) {
                unsafe {
                    std::arch::asm!(
                        "",
                        in("ecx") entity,
                        in("edx") verlet_parent,
                    );
                }
                $crate::FIRE.call(
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
            $fun(
                inner_fun,
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
        $crate::install_fire_wand_manual(on_fire_inner)
    };
}
