use crate::world::{Pixel, PixelRun, get_section_mut_enumerate};
use noita_api::{
    Cell, CellData, CellType, GameGlobal, GridWorld, StdBox, get_construct_cell, this_call,
};
use std::ptr;
#[derive(Clone, Copy)]
pub struct WorldWrite {
    construct_cell: this_call!(
        fn(StdBox<GridWorld>, isize, isize, StdBox<CellData>, *mut ()) -> Option<StdBox<Cell>>
    ),
}
unsafe impl Send for WorldWrite {}
unsafe impl Sync for WorldWrite {}
impl Default for WorldWrite {
    fn default() -> Self {
        let construct_cell = get_construct_cell();
        Self { construct_cell }
    }
}
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct ChunkWrite {
    pub pixel_run: PixelRun,
    pub x: usize,
    pub y: usize,
    pub section: u8,
}
impl WorldWrite {
    pub fn write_chunks(self, chunks: &[ChunkWrite]) {
        let mut game_global = GameGlobal::global();
        let grid_world = game_global.m_grid_world;
        let map = &grid_world.chunk_map;
        for chunk in chunks {
            if let Some(mut real_chunk) = map.chunk_array[chunk.y][chunk.x] {
                for ((sx, sy, pixel), new) in
                    get_section_mut_enumerate(usize::from(chunk.section), &mut real_chunk.data)
                        .zip(chunk.pixel_run.iter())
                        .filter(|((_, _, p), n)| {
                            *n != Pixel::MAX
                                && p.map_or(n.id != 0, |v| {
                                    v.material.material_type != usize::from(n.id)
                                        && !matches!(v.material.cell_type, CellType::Solid)
                                })
                        })
                {
                    if let Some(inner) = pixel {
                        inner.free();
                        *pixel = None;
                    }
                    if new.id != 0 {
                        let mat = StdBox::from(
                            &mut game_global.m_cell_factory.cell_data[usize::from(new.id)],
                        );
                        let x = (chunk.x.cast_signed() - 256) * 512 + sx.cast_signed();
                        let y = (chunk.y.cast_signed() - 256) * 512 + sy.cast_signed();
                        if let Some(cell) =
                            (self.construct_cell)(grid_world, x, y, mat, ptr::null_mut())
                        {
                            *pixel = Some(cell);
                        } else {
                            *pixel = None;
                        }
                    }
                }
            }
        }
    }
}
