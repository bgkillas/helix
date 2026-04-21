#[noita_api::lua_module(true)]
mod lua {
    use std::cell::RefCell;
    use std::time::Instant;
    thread_local! {
        static TIME: RefCell<Instant> = Instant::now().into();
    }
    #[lua_function]
    fn update() {
        TIME.with(|time| {
            let mut time = time.borrow_mut();
            noita_api::game_print!("update {}", time.elapsed().as_micros());
            *time = Instant::now();
        });
    }
    #[lua_function]
    fn post_update() {
        TIME.with(|time| {
            let mut time = time.borrow_mut();
            noita_api::game_print!("post_update {}", time.elapsed().as_micros());
            *time = Instant::now();
        });
    }
    #[lua_function]
    fn init() {
        noita_api::game_print!("init");
    }
}
