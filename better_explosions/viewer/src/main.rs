#![allow(clippy::shadow_reuse)]
#[cfg(not(test))]
mod ui;
#[cfg(not(test))]
fn main() -> eframe::Result {
    use eframe::NativeOptions;
    use noita_api::{Chunk, GameGlobal};
    let mut game_global = GameGlobal::global();
    game_global
        .m_cell_factory
        .generate_cell_data(include_str!("../../materials.xml"))
        .unwrap();
    let mut grid_world = game_global.m_grid_world;
    grid_world.chunk_map.insert(256, 256, Chunk::default());
    grid_world.chunk_map.insert(255, 256, Chunk::default());
    grid_world.chunk_map.insert(256, 255, Chunk::default());
    grid_world.chunk_map.insert(255, 255, Chunk::default());
    eframe::run_native(
        "explosions",
        NativeOptions::default(),
        Box::new(|_| {
            let mut app = Box::<ui::App>::default();
            app.fill(29);
            Ok(app)
        }),
    )
}
