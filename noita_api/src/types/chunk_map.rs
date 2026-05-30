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
impl Default for ChunkMap {
    #[inline]
    fn default() -> Self {
        Self {
            len: 512,
            unknown: 0,
            chunk_array: StdBox::new(ChunkArray::default()),
            chunk_count: 0,
            min_chunk: Vec2 {
                x: isize::MAX,
                y: isize::MAX,
            },
            max_chunk: Vec2 {
                x: isize::MIN,
                y: isize::MIN,
            },
            min_pixel: Vec2 {
                x: isize::MAX,
                y: isize::MAX,
            },
            max_pixel: Vec2 {
                x: isize::MIN,
                y: isize::MIN,
            },
        }
    }
}
#[repr(transparent)]
pub struct ChunkArray {
    pub array: [[Option<StdBox<Chunk>>; 512]; 512],
}
impl ChunkMap {
    #[inline]
    pub fn flat_iter(&self) -> impl Iterator<Item = (u16, u16, StdBox<Chunk>)> {
        (self.min_chunk.y..=self.max_chunk.y).flat_map(move |yi| {
            let y = usize::try_from(256 + yi).unwrap();
            (self.min_chunk.x..=self.max_chunk.x).filter_map(move |xi| {
                let x = usize::try_from(256 + xi).unwrap();
                self.chunk_array[y][x]
                    .map(|c| (u16::try_from(x).unwrap(), u16::try_from(y).unwrap(), c))
            })
        })
    }
    #[inline]
    pub fn clear(&mut self) {
        for yi in self.min_chunk.y..=self.max_chunk.y {
            let y = (yi + 256).cast_unsigned();
            for xi in self.min_chunk.x..=self.max_chunk.x {
                let x = (xi + 256).cast_unsigned();
                if let Some(mut n) = self.chunk_array[y][x].take() {
                    for (_, _, p) in n.flat_iter() {
                        p.ptr.free();
                    }
                    n.ptr.free();
                }
            }
        }
        self.min_chunk.x = isize::MAX;
        self.min_chunk.y = isize::MAX;
        self.max_chunk.x = isize::MIN;
        self.max_chunk.y = isize::MIN;
        self.chunk_count = 0;
    }
    #[inline]
    pub fn remove(&mut self, x: u16, y: u16) -> Option<StdBox<Chunk>> {
        let xu = usize::from(x);
        let yu = usize::from(y);
        if let Some(ret) = self.chunk_array[yu][xu].take() {
            self.min_chunk.x = isize::MAX;
            self.min_chunk.y = isize::MAX;
            self.max_chunk.x = isize::MIN;
            self.max_chunk.y = isize::MIN;
            self.chunk_count -= 1;
            for (cx, cy, _) in self.chunk_array.flat_iter() {
                let xi = isize::try_from(cx).unwrap() - 256;
                let yi = isize::try_from(cy).unwrap() - 256;
                self.min_chunk.x = self.min_chunk.x.min(xi);
                self.min_chunk.y = self.min_chunk.y.min(yi);
                self.max_chunk.x = self.max_chunk.x.max(xi);
                self.max_chunk.y = self.max_chunk.y.max(yi);
            }
            Some(ret)
        } else {
            None
        }
    }
    #[inline]
    pub fn insert(&mut self, x: u16, y: u16, chunk: Chunk) {
        self.insert_box(x, y, StdBox::new(chunk));
    }
    #[inline]
    pub fn insert_box(&mut self, x: u16, y: u16, chunk: StdBox<Chunk>) {
        let xu = usize::from(x);
        let yu = usize::from(y);
        if let Some(mut n) = self.chunk_array[yu][xu] {
            for (_, _, p) in n.flat_iter() {
                p.ptr.free();
            }
            n.ptr.free();
        } else {
            self.chunk_count += 1;
        }
        self.chunk_array[yu][xu] = Some(chunk);
        let xi = isize::try_from(x).unwrap() - 256;
        let yi = isize::try_from(y).unwrap() - 256;
        self.min_chunk.x = self.min_chunk.x.min(xi);
        self.min_chunk.y = self.min_chunk.y.min(yi);
        self.max_chunk.x = self.max_chunk.x.max(xi);
        self.max_chunk.y = self.max_chunk.y.max(yi);
    }
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
