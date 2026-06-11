use crate::{ConfigExplosion, StdBox, Vec2, fast_call};
use noita_api_macros::fake_fast_call;
use retour::{Function as _, RawDetour};
use std::ptr;
use std::sync::OnceLock;
static RAW: OnceLock<RawDetour> = OnceLock::new();
fake_fast_call!(
    explosion,
    fn(StdBox<ConfigExplosion>, StdBox<Vec2<f32>>, isize)
);
#[inline]
pub fn install_explosion_manual(explosion_hook: ExplosionFun) {
    if RAW.get().is_some() {
        return;
    }
    unsafe {
        let raw = RawDetour::new(
            ptr::with_exposed_provenance(0x0065_7960),
            explosion_hook.to_ptr(),
        )
        .unwrap();
        raw.enable().unwrap();
        RAW.set(raw).unwrap();
    }
}
