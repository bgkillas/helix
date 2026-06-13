use crate::arc::ArcIter;
use crate::circumference::Circumference;
use crate::line::{LineIter, LineIterCompact, StepCase};
use crate::octant::octant;
use crate::uninit_map::UninitMap;
use noita_api::{
    Cell, CellData, CellType, ChunkArray, ChunkArrayGeneric, ConfigExplosion, GameGlobal,
    GridWorld, StdBox, Vec2, this_call,
};
use rand::RngExt as _;
use rand::distr::Bernoulli;
use rand::rngs::ThreadRng;
use std::f32::consts::TAU;
use std::mem::MaybeUninit;
use std::rc::Rc;
pub struct ExplosionManager {
    pub construct_cell: this_call!(
        fn(StdBox<GridWorld>, isize, isize, StdBox<CellData>, *mut ()) -> Option<Cell<()>>
    ),
    pub lines: Box<ChunkArrayGeneric<Option<Vec<LineContinue>>>>,
}
unsafe impl Send for ExplosionManager {}
unsafe impl Sync for ExplosionManager {}
#[derive(Clone)]
pub struct LineContinue {
    pub line: LineIterCompact,
    pub config: Rc<ConfigExplosion>,
    pub energy: usize,
}
impl ExplosionManager {
    #[inline]
    pub fn explosion(&mut self, config: &ConfigExplosion, pos: Vec2<f32>) {
        let r = truncate_f32u(config.explosion_radius);
        let n = (8 * r.div_ceil(16)).max((((r * 63) / 10) / 16) & !7);
        let rays: u16 = n.strict_cast::<u16>();
        self.explosion_with_rays(config, pos, rays);
    }
    #[inline]
    pub fn explosion_with_rays(&mut self, config: &ConfigExplosion, pos: Vec2<f32>, rays: u16) {
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
            Bernoulli::from_ratio(config.create_cell_probability.strict_cast(), 100).unwrap()
        };
        let mut radii = Vec::with_capacity(rays.strict_cast::<usize>());
        for ray in 0..rays {
            let theta = (f32::from(ray) + 0.5) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix1 =
                (ix0.cast_signed() + round_f32(cos * config.explosion_radius)).cast_unsigned();
            let iy1 =
                (iy0.cast_signed() + round_f32(sin * config.explosion_radius)).cast_unsigned();
            let dy = truncate_usize(iy1.abs_diff(iy0));
            let dx = truncate_usize(ix1.abs_diff(ix0));
            let mult = ((if dy > dx { dx / dy } else { dy / dx }).powi(2) + 1.0).sqrt();
            let mut energy = truncate_f32u(truncate_usize(config.ray_energy) / mult);
            let (mut ix2, mut iy2) = (ix1, iy1);
            for (_, x, y) in LineIter::new(ix0, iy0, ix1, iy1) {
                let px = x % 512;
                let py = y % 512;
                let Some(c) = chunk_map[y / 512][x / 512] else {
                    (ix2, iy2) = (x, y);
                    break;
                };
                let Some(p) = c[py][px] else {
                    continue;
                };
                if matches!(p.material.cell_type, CellType::Solid) {
                    continue;
                }
                if !config.hole_enabled || p.material.durability > config.max_durability_to_destroy
                {
                    (ix2, iy2) = (x, y);
                    break;
                }
                let Some(new) = energy.checked_sub(p.hp) else {
                    (ix2, iy2) = (x, y);
                    break;
                };
                energy = new;
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
            let ray = rayu.strict_cast::<u16>();
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
                let Some(mut c) = chunk_map[y / 512][x / 512] else {
                    continue;
                };
                if let Some(p) = c[py][px] {
                    if p.material.durability > config.max_durability_to_destroy
                        || !config.hole_enabled
                        || matches!(p.material.cell_type, CellType::Solid)
                    {
                        continue;
                    }
                    p.ptr.free();
                }
                c[py][px] = if rng.sample(bern) {
                    (self.construct_cell)(
                        grid_world,
                        x.cast_signed() - 512 * 256,
                        y.cast_signed() - 512 * 256,
                        cell_create,
                        std::ptr::null_mut(),
                    )
                } else {
                    None
                };
            }
        }
    }
    #[inline]
    pub fn explosion_chunk_update(&mut self) {
        let mut rng = rand::rng();
        let game_global = GameGlobal::global();
        let grid_world = game_global.m_grid_world;
        let chunk_map = grid_world.chunk_map.chunk_array;
        let mut hp_map = UninitMap::new(512 * 512 * grid_world.chunk_map.chunk_count);
        let mut chunk_indices: Box<[[MaybeUninit<usize>; 512]; 512]> =
            unsafe { Box::new_uninit().assume_init() };
        for (i, (x, y, _)) in grid_world.chunk_map.flat_iter().enumerate() {
            chunk_indices[y.strict_cast::<usize>()][x.strict_cast::<usize>()].write(i);
        }
        for (x, y, _) in chunk_map.iter() {
            if let Some(v) = self.lines[y.strict_cast::<usize>()][x.strict_cast::<usize>()].take() {
                for line in v {
                    let cell_create_id = game_global
                        .m_cell_factory
                        .material_ids
                        .get(&line.config.create_cell_material)
                        .copied()
                        .unwrap_or_default();
                    let cell_create =
                        StdBox::from(&game_global.m_cell_factory.cell_data[cell_create_id]);
                    let chance = if cell_create_id == 0 {
                        0
                    } else {
                        line.config.create_cell_probability.strict_cast()
                    };
                    let bern = Bernoulli::from_ratio(chance, 100).unwrap();
                    self.explosion_line_in_chunk(
                        line,
                        &mut rng,
                        grid_world,
                        chunk_map,
                        &mut hp_map,
                        &mut chunk_indices,
                        bern,
                        cell_create,
                    );
                }
            }
        }
    }
    #[inline]
    pub fn explosion_lines(&mut self, config: &ConfigExplosion, pos: Vec2<f32>) {
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
            config.create_cell_probability.strict_cast()
        };
        let bern = Bernoulli::from_ratio(chance, 100).unwrap();
        let mut hp_map = UninitMap::new(512 * 512 * grid_world.chunk_map.chunk_count);
        let mut chunk_indices: Box<[[MaybeUninit<usize>; 512]; 512]> =
            unsafe { Box::new_uninit().assume_init() };
        for (i, (x, y, _)) in grid_world.chunk_map.flat_iter().enumerate() {
            chunk_indices[y.strict_cast::<usize>()][x.strict_cast::<usize>()].write(i);
        }
        let r = truncate_f32u(config.explosion_radius);
        let config_arc = Rc::new(config.clone());
        let energy = config.ray_energy;
        for (ix1, iy1) in Circumference::new(r) {
            octant(ix0, iy0, ix1, iy1, |_, ix2, iy2| {
                let dy = truncate_usize(iy2.abs_diff(iy0));
                let dx = truncate_usize(ix2.abs_diff(ix0));
                let mult = ((if dy > dx { dx / dy } else { dy / dx }).powi(2) + 1.0).sqrt();
                self.explosion_line(
                    LineContinue {
                        line: LineIter::new(ix0, iy0, ix2, iy2).into(),
                        config: config_arc.clone(),
                        energy: truncate_f32u(truncate_usize(energy) / mult),
                    },
                    &mut rng,
                    grid_world,
                    chunk_map,
                    &mut hp_map,
                    &mut chunk_indices,
                    bern,
                    cell_create,
                );
            });
        }
    }
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn explosion_line_in_chunk<const ORIG: bool>(
        &mut self,
        mut line: LineContinue,
        rng: &mut ThreadRng,
        grid_world: StdBox<GridWorld>,
        chunk_map: ChunkArray,
        hp_map: &mut UninitMap<usize>,
        chunk_indices: &mut [[MaybeUninit<usize>; 512]; 512],
        bern: Bernoulli,
        cell_create: StdBox<CellData>,
    ) {
        let mut line_iter = LineIter::from(line.line);
        while let Some((_, mut x, mut y)) = line_iter.next() {

        }
    }
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn explosion_line(
        &mut self,
        mut line: LineContinue,
        rng: &mut ThreadRng,
        grid_world: StdBox<GridWorld>,
        chunk_map: ChunkArray,
        hp_map: &mut UninitMap<usize>,
        chunk_indices: &mut [[MaybeUninit<usize>; 512]; 512],
        bern: Bernoulli,
        cell_create: StdBox<CellData>,
    ) {
        let mut line_iter = LineIter::from(line.line);
        let mut last = (0, 0);
        let energy_orig = line.energy;
        let mut i: usize = 0;
        while let Some((case, mut x, mut y)) = line_iter.next() {
            let mut cx = x / 512;
            let mut cy = y / 512;
            let Some(mut c) = chunk_map[cy][cx] else {
                    if last == (cx, cy) {
                        if energy_orig == line.energy {
                            i += 1;
                            continue;
                        }
                        let hp = (energy_orig - line.energy) / i;
                        let Some(new) = line.energy.checked_sub(hp) else {
                            break;
                        };
                        line.energy = new;
                    } else {
                        last = (cx, cy);
                        let vec = self.lines[cy][cx].get_or_insert_with(|| Vec::with_capacity(512));
                        let mut line_iter_back = line_iter.clone();
                        line_iter_back.back(case);
                        line.line = line_iter_back.into();
                        vec.push(line.clone());
                    }
                    i += 1;
                    continue;
            };
            i += 1;
            let mut px = x % 512;
            let mut py = y % 512;
            let mut i = unsafe { chunk_indices[cy][cx].assume_init() };
            let mut hp_index = 512 * 512 * i + 512 * py + px;
            if let Some(hp) = hp_map.get(hp_index) {
                let Some(new) = line.energy.checked_sub(*hp) else {
                    break;
                };
                line.energy = new;
            } else {
                let hp = if let Some(p) = c[py][px] {
                    if matches!(p.material.cell_type, CellType::Solid) {
                        continue;
                    }
                    if !line.config.hole_enabled
                        || p.material.durability > line.config.max_durability_to_destroy
                    {
                        break;
                    }
                    let Some(new) = line.energy.checked_sub(p.hp) else {
                        break;
                    };
                    line.energy = new;
                    p.ptr.free();
                    p.hp
                } else {
                    0
                };
                hp_map.insert(hp_index, hp);
                c[py][px] = if rng.sample(bern) {
                    (self.construct_cell)(
                        grid_world,
                        x.cast_signed() - 512 * 256,
                        y.cast_signed() - 512 * 256,
                        cell_create,
                        std::ptr::null_mut(),
                    )
                } else {
                    None
                };
            }
            if !matches!(case, StepCase::Both) {
                continue;
            }
            if line_iter.dy_abs > line_iter.dx_abs {
                if line_iter.dy_neg {
                    y += 1;
                } else {
                    y -= 1;
                }
                cy = y / 512;
                py = y % 512;
            } else {
                if line_iter.dx_neg {
                    x += 1;
                } else {
                    x -= 1;
                }
                cx = x / 512;
                px = x % 512;
            }
            let Some(d) = chunk_map[cy][cx] else {
                continue;
            };
            c = d;
            i = unsafe { chunk_indices[cy][cx].assume_init() };
            hp_index = 512 * 512 * i + 512 * py + px;
            if hp_map.get(hp_index).is_some() {
                continue;
            }
            let hp = if let Some(p) = c[py][px] {
                if !line.config.hole_enabled
                    || p.material.durability > line.config.max_durability_to_destroy
                    || matches!(p.material.cell_type, CellType::Solid)
                {
                    continue;
                }
                p.ptr.free();
                p.hp
            } else {
                0
            };
            hp_map.insert(hp_index, hp);
            c[py][px] = if rng.sample(bern) {
                (self.construct_cell)(
                    grid_world,
                    x.cast_signed() - 512 * 256,
                    y.cast_signed() - 512 * 256,
                    cell_create,
                    std::ptr::null_mut(),
                )
            } else {
                None
            };
        }
    }
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
#[cfg(feature = "test")]
#[test]
pub fn lines_colors() {
    let ix0 = 68;
    let iy0 = 68;
    let r: u16 = 64;
    let mut image = image::RgbImage::new(
        2 * r.strict_cast::<u32>() + 8,
        2 * r.strict_cast::<u32>() + 8,
    );
    for (i, (ix1, iy1)) in Circumference::new(r.strict_cast::<usize>()).enumerate() {
        octant(ix0, iy0, ix1, iy1, |o, ix2, iy2| {
            for (case, x, y) in LineIter::new(ix0, iy0, ix2, iy2) {
                let mut p = &mut image
                    .get_pixel_mut(x.try_into().unwrap(), y.try_into().unwrap())
                    .0;
                p[0] = 128 + 32 * (i % 4).strict_cast::<u8>();
                p[1] = 128 + 32 * (i % 4).strict_cast::<u8>();
                p[2] = 128 + 32 * (i % 4).strict_cast::<u8>();
                if matches!(case, crate::line::StepCase::Both) {
                    if o == 1 || o == 2 || o == 5 || o == 6 {
                        p = &mut image
                            .get_pixel_mut(
                                x.try_into().unwrap(),
                                if o == 1 || o == 2 { y - 1 } else { y + 1 }
                                    .try_into()
                                    .unwrap(),
                            )
                            .0;
                    } else {
                        p = &mut image
                            .get_pixel_mut(
                                if o == 0 || o == 7 { x - 1 } else { x + 1 }
                                    .try_into()
                                    .unwrap(),
                                y.try_into().unwrap(),
                            )
                            .0;
                    }
                    p[0] = 128 + 32 * (i % 4).strict_cast::<u8>();
                    p[1] = 128 + 32 * (i % 4).strict_cast::<u8>();
                    p[2] = 128 + 32 * (i % 4).strict_cast::<u8>();
                }
            }
        })
    }
    image.save("../../test_line.png").unwrap();
}
