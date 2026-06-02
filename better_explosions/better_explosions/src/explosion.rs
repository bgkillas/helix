use crate::arc::ArcIter;
use crate::circumference::Circumference;
use crate::line::LineIter;
use crate::octant::octant;
use crate::uninit_map::UninitMap;
use crate::{ExplosionManager, LineContinue, LineContinueRef};
use noita_api::{CellType, ChunkArray, ConfigExplosion, GameGlobal, GridWorld, StdBox, Vec2};
use rand::RngExt as _;
use rand::distr::Bernoulli;
use rand::rngs::ThreadRng;
use std::f32::consts::TAU;
use std::mem::MaybeUninit;
impl ExplosionManager {
    #[inline]
    pub fn explosion(&mut self, config: &ConfigExplosion, pos: Vec2<f32>) {
        let r = truncate_f32u(config.explosion_radius);
        let n = (8 * r.div_ceil(16)).max((((r * 63) / 10) / 16) & !7);
        let rays: u16 = u16::try_from(n).unwrap();
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
            let dy = truncate_usize(iy1.abs_diff(iy0));
            let dx = truncate_usize(ix1.abs_diff(ix0));
            let m = ((if dy > dx { dx / dy } else { dy / dx }).powi(2) + 1.0).sqrt();
            let hp_f = |hp| -> usize { truncate_f32u(truncate_usize(hp) * m) };
            for (_, x, y) in LineIter::new(ix0, iy0, ix1, iy1) {
                let px = x % 512;
                let py = y % 512;
                let Some(c) = chunk_map[y / 512][x / 512] else {
                    (ix2, iy2) = (x, y);
                    break;
                };
                if let Some(p) = c[py][px] {
                    if matches!(p.material.cell_type, CellType::Solid) {
                        continue;
                    }
                    if !config.hole_enabled
                        || p.material.durability > config.max_durability_to_destroy
                    {
                        (ix2, iy2) = (x, y);
                        break;
                    }
                    let Some(new) = energy.checked_sub(hp_f(p.hp)) else {
                        (ix2, iy2) = (x, y);
                        break;
                    };
                    energy = new;
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
                        if p.material.durability > config.max_durability_to_destroy
                            || !config.hole_enabled
                            || matches!(p.material.cell_type, CellType::Solid)
                        {
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
    pub fn explosion_chunk_update(&mut self) {
        let mut rng = rand::rng();
        let game_global = GameGlobal::global();
        let grid_world = game_global.m_grid_world;
        let chunk_map = grid_world.chunk_map.chunk_array;
        let mut hp_map = UninitMap::new(512 * 512 * grid_world.chunk_map.chunk_count);
        let mut chunk_indices: Box<[[MaybeUninit<usize>; 512]; 512]> =
            unsafe { Box::new_uninit().assume_init() };
        for (i, (x, y, _)) in grid_world.chunk_map.flat_iter().enumerate() {
            chunk_indices[usize::from(y)][usize::from(x)].write(i);
        }
        for (x, y, _) in chunk_map.iter() {
            if let Some(v) = self.lines[usize::from(y)][usize::from(x)].take() {
                for line in v {
                    self.explosion_line_ref(
                        line.as_ref(),
                        &mut rng,
                        grid_world,
                        chunk_map,
                        &mut hp_map,
                        &mut chunk_indices,
                        false,
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
            u32::try_from(config.create_cell_probability).unwrap()
        };
        let mut hp_map = UninitMap::new(512 * 512 * grid_world.chunk_map.chunk_count);
        let mut chunk_indices: Box<[[MaybeUninit<usize>; 512]; 512]> =
            unsafe { Box::new_uninit().assume_init() };
        for (i, (x, y, _)) in grid_world.chunk_map.flat_iter().enumerate() {
            chunk_indices[usize::from(y)][usize::from(x)].write(i);
        }
        let bern = Bernoulli::from_ratio(chance, 100).unwrap();
        let r = truncate_f32u(config.explosion_radius);
        for (ix1, iy1) in Circumference::new(r) {
            octant(ix0, iy0, ix1, iy1, |_, ix2, iy2| {
                let dy = truncate_usize(iy2.abs_diff(iy0));
                let dx = truncate_usize(ix2.abs_diff(ix0));
                let mult = ((if dy > dx { dx / dy } else { dy / dx }).powi(2) + 1.0).sqrt();
                self.explosion_line_ref(
                    LineContinueRef {
                        line: LineIter::new(ix0, iy0, ix2, iy2),
                        config,
                        cell_create,
                        bern,
                        energy: config.ray_energy,
                        mult,
                    },
                    &mut rng,
                    grid_world,
                    chunk_map,
                    &mut hp_map,
                    &mut chunk_indices,
                    true,
                );
            });
        }
    }
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn explosion_line_ref(
        &mut self,
        mut line: LineContinueRef<'_>,
        rng: &mut ThreadRng,
        grid_world: StdBox<GridWorld>,
        chunk_map: ChunkArray,
        hp_map: &mut UninitMap<usize>,
        chunk_indices: &mut [[MaybeUninit<usize>; 512]; 512],
        is_initial: bool,
    ) {
        let hp_f = |hp| -> usize { truncate_f32u(truncate_usize(hp) * line.mult) };
        while let Some((case, mut x, y)) = line.line.next() {
            let mut cx = x / 512;
            let cy = y / 512;
            let Some(mut c) = chunk_map[cy][cx] else {
                let vec = self.lines[cy][cx].get_or_insert_with(|| Vec::with_capacity(512));
                line.line.back(case);
                vec.push(LineContinue {
                    line: line.line,
                    config: line.config.clone(),
                    cell_create: line.cell_create,
                    bern: line.bern,
                    energy: line.energy,
                    mult: line.mult,
                });
                break;
            };
            let mut px = x % 512;
            let py = y % 512;
            let mut i = unsafe { chunk_indices[cy][cx].assume_init() };
            let mut hp_index = 512 * 512 * i + 512 * py + px;
            if let Some(hp) = hp_map.get(hp_index) {
                if let Some(new) = line.energy.checked_sub(hp_f(hp)) {
                    line.energy = new;
                } else {
                    break;
                }
            } else {
                if let Some(p) = c[py][px] {
                    if matches!(p.material.cell_type, CellType::Solid) {
                        continue;
                    }
                    if !line.config.hole_enabled
                        || p.material.durability > line.config.max_durability_to_destroy
                    {
                        break;
                    }
                    let Some(new) = line.energy.checked_sub(hp_f(p.hp)) else {
                        break;
                    };
                    hp_map.insert(hp_index, p.hp);
                    line.energy = new;
                    p.ptr.free();
                } else {
                    hp_map.insert(hp_index, 0);
                }
                if rng.sample(line.bern) {
                    c[py][px] = (self.construct_cell)(
                        grid_world,
                        x.cast_signed() - 512 * 256,
                        y.cast_signed() - 512 * 256,
                        line.cell_create,
                        std::ptr::null_mut(),
                    );
                } else {
                    c[py][px] = None;
                }
            }
            x += 1;
            cx = x / 512;
            if let Some(d) = chunk_map[cy][cx] {
                c = d;
            } else {
                continue;
            }
            px = x % 512;
            i = unsafe { chunk_indices[cy][cx].assume_init() };
            hp_index = 512 * 512 * i + 512 * py + px;
            if hp_map.get(hp_index).is_none() {
                if let Some(p) = c[py][px] {
                    if !line.config.hole_enabled
                        || p.material.durability > line.config.max_durability_to_destroy
                        || matches!(p.material.cell_type, CellType::Solid)
                    {
                        continue;
                    }
                    hp_map.insert(hp_index, p.hp);
                    p.ptr.free();
                } else {
                    hp_map.insert(hp_index, 0);
                }
                if rng.sample(line.bern) {
                    c[py][px] = (self.construct_cell)(
                        grid_world,
                        x.cast_signed() - 512 * 256,
                        y.cast_signed() - 512 * 256,
                        line.cell_create,
                        std::ptr::null_mut(),
                    );
                } else {
                    c[py][px] = None;
                }
            }
            if !is_initial {
                x -= 2;
                cx = x / 512;
                if let Some(d) = chunk_map[cy][cx] {
                    c = d;
                } else {
                    continue;
                }
                px = x % 512;
                i = unsafe { chunk_indices[cy][cx].assume_init() };
                hp_index = 512 * 512 * i + 512 * py + px;
                if hp_map.get(hp_index).is_none() {
                    if let Some(p) = c[py][px] {
                        if !line.config.hole_enabled
                            || p.material.durability > line.config.max_durability_to_destroy
                        {
                            continue;
                        }
                        hp_map.insert(hp_index, p.hp);
                        p.ptr.free();
                    } else {
                        hp_map.insert(hp_index, 0);
                    }
                    if rng.sample(line.bern) {
                        c[py][px] = (self.construct_cell)(
                            grid_world,
                            x.cast_signed() - 512 * 256,
                            y.cast_signed() - 512 * 256,
                            line.cell_create,
                            std::ptr::null_mut(),
                        );
                    } else {
                        c[py][px] = None;
                    }
                }
            }
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
