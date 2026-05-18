use crate::{ConfigExplosion, StdBox, Transform, fast_call};
use noita_api_macros::fake_fast_call;
use retour::RawDetour;
use std::sync::OnceLock;
static RAW: OnceLock<RawDetour> = OnceLock::new();
fake_fast_call!(
    explosion,
    fn(StdBox<ConfigExplosion>, StdBox<Transform>, isize)
);
#[allow(clippy::as_conversions)]
#[inline]
pub fn install_explosion_manual(explosion_hook: ExplosionFun) {
    if RAW.get().is_some() {
        return;
    }
    unsafe {
        let raw = RawDetour::new(0x0065_7960 as *const (), explosion_hook as *const ()).unwrap();
        raw.enable().unwrap();
        RAW.set(raw).unwrap();
    }
}
