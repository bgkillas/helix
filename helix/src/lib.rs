#[noita_api::lua_module(true)]
mod lua {
    use noita_api::types::death_match::DeathMatch;
    #[lua_function]
    fn update() {}
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn world_init() {}
    #[lua_function]
    fn init() {}
    #[lua_function]
    fn world_seed_init() {}
    #[lua_function]
    fn text_msg(msg: &str) {
        if let Some(_host) = msg.strip_prefix("/connect ") {
        } else if msg == "/host" {
            let fun = unsafe {
                std::mem::transmute::<
                    usize,
                    noita_api::types::funs::FastCall<noita_api::alloc::StdPtr<DeathMatch>, ()>,
                >(0x006af160)
            };
            fun.call(DeathMatch::global().into());
        } else {
            noita_api::game_print!("{msg}");
        }
    }
}
