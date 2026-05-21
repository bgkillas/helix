use crate::{Cell, StdBox, Vec2};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
#[repr(C)]
#[derive(Debug)]
pub struct ChunkMap {
    pub len: usize,
    pub unknown: isize,
    pub chunk_array: StdBox<ChunkArray>,
    pub chunk_count: usize,
    pub min_chunk: Vec2<isize>,
    pub max_chunk: Vec2<isize>,
    pub min_pixel: Vec2<isize>,
    pub max_pixel: Vec2<isize>,
}
#[repr(transparent)]
pub struct ChunkArray {
    pub array: [[Option<StdBox<Chunk>>; 512]; 512],
}
impl ChunkArray {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (u16, u16, Option<StdBox<Chunk>>)> {
        self.array.iter().enumerate().flat_map(|(y, yc)| {
            yc.iter()
                .copied()
                .enumerate()
                .map(move |(x, xc)| (u16::try_from(x).unwrap(), u16::try_from(y).unwrap(), xc))
        })
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u16, u16, &mut Option<StdBox<Chunk>>)> {
        self.array.iter_mut().enumerate().flat_map(|(y, yc)| {
            yc.iter_mut()
                .enumerate()
                .map(move |(x, xc)| (u16::try_from(x).unwrap(), u16::try_from(y).unwrap(), xc))
        })
    }
    #[inline]
    pub fn flat_iter(&self) -> impl Iterator<Item = (u16, u16, StdBox<Chunk>)> {
        self.iter().filter_map(|(x, y, oc)| oc.map(|c| (x, y, c)))
    }
}
impl Default for ChunkArray {
    #[inline]
    #[allow(clippy::large_stack_arrays)]
    fn default() -> Self {
        Self {
            array: [[None; 512]; 512],
        }
    }
}
impl Debug for ChunkArray {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.flat_iter().map(|(x, y, _)| (x, y)))
            .finish()
    }
}
impl Deref for ChunkArray {
    type Target = [[Option<StdBox<Chunk>>; 512]; 512];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}
impl DerefMut for ChunkArray {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}
#[repr(transparent)]
#[derive(Debug)]
pub struct Chunk {
    pub data: StdBox<[[Option<Cell<()>>; 512]; 512]>,
}
impl Chunk {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (u16, u16, Option<Cell<()>>)> {
        self.data.iter().enumerate().flat_map(|(y, yc)| {
            yc.iter()
                .copied()
                .enumerate()
                .map(move |(x, xc)| (u16::try_from(x).unwrap(), u16::try_from(y).unwrap(), xc))
        })
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u16, u16, &mut Option<Cell<()>>)> {
        self.data.iter_mut().enumerate().flat_map(|(y, yc)| {
            yc.iter_mut()
                .enumerate()
                .map(move |(x, xc)| (u16::try_from(x).unwrap(), u16::try_from(y).unwrap(), xc))
        })
    }
    #[inline]
    pub fn flat_iter(&self) -> impl Iterator<Item = (u16, u16, Cell<()>)> {
        self.iter().filter_map(|(x, y, oc)| oc.map(|c| (x, y, c)))
    }
}
impl Default for Chunk {
    #[inline]
    #[allow(clippy::large_stack_arrays)]
    fn default() -> Self {
        Self {
            data: StdBox::new([[None; 512]; 512]),
        }
    }
}
impl Deref for Chunk {
    type Target = [[Option<Cell<()>>; 512]; 512];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for Chunk {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
