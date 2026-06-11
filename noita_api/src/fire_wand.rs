use crate::{Entity, StdBox, Vec2, fast_call};
use noita_api_macros::{fake_fast_call, search_fun};
use retour::{Function as _, RawDetour};
use std::sync::OnceLock;
static RAW: OnceLock<RawDetour> = OnceLock::new();
fake_fast_call!(
    fire_wand,
    fn(
        Option<Entity>,
        Option<Entity>,
        StdBox<Vec2<f32>>,
        Option<Entity>,
        isize,
        isize,
        u8,
        bool,
        f32,
        f32,
    )
);
#[inline]
pub fn install_fire_wand_manual(fire_fun_hook: FireWandFun) {
    if RAW.get().is_some() {
        return;
    }
    // 0x00c0_d290
    let fun_addr = search_fun![0x80, 0xbf, ???2, 0x00, 0x00, 0x00, 0x0f, 0x84, ???4, 0x69, 0x0d, ???4, 0xfd, 0x43, 0x03, 0x00];
    unsafe {
        let raw = RawDetour::new(fun_addr.cast(), fire_fun_hook.to_ptr()).unwrap();
        raw.enable().unwrap();
        RAW.set(raw).unwrap();
    }
}
