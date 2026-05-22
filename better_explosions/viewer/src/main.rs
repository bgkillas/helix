#![allow(clippy::shadow_reuse)]
#[cfg(not(test))]
mod ui;
#[cfg(not(test))]
fn main() -> eframe::Result {
    use eframe::NativeOptions;
    use noita_api::{Chunk, GameGlobal, StdBox};
    let mut game_global = GameGlobal::global();
    game_global
        .m_cell_factory
        .generate_cell_data(include_str!("../materials.xml"))
        .unwrap();
    let grid_world = game_global.m_grid_world;
    let mut chunk_map = grid_world.chunk_map.chunk_array;
    chunk_map[256][256] = Some(StdBox::new(Chunk::default()));
    chunk_map[255][256] = Some(StdBox::new(Chunk::default()));
    chunk_map[256][255] = Some(StdBox::new(Chunk::default()));
    chunk_map[255][255] = Some(StdBox::new(Chunk::default()));
    ui::fill(29);
    eframe::run_native(
        "explosions",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::<ui::App>::default())),
    )
}
