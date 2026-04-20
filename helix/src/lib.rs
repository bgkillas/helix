use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(AtomicUsize::default);
//noita_api::register_lua_functions!(init, update);
noita_api::register_lua_functions_dont_unload!(helix, init, update);
fn update() -> eyre::Result<()> {
    let val = COUNTER.load(Ordering::Relaxed);
    noita_api::game_print!("hi {val}");
    Ok(())
}
fn init() -> eyre::Result<()> {
    let val = COUNTER.load(Ordering::Relaxed);
    COUNTER.store(val + 1, Ordering::Relaxed);
    Ok(())
}
