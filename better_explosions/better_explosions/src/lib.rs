mod line;
use crate::line::line;
use noita_api::{ConfigExplosion, GameGlobal, Vec2};
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
pub(crate) fn explosion(config: &ConfigExplosion, pos: Vec2<f32>) {
    let rng = rand::rng();
    let game_global = GameGlobal::global();
    let grid_world = game_global.m_grid_world;
    let chunk_map = grid_world.chunk_map.chunk_array;
    let ix0 = truncate_f32(pos.x);
    let iy0 = truncate_f32(pos.y);
    let mut chunk_x = ix0.div_euclid(512);
    let mut chunk_y = iy0.div_euclid(512);
    let mut chunk =
        chunk_map[(256 + chunk_y).cast_unsigned()][(256 + chunk_x).cast_unsigned()].unwrap();
    _ = rng;
    let rays: u16 = 16;
    let delta_theta = TAU / f32::from(rays);
    for ray in 0..rays {
        let theta = f32::from(ray) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix1 = ix0 + truncate_f32(cos * config.explosion_radius);
        let iy1 = iy0 + truncate_f32(sin * config.explosion_radius);
        line(
            |x, y| {
                let px = x.rem_euclid(512).cast_unsigned();
                let py = y.rem_euclid(512).cast_unsigned();
                if px == 0 || py == 0 {
                    chunk_x = x.div_euclid(512);
                    chunk_y = y.div_euclid(512);
                    chunk = chunk_map[(256 + chunk_y).cast_unsigned()]
                        [(256 + chunk_x).cast_unsigned()]
                    .unwrap();
                }
                let c = &mut chunk[py][px];
                if let Some(p) = c {
                    p.ptr.free();
                }
                *c = None;
                false
            },
            ix0,
            iy0,
            ix1,
            iy1,
        );
    }
}
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::as_conversions)]
fn truncate_f32(f: f32) -> isize {
    f as isize
}
