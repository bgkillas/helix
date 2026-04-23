#[noita_api::lua_module(true)]
mod lua {
    use noita_api::globals::{GameGlobal, WorldSeed};
    #[lua_function]
    fn update() {
        let world_seed = *WorldSeed::default();
        let game_global = GameGlobal::default();
        noita_api::game_print!("{world_seed} {}", game_global.frame_num);
    }
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn init() {}
}
