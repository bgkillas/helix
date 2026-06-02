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
use crate::line::LineIter;
use noita_api::{Cell, CellData, ChunkArrayGeneric, ConfigExplosion, GridWorld, StdBox, this_call};
use rand::distr::Bernoulli;
use std::mem;
use std::mem::MaybeUninit;
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
            self.explosion_lines(&config, *pos);
        }
        #[lua_function]
        fn update(&mut self) {
            self.explosion_chunk_update();
        }
    }
}
pub struct ExplosionManager {
    pub construct_cell: this_call!(
        fn(StdBox<GridWorld>, isize, isize, StdBox<CellData>, *mut ()) -> Option<Cell<()>>
    ),
    pub lines: Box<ChunkArrayGeneric<Option<Vec<LineContinue>>>>,
}
pub struct LineContinue {
    pub line: LineIter,
    pub config: ConfigExplosion,
    pub cell_create: StdBox<CellData>,
    pub bern: Bernoulli,
    pub energy: usize,
    pub mult: f32,
}
impl LineContinue {
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> LineContinueRef<'_> {
        LineContinueRef {
            line: self.line.clone(),
            config: &self.config,
            cell_create: self.cell_create,
            bern: self.bern,
            energy: self.energy,
            mult: self.mult,
        }
    }
}
pub struct LineContinueRef<'a> {
    pub line: LineIter,
    pub config: &'a ConfigExplosion,
    pub cell_create: StdBox<CellData>,
    pub bern: Bernoulli,
    pub energy: usize,
    pub mult: f32,
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
