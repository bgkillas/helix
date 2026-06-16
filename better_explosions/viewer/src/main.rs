#![feature(integer_casts)]
#![allow(clippy::shadow_reuse)]
#[cfg(not(test))]
mod ui;
#[cfg(not(test))]
fn main() -> eframe::Result {
    use better_explosions::explosion::ExplosionManager;
    use eframe::NativeOptions;
    use noita_api::{Chunk, ConfigExplosion, GameGlobal};
    use std::env::args;
    let mut game_global = GameGlobal::global();
    game_global
        .m_cell_factory
        .generate_cell_data(include_str!("../../materials.xml"))
        .unwrap();
    let mut grid_world = game_global.m_grid_world;
    let n = 2;
    for i in 256 - n..256 + n {
        for j in 256 - n..256 + n {
            grid_world.chunk_map.insert(i, j, Chunk::default());
        }
    }
    let mut args = args();
    if let Some(r) = args.nth(1).and_then(|s| s.parse::<u16>().ok())
        && let Some(n) = args.next().and_then(|s| s.parse::<u128>().ok())
    {
        let mut config = ConfigExplosion::default();
        config.explosion_radius = f32::from(r);
        config.max_durability_to_destroy = 12;
        config.ray_energy = usize::MAX;
        let game_global = GameGlobal::global();
        config.create_cell_material = game_global.m_cell_factory.cell_data[0].name.clone();
        config.create_cell_probability = 0;
        config.hole_enabled = true;
        let mut em = ExplosionManager::default();
        for _ in 0..16 {
            em.explosion_lines(&config, noita_api::Vec2 { x: 0.0, y: 0.0 });
        }
        let tmr = std::time::Instant::now();
        for _ in 0..n {
            em.explosion_lines(&config, noita_api::Vec2 { x: 0.0, y: 0.0 });
        }
        println!("{}", tmr.elapsed().as_nanos() / n);
        return Ok(());
    }
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
