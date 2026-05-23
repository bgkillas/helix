pub mod arc;
pub mod line;
use crate::arc::ArcIter;
use crate::line::LineIter;
use noita_api::{Cell, ConfigExplosion, GameGlobal, StdBox, Vec2};
use rand::RngExt as _;
use rand::distr::Bernoulli;
use std::f32::consts::TAU;
#[noita_api::lua_module]
mod lua {
    use crate::explosion;
    use noita_api::{ConfigExplosion, ExplosionFun, StdBox, Vec2};
    #[explosion_hook]
    fn on_explosion(
        _: ExplosionFun,
        config: StdBox<ConfigExplosion>,
        pos: StdBox<Vec2<f32>>,
        _: isize,
    ) {
        explosion(&config, *pos);
    }
}
#[inline]
pub fn explosion(config: &ConfigExplosion, pos: Vec2<f32>) {
    let rays: u16 = 32;
    explosion_with_rays(config, pos, rays);
}
#[inline]
pub fn explosion_with_rays(config: &ConfigExplosion, pos: Vec2<f32>, rays: u16) {
    let mut rng = rand::rng();
    let game_global = GameGlobal::global();
    let cell_create_id = *game_global
        .m_cell_factory
        .material_ids
        .get(&config.create_cell_material)
        .unwrap();
    let cell_create = StdBox::from(&game_global.m_cell_factory.cell_data[cell_create_id]);
    let grid_world = game_global.m_grid_world;
    let chunk_map = grid_world.chunk_map.chunk_array;
    let ix0 = (512 * 256 + truncate_f32(pos.x)).cast_unsigned();
    let iy0 = (512 * 256 + truncate_f32(pos.y)).cast_unsigned();
    let delta_theta = TAU / f32::from(rays);
    let bern =
        Bernoulli::from_ratio(u32::try_from(config.create_cell_probability).unwrap(), 100).unwrap();
    for ray in 0..rays {
        let Some(mut chunk) = chunk_map[iy0 / 512][ix0 / 512] else {
            return;
        };
        let theta = (f32::from(ray) + 0.5) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix1 = (ix0.cast_signed() + truncate_f32(cos * config.explosion_radius)).cast_unsigned();
        let iy1 = (iy0.cast_signed() + truncate_f32(sin * config.explosion_radius)).cast_unsigned();
        let mut energy = config.ray_energy;
        let (mut ix2, mut iy2) = (ix1, iy1);
        for (x, y) in LineIter::new(ix0, iy0, ix1, iy1) {
            let px = x % 512;
            let py = y % 512;
            if px == 0 || py == 0 || px == 511 || py == 511 {
                if let Some(c) = chunk_map[y / 512][x / 512] {
                    chunk = c;
                } else {
                    break;
                }
            }
            if let Some(p) = chunk[py][px] {
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
        }
        let r = if (ix2, iy2) == (ix1, iy1) {
            truncate_f32(config.explosion_radius)
        } else {
            truncate_f32(truncate_usize(ix2).hypot(truncate_usize(iy2)))
        };
        let rf = truncate_isize(r);
        let theta = f32::from(ray) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix3 = (ix0.cast_signed() + truncate_f32(cos * rf)).cast_unsigned();
        let iy3 = (iy0.cast_signed() + truncate_f32(sin * rf)).cast_unsigned();
        let theta = f32::from(ray + 1) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix4 = (ix0.cast_signed() + truncate_f32(cos * rf)).cast_unsigned();
        let iy4 = (iy0.cast_signed() + truncate_f32(sin * rf)).cast_unsigned();
        for (x, y) in ArcIter::new(ix0, iy0, ix3, iy3, ix4, iy4, r * r) {
            let px = x % 512;
            let py = y % 512;
            if px == 0 || py == 0 || px == 511 || py == 511 {
                if let Some(c) = chunk_map[y / 512][x / 512] {
                    chunk = c;
                } else {
                    break;
                }
            }
            if let Some(p) = chunk[py][px] {
                p.ptr.free();
            }
            if cell_create_id != 0 && rng.sample(bern) {
                chunk[py][px] = Some(Cell::new(cell_create));
            } else {
                chunk[py][px] = None;
            }
        }
    }
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::as_conversions)]
fn truncate_f32(f: f32) -> isize {
    f as isize
}
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::as_conversions)]
fn truncate_isize(f: isize) -> f32 {
    f as f32
}
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::as_conversions)]
fn truncate_usize(f: usize) -> f32 {
    f as f32
}
