use crate::{Cell, StdBox, Vec2};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
pub type Chunk = StdBox<ChunkArrayGeneric<Option<Cell<()>>>>;
pub type ChunkArray = StdBox<ChunkArrayGeneric<Option<StdBox<Chunk>>>>;
#[repr(C)]
#[derive(Debug)]
pub struct ChunkMap {
    pub len: usize,
    pub unknown: isize,
    pub chunk_array: StdBox<ChunkArrayGeneric<Option<StdBox<Chunk>>>>,
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
            chunk_array: StdBox::default(),
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
pub struct ChunkArrayGeneric<T> {
    pub array: [[T; 512]; 512],
}
impl ChunkMap {
    #[inline]
    pub fn flat_iter(&self) -> impl Iterator<Item = (u16, u16, StdBox<Chunk>)> {
        (self.min_chunk.y..=self.max_chunk.y).flat_map(move |yi| {
            let y = (256 + yi).strict_cast::<usize>();
            (self.min_chunk.x..=self.max_chunk.x).filter_map(move |xi| {
                let x = (256 + xi).strict_cast::<usize>();
                self.chunk_array[y][x].map(|c| (x.strict_cast::<u16>(), y.strict_cast::<u16>(), c))
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
        let xu = x.strict_cast::<usize>();
        let yu = y.strict_cast::<usize>();
        if let Some(ret) = self.chunk_array[yu][xu].take() {
            let min_x = (self.min_chunk.x + 256).strict_cast_unsigned();
            let max_x = (self.max_chunk.x + 256).strict_cast_unsigned();
            let min_y = (self.min_chunk.y + 256).strict_cast_unsigned();
            let max_y = (self.max_chunk.y + 256).strict_cast_unsigned();
            self.min_chunk.x = isize::MAX;
            self.min_chunk.y = isize::MAX;
            self.max_chunk.x = isize::MIN;
            self.max_chunk.y = isize::MIN;
            self.chunk_count -= 1;
            for cy in min_y..=max_y {
                for cx in min_x..=max_x {
                    if self.chunk_array[cy][cx].is_some() {
                        let xi = cx.strict_cast::<isize>() - 256;
                        let yi = cy.strict_cast::<isize>() - 256;
                        self.min_chunk.x = self.min_chunk.x.min(xi);
                        self.min_chunk.y = self.min_chunk.y.min(yi);
                        self.max_chunk.x = self.max_chunk.x.max(xi);
                        self.max_chunk.y = self.max_chunk.y.max(yi);
                    }
                }
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
        let xu = x.strict_cast::<usize>();
        let yu = y.strict_cast::<usize>();
        if let Some(mut n) = self.chunk_array[yu][xu] {
            for (_, _, p) in n.flat_iter() {
                p.ptr.free();
            }
            n.ptr.free();
        } else {
            self.chunk_count += 1;
        }
        self.chunk_array[yu][xu] = Some(chunk);
        let xi = x.strict_cast::<isize>() - 256;
        let yi = y.strict_cast::<isize>() - 256;
        self.min_chunk.x = self.min_chunk.x.min(xi);
        self.min_chunk.y = self.min_chunk.y.min(yi);
        self.max_chunk.x = self.max_chunk.x.max(xi);
        self.max_chunk.y = self.max_chunk.y.max(yi);
    }
}
impl<T> ChunkArrayGeneric<T> {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (u16, u16, &T)> {
        self.array.iter().enumerate().flat_map(|(y, yc)| {
            yc.iter()
                .enumerate()
                .map(move |(x, xc)| (x.strict_cast::<u16>(), y.strict_cast::<u16>(), xc))
        })
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u16, u16, &mut T)> {
        self.array.iter_mut().enumerate().flat_map(|(y, yc)| {
            yc.iter_mut()
                .enumerate()
                .map(move |(x, xc)| (x.strict_cast::<u16>(), y.strict_cast::<u16>(), xc))
        })
    }
}
impl<T> ChunkArrayGeneric<Option<T>> {
    #[inline]
    pub fn flat_iter(&self) -> impl Iterator<Item = (u16, u16, &T)> {
        self.iter()
            .filter_map(|(x, y, oc)| oc.as_ref().map(|c| (x, y, c)))
    }
    #[inline]
    pub fn flat_iter_mut(&mut self) -> impl Iterator<Item = (u16, u16, &mut T)> {
        self.iter_mut()
            .filter_map(|(x, y, oc)| oc.as_mut().map(|c| (x, y, c)))
    }
}
impl<T> Default for ChunkArrayGeneric<Option<T>> {
    #[inline]
    #[allow(clippy::large_stack_arrays)]
    fn default() -> Self {
        Self {
            array: [const { [const { None }; 512] }; 512],
        }
    }
}
impl<T> Debug for ChunkArrayGeneric<Option<T>> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.flat_iter().map(|(x, y, _)| (x, y)))
            .finish()
    }
}
impl<T> Deref for ChunkArrayGeneric<T> {
    type Target = [[T; 512]; 512];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}
impl<T> DerefMut for ChunkArrayGeneric<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}
