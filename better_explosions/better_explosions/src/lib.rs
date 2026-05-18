#[noita_api::lua_module]
mod lua {
    use noita_api::{ConfigExplosion, ExplosionFun, StdBox, Vec2};
    #[explosion_hook]
    fn on_explosion(
        orig: ExplosionFun,
        config: StdBox<ConfigExplosion>,
        transform: StdBox<Vec2<f32>>,
        unk: isize,
    ) {
        orig(config, transform, unk);
    }
}
