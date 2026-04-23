#[noita_api::lua_module(true)]
mod lua {
    use noita_api::types::game_global::GameGlobal;
    use noita_api::types::world_seed::WorldSeed;
    #[lua_function]
    fn update() {
        let world_seed = WorldSeed::global();
        let game_global = GameGlobal::global();
        noita_api::game_print!("{} {}", world_seed.seed, game_global.frame_num);
    }
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn init() {}
}
