#[noita_api::lua_module(true)]
mod lua {
    use std::sync::{LazyLock, Mutex};
    use std::time::Instant;
    static TIME: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now()));
    #[lua_function]
    fn update() {
        let mut time = TIME.lock().unwrap();
        noita_api::game_print!("update {}", time.elapsed().as_micros());
        *time = Instant::now();
    }
    #[lua_function]
    fn post_update() {
        let mut time = TIME.lock().unwrap();
        noita_api::game_print!("post_update {}", time.elapsed().as_micros());
        *time = Instant::now();
    }
    #[lua_function]
    fn init() {
        noita_api::print!("init");
    }
}
