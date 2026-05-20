pub mod arc;
pub mod line;
use crate::arc::ArcIter;
use crate::line::LineIter;
use noita_api::{Cell, ConfigExplosion, GameGlobal, StdBox, Vec2};
use rand::RngExt as _;
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
    let ix0 = truncate_f32(pos.x);
    let iy0 = truncate_f32(pos.y);
    let rays: u16 = 16;
    let delta_theta = TAU / f32::from(rays);
    for ray in 0..rays {
        let mut chunk_x = ix0.div_euclid(512);
        let mut chunk_y = iy0.div_euclid(512);
        let Some(mut chunk) =
            chunk_map[(256 + chunk_y).cast_unsigned()][(256 + chunk_x).cast_unsigned()]
        else {
            return;
        };
        let theta = (f32::from(ray) + 0.5) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix1 = ix0 + truncate_f32(cos * config.explosion_radius);
        let iy1 = iy0 + truncate_f32(sin * config.explosion_radius);
        let mut energy = config.ray_energy;
        for (x, y) in LineIter::new(ix0, iy0, ix1, iy1) {
            let px = x.rem_euclid(512).cast_unsigned();
            let py = y.rem_euclid(512).cast_unsigned();
            if px == 0 || py == 0 || px == 511 || py == 511 {
                chunk_x = x.div_euclid(512);
                chunk_y = y.div_euclid(512);
                if let Some(c) =
                    chunk_map[(256 + chunk_y).cast_unsigned()][(256 + chunk_x).cast_unsigned()]
                {
                    chunk = c;
                } else {
                    break;
                }
            }
            let c = &mut chunk[py][px];
            if let Some(p) = c {
                if energy > p.hp
                    && p.material.durability <= config.max_durability_to_destroy
                    && config.hole_enabled
                {
                    energy -= p.hp;
                    p.ptr.free();
                    *c = None;
                } else {
                    break;
                }
            }
        }
        for (x, y) in ArcIter::new() {}
        /*
                    if cell_create_id != 0
                        && rng.random_ratio(
                            u32::try_from(config.create_cell_probability).unwrap(),
                            100,
                        )
                    {
                        println!("{px} {py}");
                        p.ptr.free();
                        *c = Some(Cell::new(cell_create));
                    } else {
        */
        /*let r = if (ix2, iy2) == (ix1, iy1) {
            truncate_f32(config.explosion_radius).pow(2)
        } else {
            ix2 * ix2 + iy2 * iy2
        };
        let rf = truncate_isize(r);
        let theta = f32::from(ray) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix3 = ix0 + truncate_f32(cos * rf);
        let iy3 = iy0 + truncate_f32(sin * rf);
        let theta = f32::from(ray + 1) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix4 = ix0 + truncate_f32(cos * rf);
        let iy4 = iy0 + truncate_f32(sin * rf);*/
    }
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::as_conversions)]
fn truncate_f32(f: f32) -> isize {
    f as isize
}
/*#[allow(clippy::cast_precision_loss)]
#[allow(clippy::as_conversions)]
fn truncate_isize(f: isize) -> f32 {
    f as f32
}*/
