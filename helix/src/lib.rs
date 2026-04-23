#[noita_api::lua_module(true)]
mod lua {
    use noita_api::constants::GameGlobal;
    #[lua_function]
    fn update() {
        let game_global = GameGlobal::default();
        noita_api::game_print!("{}", game_global.frame_num);
    }
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn init() {}
}
