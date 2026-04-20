use noita_api::register_function;
#[register_function]
fn update(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    noita_api::game_print!("update");
}
#[register_function]
fn init() {
    noita_api::game_print!("init");
}
#[register_function]
fn test() {}
//noita_api::register_lua_functions!(helix, init, update, test);
noita_api::register_lua_functions_dont_unload!(helix, init, update, test);
