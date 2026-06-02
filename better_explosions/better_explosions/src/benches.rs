use crate::ExplosionManager;
use noita_api::{Cell, Chunk, ConfigExplosion, GameGlobal, StdBox, Vec2};
use std::hint::black_box;
#[bench]
fn bench0_setup(_: &mut test::Bencher) {
    let mut game_global = GameGlobal::global();
    #[cfg(not(miri))]
    game_global
        .m_cell_factory
        .generate_cell_data(include_str!("../../materials.xml"))
        .unwrap();
    #[cfg(miri)]
    {
        for _ in 0..58 {
            game_global
                .m_cell_factory
                .cell_data
                .push(CellData::default());
        }
        let mut cell_data = CellData::default();
        cell_data.durability = 14;
        game_global.m_cell_factory.cell_data.push(cell_data);
    }
    let game_global = GameGlobal::global();
    let mut grid_world = game_global.m_grid_world;
    grid_world.chunk_map.insert(256, 256, Chunk::default());
    grid_world.chunk_map.insert(255, 256, Chunk::default());
    grid_world.chunk_map.insert(256, 255, Chunk::default());
    grid_world.chunk_map.insert(255, 255, Chunk::default());
    let mut config = ConfigExplosion::default();
    config.explosion_radius = 8.0;
    config.max_durability_to_destroy = 12;
    config.ray_energy = usize::MAX;
    let game_global = GameGlobal::global();
    config.create_cell_material = game_global.m_cell_factory.cell_data[1].name.clone();
    config.create_cell_probability = 0;
    config.hole_enabled = true;
    let c = black_box(config);
    let pos = black_box(Vec2 { x: 10.0, y: 10.0 });
    let mut em = ExplosionManager::default();
    em.explosion(&c, pos);
    em.explosion_lines(&c, pos);
    grid_world.chunk_map.clear();
}
#[cfg(test)]
fn empty_explosion(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&mut ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    let game_global = GameGlobal::global();
    let mut grid_world = game_global.m_grid_world;
    grid_world.chunk_map.insert(256, 256, Chunk::default());
    grid_world.chunk_map.insert(255, 256, Chunk::default());
    grid_world.chunk_map.insert(256, 255, Chunk::default());
    grid_world.chunk_map.insert(255, 255, Chunk::default());
    let mut config = ConfigExplosion::default();
    config.explosion_radius = r;
    config.max_durability_to_destroy = 12;
    config.ray_energy = usize::MAX;
    let game_global = GameGlobal::global();
    config.create_cell_material = game_global.m_cell_factory.cell_data[1].name.clone();
    config.create_cell_probability = 0;
    config.hole_enabled = true;
    let c = black_box(config);
    let pos = black_box(Vec2 { x: 10.0, y: 10.0 });
    let mut em = ExplosionManager::default();
    bencher.iter(|| f(&mut em, &c, pos));
    grid_world.chunk_map.clear();
}
#[cfg(test)]
fn half_explosion(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&mut ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    let game_global = GameGlobal::global();
    let mut grid_world = game_global.m_grid_world;
    grid_world.chunk_map.insert(256, 256, Chunk::default());
    grid_world.chunk_map.insert(255, 256, Chunk::default());
    grid_world.chunk_map.insert(256, 255, Chunk::default());
    grid_world.chunk_map.insert(255, 255, Chunk::default());
    let mut config = ConfigExplosion::default();
    config.explosion_radius = r;
    config.max_durability_to_destroy = 12;
    config.ray_energy = usize::MAX;
    let game_global = GameGlobal::global();
    config.create_cell_material = game_global.m_cell_factory.cell_data[1].name.clone();
    config.create_cell_probability = 50;
    config.hole_enabled = true;
    let c = black_box(config);
    let pos = black_box(Vec2 { x: 10.0, y: 10.0 });
    let mut em = ExplosionManager::default();
    bencher.iter(|| f(&mut em, &c, pos));
    grid_world.chunk_map.clear();
}
#[cfg(test)]
fn empty_explosion_wall(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&mut ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    let game_global = GameGlobal::global();
    let mut grid_world = game_global.m_grid_world;
    let mut chunk1 = Chunk::default();
    let mut chunk3 = Chunk::default();
    let cell_data = StdBox::from(&game_global.m_cell_factory.cell_data[58]);
    for y in 0..512 {
        for x in 100..108 {
            chunk1[y][x] = Some(Cell::new(cell_data));
            chunk3[y][x] = Some(Cell::new(cell_data));
        }
    }
    grid_world.chunk_map.insert(256, 256, Chunk::default());
    grid_world.chunk_map.insert(255, 256, chunk1);
    grid_world.chunk_map.insert(256, 255, Chunk::default());
    grid_world.chunk_map.insert(255, 255, chunk3);
    let mut config = ConfigExplosion::default();
    config.explosion_radius = r;
    config.max_durability_to_destroy = 12;
    config.ray_energy = usize::MAX;
    let game_global = GameGlobal::global();
    config.create_cell_material = game_global.m_cell_factory.cell_data[1].name.clone();
    config.create_cell_probability = 0;
    config.hole_enabled = true;
    let c = black_box(config);
    let pos = black_box(Vec2 { x: 10.0, y: 10.0 });
    let mut em = ExplosionManager::default();
    bencher.iter(|| f(&mut em, &c, pos));
    grid_world.chunk_map.clear();
}
#[cfg(test)]
fn half_explosion_wall(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&mut ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    let game_global = GameGlobal::global();
    let mut grid_world = game_global.m_grid_world;
    let mut chunk1 = Chunk::default();
    let mut chunk3 = Chunk::default();
    let cell_data = StdBox::from(&game_global.m_cell_factory.cell_data[58]);
    for y in 0..512 {
        for x in 100..108 {
            chunk1[y][x] = Some(Cell::new(cell_data));
            chunk3[y][x] = Some(Cell::new(cell_data));
        }
    }
    grid_world.chunk_map.insert(256, 256, Chunk::default());
    grid_world.chunk_map.insert(255, 256, chunk1);
    grid_world.chunk_map.insert(256, 255, Chunk::default());
    grid_world.chunk_map.insert(255, 255, chunk3);
    let mut config = ConfigExplosion::default();
    config.explosion_radius = r;
    config.max_durability_to_destroy = 12;
    config.ray_energy = usize::MAX;
    let game_global = GameGlobal::global();
    config.create_cell_material = game_global.m_cell_factory.cell_data[1].name.clone();
    config.create_cell_probability = 50;
    config.hole_enabled = true;
    let c = black_box(config);
    let pos = black_box(Vec2 { x: 10.0, y: 10.0 });
    let mut em = ExplosionManager::default();
    bencher.iter(|| f(&mut em, &c, pos));
    grid_world.chunk_map.clear();
}
#[cfg(not(miri))]
#[bench]
fn bench1_200_empty(bencher: &mut test::Bencher) {
    empty_explosion(200.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench2_200_half(bencher: &mut test::Bencher) {
    half_explosion(200.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench3_200_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(200.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench4_200_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(200.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench1_032_empty(bencher: &mut test::Bencher) {
    empty_explosion(32.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench2_032_half(bencher: &mut test::Bencher) {
    half_explosion(32.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench3_032_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(32.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench4_032_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(32.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench1_008_empty(bencher: &mut test::Bencher) {
    empty_explosion(8.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench2_008_half(bencher: &mut test::Bencher) {
    half_explosion(8.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench3_008_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(8.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench4_008_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(8.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench1_064_empty(bencher: &mut test::Bencher) {
    empty_explosion(64.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench2_064_half(bencher: &mut test::Bencher) {
    half_explosion(64.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench3_064_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(64.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench4_064_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(64.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench1_480_empty(bencher: &mut test::Bencher) {
    empty_explosion(480.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench2_480_half(bencher: &mut test::Bencher) {
    half_explosion(480.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench3_480_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(480.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench4_480_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(480.0, bencher, ExplosionManager::explosion);
}
#[cfg(not(miri))]
#[bench]
fn bench1_200_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(200.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench2_200_half_lines(bencher: &mut test::Bencher) {
    half_explosion(200.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench3_200_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(200.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench4_200_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(200.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench1_032_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(32.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench2_032_half_lines(bencher: &mut test::Bencher) {
    half_explosion(32.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench3_032_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(32.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench4_032_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(32.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench1_008_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(8.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench2_008_half_lines(bencher: &mut test::Bencher) {
    half_explosion(8.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench3_008_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(8.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench4_008_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(8.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench1_064_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(64.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench2_064_half_lines(bencher: &mut test::Bencher) {
    half_explosion(64.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench3_064_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(64.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench4_064_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(64.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench1_480_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(480.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench2_480_half_lines(bencher: &mut test::Bencher) {
    half_explosion(480.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench3_480_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(480.0, bencher, ExplosionManager::explosion_lines);
}
#[cfg(not(miri))]
#[bench]
fn bench4_480_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(480.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench5_exit(_: &mut test::Bencher) {
    GameGlobal::global().m_cell_factory.cell_data.free();
}
