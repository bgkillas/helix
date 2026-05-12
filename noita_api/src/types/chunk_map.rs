use crate::{Cell, StdBox, Vec2};
use std::fmt::Debug;
#[repr(C)]
#[derive(Debug)]
pub struct ChunkMap {
    pub len: usize,
    pub unknown: isize,
    pub chunk_array: StdBox<[[Option<StdBox<Chunk>>; 512]; 512]>,
    pub chunk_count: usize,
    pub min_chunk: Vec2<isize>,
    pub max_chunk: Vec2<isize>,
    pub min_pixel: Vec2<isize>,
    pub max_pixel: Vec2<isize>,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chunk {
    pub data: StdBox<[[Option<Cell<()>>; 512]; 512]>,
}
