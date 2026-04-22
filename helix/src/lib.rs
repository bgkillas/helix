#[noita_api::lua_module(true)]
mod lua {
    use noita_api::alloc::{box_new, free, malloc};
    use std::sync::{LazyLock, Mutex};
    use std::time::Instant;
    static TIME: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now()));
    #[lua_function]
    fn update() {
        let mut time = TIME.lock().unwrap();
        noita_api::game_print!("update {}", time.elapsed().as_micros());
        *time = Instant::now();
        let tmr = Instant::now();
        let ptr = malloc::<usize>();
        std::hint::black_box(ptr);
        noita_api::game_print!("malloc {}", tmr.elapsed().as_nanos());
        let tmr = Instant::now();
        let ptrbox = box_new(10);
        let ptrbox = std::hint::black_box(ptrbox);
        noita_api::game_print!("box {}", tmr.elapsed().as_nanos());
        let tmr = Instant::now();
        free(ptr);
        ptrbox.free();
        noita_api::game_print!("free {}", tmr.elapsed().as_nanos());
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
