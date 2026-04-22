#[noita_api::lua_module(true)]
mod lua {
    use noita_api::constants::GAMEGLOBAL;
    #[lua_function]
    fn update() {
        noita_api::game_print!("{}", GAMEGLOBAL.frame_num);
    }
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn init() {}
}
