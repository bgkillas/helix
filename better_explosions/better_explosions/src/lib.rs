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
mod uninit_map;
use crate::line::LineIter;
use noita_api::{Cell, CellData, ConfigExplosion, GridWorld, StdBox, this_call};
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
pub struct ExplosionManager {
    pub construct_cell: this_call!(
        fn(StdBox<GridWorld>, isize, isize, StdBox<CellData>, *mut ()) -> Option<Cell<()>>
    ),
    pub lines: Box<[[Option<Vec<LineContinue>>; 512]; 512]>,
}
pub struct LineContinue {
    pub line: LineIter,
    pub config: ConfigExplosion,
    pub cell_create: StdBox<CellData>,
    pub energy: usize,
    pub mult: f32,
}
impl LineContinue {
    #[inline]
    #[must_use]
    pub fn as_ref(&mut self) -> LineContinueRef<'_> {
        LineContinueRef {
            line: &mut self.line,
            config: &self.config,
            cell_create: self.cell_create,
            energy: self.energy,
            mult: self.mult,
        }
    }
}
pub struct LineContinueRef<'a> {
    pub line: &'a mut LineIter,
    pub config: &'a ConfigExplosion,
    pub cell_create: StdBox<CellData>,
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
    fn default() -> Self {
        Self {
            #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
            construct_cell: dummy,
            #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
            construct_cell: noita_api::get_construct_cell(),
            lines: unsafe { Box::new_zeroed().assume_init() },
        }
    }
}
