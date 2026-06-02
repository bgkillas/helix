#![feature(sync_unsafe_cell)]
#![feature(test)]
#![feature(allocator_api)]
#![feature(ptr_as_uninit)]
#![feature(slice_ptr_get)]
extern crate test;
pub mod arc;
#[cfg(test)]
mod benches;
pub mod circle;
pub mod circumference;
pub mod explosion;
pub mod line;
pub mod octant;
pub mod uninit_map;
use crate::explosion::{ExplosionManager, LineContinue};
use noita_api::{Cell, CellData, ChunkArrayGeneric, GridWorld, StdBox};
use std::mem;
use std::mem::MaybeUninit;
#[noita_api::lua_module]
mod lua {
    use crate::explosion::ExplosionManager;
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
            self.explosion_lines(&config, *pos);
        }
        #[lua_function]
        fn update(&mut self) {
            self.explosion_chunk_update();
        }
    }
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
    #[allow(clippy::missing_transmute_annotations)]
    fn default() -> Self {
        let mut lines_uninit: Box<ChunkArrayGeneric<MaybeUninit<Option<Vec<LineContinue>>>>> = unsafe {
            mem::transmute(Box::<ChunkArrayGeneric<Option<Vec<LineContinue>>>>::new_uninit())
        };
        for arr in &mut lines_uninit.array {
            for val in arr {
                val.write(None);
            }
        }
        Self {
            #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
            construct_cell: dummy,
            #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
            construct_cell: noita_api::get_construct_cell(),
            lines: unsafe { mem::transmute(lines_uninit) },
        }
    }
}
