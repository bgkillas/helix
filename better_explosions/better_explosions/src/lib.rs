#![feature(sync_unsafe_cell)]
#![feature(test)]
extern crate test;
pub mod arc;
pub mod circle;
pub mod circumference;
pub mod line;
pub mod octant;
use crate::arc::ArcIter;
use crate::circumference::Circumference;
use crate::line::LineIter;
use crate::octant::octant;
use noita_api::{Cell, CellData, ConfigExplosion, GameGlobal, GridWorld, StdBox, Vec2, this_call};
use rand::RngExt as _;
use rand::distr::Bernoulli;
use std::f32::consts::TAU;
use std::num::NonZeroUsize;
#[noita_api::lua_module]
mod lua {
    use crate::ExplosionManager;
    use noita_api::{ConfigExplosion, ExplosionFun, StdBox, Vec2};
    impl ExplosionManager {
        #[explosion_hook]
        fn on_explosion(
            &mut self,
            _: ExplosionFun,
            config: StdBox<ConfigExplosion>,
            pos: StdBox<Vec2<f32>>,
            _: isize,
        ) {
            self.explosion(&config, *pos);
        }
    }
}
#[allow(dead_code)]
pub struct ExplosionManager {
    construct_cell: this_call!(
        fn(StdBox<GridWorld>, isize, isize, StdBox<CellData>, *mut ()) -> Option<Cell<()>>
    ),
}
#[allow(clippy::unnecessary_wraps)]
#[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
extern "C" fn dummy(
    _: StdBox<GridWorld>,
    _: isize,
    _: isize,
    cell_data: StdBox<CellData>,
    _: *mut (),
) -> Option<Cell<()>> {
    Some(Cell::new(cell_data))
}
impl Default for ExplosionManager {
    #[inline]
    fn default() -> Self {
        Self {
            #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
            construct_cell: dummy,
            #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
            construct_cell: noita_api::get_construct_cell(),
        }
    }
}
impl ExplosionManager {
    #[inline]
    pub fn explosion(&self, config: &ConfigExplosion, pos: Vec2<f32>) {
        let r = truncate_f32u(config.explosion_radius);
        let n = (8 * r.div_ceil(16)).max((((r * 63) / 10) / 16) & !7);
        let rays: u16 = u16::try_from(n).unwrap();
        self.explosion_with_rays(config, pos, rays);
    }
    #[inline]
    pub fn explosion_with_rays(&self, config: &ConfigExplosion, pos: Vec2<f32>, rays: u16) {
        let mut rng = rand::rng();
        let game_global = GameGlobal::global();
        let cell_create_id = game_global
            .m_cell_factory
            .material_ids
            .get(&config.create_cell_material)
            .copied()
            .unwrap_or_default();
        let cell_create = StdBox::from(&game_global.m_cell_factory.cell_data[cell_create_id]);
        let grid_world = game_global.m_grid_world;
        let chunk_map = grid_world.chunk_map.chunk_array;
        let ix0 = (512 * 256 + truncate_f32(pos.x)).cast_unsigned();
        let iy0 = (512 * 256 + truncate_f32(pos.y)).cast_unsigned();
        let delta_theta = TAU / f32::from(rays);
        let bern = if cell_create_id == 0 {
            Bernoulli::from_ratio(0, 1).unwrap()
        } else {
            Bernoulli::from_ratio(u32::try_from(config.create_cell_probability).unwrap(), 100)
                .unwrap()
        };
        let mut radii = Vec::with_capacity(usize::from(rays));
        for ray in 0..rays {
            let theta = (f32::from(ray) + 0.5) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix1 =
                (ix0.cast_signed() + round_f32(cos * config.explosion_radius)).cast_unsigned();
            let iy1 =
                (iy0.cast_signed() + round_f32(sin * config.explosion_radius)).cast_unsigned();
            let mut energy = config.ray_energy;
            let (mut ix2, mut iy2) = (ix1, iy1);
            for (x, y) in LineIter::new(ix0, iy0, ix1, iy1) {
                let px = x % 512;
                let py = y % 512;
                if let Some(c) = chunk_map[y / 512][x / 512] {
                    if let Some(p) = c[py][px] {
                        if energy > p.hp
                            && p.material.durability <= config.max_durability_to_destroy
                            && config.hole_enabled
                        {
                            energy -= p.hp;
                        } else {
                            (ix2, iy2) = (x, y);
                            break;
                        }
                    }
                } else {
                    (ix2, iy2) = (x, y);
                    break;
                }
            }
            let r = if (ix2, iy2) == (ix1, iy1) {
                truncate_f32u(config.explosion_radius)
            } else {
                truncate_f32u(
                    (truncate_usize(ix2.abs_diff(ix0)).hypot(truncate_usize(iy2.abs_diff(iy0)))
                        * 1.2)
                        .min(config.explosion_radius),
                )
            };
            radii.push(r);
        }
        for (rayu, r) in radii.into_iter().enumerate() {
            let ray = u16::try_from(rayu).unwrap();
            let rf = truncate_usize(r);
            let theta = f32::from(ray) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix3 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy3 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
            let theta = f32::from(ray + 1) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix4 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy4 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
            for (x, y) in ArcIter::new(ix0, iy0, ix3, iy3, ix4, iy4, r * r) {
                let px = x % 512;
                let py = y % 512;
                if let Some(mut c) = chunk_map[y / 512][x / 512] {
                    if let Some(p) = c[py][px] {
                        if p.material.durability > config.max_durability_to_destroy {
                            continue;
                        }
                        p.ptr.free();
                    }
                    if rng.sample(bern) {
                        c[py][px] = (self.construct_cell)(
                            grid_world,
                            x.cast_signed() - 512 * 256,
                            y.cast_signed() - 512 * 256,
                            cell_create,
                            std::ptr::null_mut(),
                        );
                    } else {
                        c[py][px] = None;
                    }
                }
            }
        }
    }
    #[inline]
    pub fn explosion_lines(&self, config: &ConfigExplosion, pos: Vec2<f32>) {
        let mut rng = rand::rng();
        let game_global = GameGlobal::global();
        let cell_create_id = game_global
            .m_cell_factory
            .material_ids
            .get(&config.create_cell_material)
            .copied()
            .unwrap_or_default();
        let cell_create = StdBox::from(&game_global.m_cell_factory.cell_data[cell_create_id]);
        let grid_world = game_global.m_grid_world;
        let chunk_map = grid_world.chunk_map.chunk_array;
        let ix0 = (512 * 256 + truncate_f32(pos.x)).cast_unsigned();
        let iy0 = (512 * 256 + truncate_f32(pos.y)).cast_unsigned();
        let chance = if cell_create_id == 0 {
            0
        } else {
            u32::try_from(config.create_cell_probability).unwrap()
        };
        let mut hp_vec: Vec<Option<NonZeroUsize>> =
            vec![None; 512 * 512 * grid_world.chunk_map.chunk_count];
        let mut hp_map: Box<[[usize; 512]; 512]> = unsafe { Box::new_uninit().assume_init() };
        for (i, (x, y, _)) in grid_world.chunk_map.flat_iter().enumerate() {
            hp_map[usize::from(y)][usize::from(x)] = i;
        }
        let bern = Bernoulli::from_ratio(chance, 100).unwrap();
        let r = truncate_f32u(config.explosion_radius);
        for (ix1, iy1) in Circumference::new(r) {
            octant(ix0, iy0, ix1, iy1, |_, ix2, iy2| {
                let mut energy = config.ray_energy;
                'a: for (xi, y) in LineIter::new(ix0, iy0, ix2, iy2) {
                    for x in xi..xi + 2 {
                        let px = x % 512;
                        let py = y % 512;
                        let cx = x / 512;
                        let cy = y / 512;
                        if let Some(mut c) = chunk_map[cy][cx] {
                            let hp_index = 512 * 512 * hp_map[cy][cx] + 512 * py + px;
                            if let Some(hp) = hp_vec[hp_index] {
                                if hp.get() == usize::MAX {
                                    continue 'a;
                                }
                                if let Some(new) = energy.checked_sub(hp.get()) {
                                    energy = new;
                                } else {
                                    break 'a;
                                }
                            } else {
                                if let Some(p) = c[py][px] {
                                    if p.material.durability <= config.max_durability_to_destroy
                                        && config.hole_enabled
                                        && let Some(new) = energy.checked_sub(p.hp)
                                    {
                                        hp_vec[hp_index] = Some(
                                            NonZeroUsize::new(p.hp).unwrap_or(NonZeroUsize::MAX),
                                        );
                                        energy = new;
                                        p.ptr.free();
                                    } else {
                                        break 'a;
                                    }
                                } else {
                                    hp_vec[hp_index] = Some(NonZeroUsize::MAX);
                                }
                                if rng.sample(bern) {
                                    c[py][px] = (self.construct_cell)(
                                        grid_world,
                                        x.cast_signed() - 512 * 256,
                                        y.cast_signed() - 512 * 256,
                                        cell_create,
                                        std::ptr::null_mut(),
                                    );
                                } else {
                                    c[py][px] = None;
                                }
                            }
                        } else {
                            break 'a;
                        }
                    }
                }
            });
        }
    }
}
#[bench]
fn bench0_setup(_: &mut test::Bencher) {
    let mut game_global = GameGlobal::global();
    game_global
        .m_cell_factory
        .generate_cell_data(include_str!("../../materials.xml"))
        .unwrap();
}
#[cfg(test)]
fn empty_explosion(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    use noita_api::{Chunk, GameGlobal};
    use std::hint::black_box;
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
    let em = ExplosionManager {
        construct_cell: dummy,
    };
    bencher.iter(|| f(&em, &c, pos));
}
#[cfg(test)]
fn half_explosion(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    use noita_api::GameGlobal;
    use std::hint::black_box;
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
    let em = ExplosionManager {
        construct_cell: dummy,
    };
    bencher.iter(|| f(&em, &c, pos));
}
#[cfg(test)]
fn empty_explosion_wall(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    use noita_api::{Chunk, GameGlobal};
    use std::hint::black_box;
    let game_global = GameGlobal::global();
    let mut grid_world = game_global.m_grid_world;
    let mut chunk1 = Chunk::default();
    let mut chunk3 = Chunk::default();
    let cell_data = StdBox::from(&game_global.m_cell_factory.cell_data[58]);
    for y in 0..512 {
        for x in 100..512 {
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
    let em = ExplosionManager {
        construct_cell: dummy,
    };
    bencher.iter(|| f(&em, &c, pos));
}
#[cfg(test)]
fn half_explosion_wall(
    r: f32,
    bencher: &mut test::Bencher,
    f: fn(&ExplosionManager, &ConfigExplosion, Vec2<f32>),
) {
    use noita_api::GameGlobal;
    use std::hint::black_box;
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
    let em = ExplosionManager {
        construct_cell: dummy,
    };
    bencher.iter(|| f(&em, &c, pos));
}
#[bench]
fn bench1_200_empty(bencher: &mut test::Bencher) {
    empty_explosion(200.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench2_200_half(bencher: &mut test::Bencher) {
    half_explosion(200.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench3_200_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(200.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench4_200_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(200.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench1_032_empty(bencher: &mut test::Bencher) {
    empty_explosion(32.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench2_032_half(bencher: &mut test::Bencher) {
    half_explosion(32.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench3_032_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(32.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench4_032_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(32.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench1_064_empty(bencher: &mut test::Bencher) {
    empty_explosion(64.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench2_064_half(bencher: &mut test::Bencher) {
    half_explosion(64.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench3_064_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(64.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench4_064_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(64.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench1_480_empty(bencher: &mut test::Bencher) {
    empty_explosion(480.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench2_480_half(bencher: &mut test::Bencher) {
    half_explosion(480.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench3_480_empty_wall(bencher: &mut test::Bencher) {
    empty_explosion_wall(480.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench4_480_half_wall(bencher: &mut test::Bencher) {
    half_explosion_wall(480.0, bencher, ExplosionManager::explosion);
}
#[bench]
fn bench1_200_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(200.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench2_200_half_lines(bencher: &mut test::Bencher) {
    half_explosion(200.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench3_200_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(200.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench4_200_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(200.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench1_032_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(32.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench2_032_half_lines(bencher: &mut test::Bencher) {
    half_explosion(32.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench3_032_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(32.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench4_032_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(32.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench1_064_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(64.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench2_064_half_lines(bencher: &mut test::Bencher) {
    half_explosion(64.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench3_064_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(64.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench4_064_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(64.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench1_480_empty_lines(bencher: &mut test::Bencher) {
    empty_explosion(480.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench2_480_half_lines(bencher: &mut test::Bencher) {
    half_explosion(480.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench3_480_empty_wall_lines(bencher: &mut test::Bencher) {
    empty_explosion_wall(480.0, bencher, ExplosionManager::explosion_lines);
}
#[bench]
fn bench4_480_half_wall_lines(bencher: &mut test::Bencher) {
    half_explosion_wall(480.0, bencher, ExplosionManager::explosion_lines);
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::as_conversions)]
fn truncate_f32(f: f32) -> isize {
    f as isize
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::as_conversions)]
fn round_f32(f: f32) -> isize {
    f.round() as isize
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::as_conversions)]
fn truncate_f32u(f: f32) -> usize {
    f as usize
}
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::as_conversions)]
fn truncate_usize(f: usize) -> f32 {
    f as f32
}
