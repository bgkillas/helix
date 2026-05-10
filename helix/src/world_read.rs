use crate::world_sync::SendType;
use crate::{Context, Message};
use bevy_tangled::{ClientTrait as _, Compression, Reliability};
use noita_api::{AABB, Cell, CellType, GameGlobal, StdBox, Vec2, game_print};
pub const COLS: usize = 16;
pub const SECTIONS: usize = COLS * COLS;
pub const WIDTH: usize = 512 / COLS;
pub const AREA: usize = WIDTH * WIDTH;
impl Context {
    pub fn sync_world(&mut self) {
        let game_global = GameGlobal::global();
        let map = &game_global.m_grid_world.chunk_map;
        if !self.world_init
            || map.len == 0
            || map.min_chunk.x > map.max_chunk.x
            || game_global.frame_num.is_multiple_of(6)
        {
            return;
        }
        let aabb_cam = game_global.m_grid_world.cam;
        let aabb = AABB {
            top_left: Vec2 {
                x: (aabb_cam.top_left.x + 512 * 256).cast_unsigned(),
                y: (aabb_cam.top_left.y + 512 * 256).cast_unsigned(),
            },
            bottom_right: Vec2 {
                x: (aabb_cam.bottom_right.x + 512 * 256).cast_unsigned(),
                y: (aabb_cam.bottom_right.y + 512 * 256).cast_unsigned(),
            },
        };
        let inner_aabb = AABB {
            top_left: Vec2 {
                x: aabb.top_left.x + (aabb.bottom_right.x - aabb.top_left.x) / 4,
                y: aabb.top_left.y + (aabb.bottom_right.y - aabb.top_left.y) / 4,
            },
            bottom_right: Vec2 {
                x: aabb.bottom_right.x - (aabb.bottom_right.x - aabb.top_left.x) / 4,
                y: aabb.bottom_right.y - (aabb.bottom_right.y - aabb.top_left.y) / 4,
            },
        };
        let mut chunks: Vec<Chunk> = Vec::with_capacity(map.chunk_count * SECTIONS);
        for y in aabb.top_left.y / 512..=aabb.bottom_right.y / 512 {
            for x in aabb.top_left.x / 512..=aabb.bottom_right.x / 512 {
                if let Some(chunk) = map.chunk_array[y][x] {
                    for (section, priority, chunk_section) in
                        get_sections(aabb, inner_aabb, x, y, chunk.data.as_ref())
                    {
                        let mut pixel_run = PixelRunBuilder::default();
                        pixel_run.extend(chunk_section);
                        let pos = ChunkPos { x, y, section };
                        self.seen_chunks.insert(pos, true);
                        chunks.push(Chunk {
                            pixel_run: pixel_run.build(),
                            pos,
                            priority,
                        });
                    }
                }
            }
        }
        let mut del = Vec::with_capacity(self.seen_chunks.len());
        self.seen_chunks.retain(|pos, is_seen| {
            if *is_seen {
                true
            } else {
                del.push(*pos);
                false
            }
        });
        for is_seen in self.seen_chunks.values_mut() {
            *is_seen = false;
        }
        if !del.is_empty() {
            if self.net.is_host() {
                self.world_sync
                    .as_mut()
                    .unwrap()
                    .del_world(self.net.my_id(), del);
            } else if let Err(e) = self.net.send(
                self.net.host_id(),
                &Message::RemoveChunks(del),
                Reliability::Reliable,
                Compression::Compressed,
            ) {
                game_print!("{e:?}");
            }
        }
        if !chunks.is_empty() {
            if self.net.is_host() {
                self.world_sync.as_mut().unwrap().push_world(
                    SendType::World(self.world_write),
                    self.net.my_id(),
                    chunks,
                );
            } else if let Err(e) = self.net.send(
                self.net.host_id(),
                &Message::Chunks(chunks),
                Reliability::Reliable,
                Compression::Compressed,
            ) {
                game_print!("{e:?}");
            }
        }
    }
}
pub fn get_sections(
    aabb: AABB<usize>,
    inner_aabb: AABB<usize>,
    x: usize,
    y: usize,
    arr: &[[Option<StdBox<Cell>>; 512]; 512],
) -> impl Iterator<Item = (u8, Priority, impl Iterator<Item = Option<StdBox<Cell>>>)> + '_ {
    (0..SECTIONS).filter_map(move |s| {
        section_in(aabb, inner_aabb, x, y, s)
            .map(|p| (u8::try_from(s).unwrap(), p, get_section(s, arr)))
    })
}
fn section_in(
    aabb: AABB<usize>,
    inner_aabb: AABB<usize>,
    x: usize,
    y: usize,
    section: usize,
) -> Option<Priority> {
    let sx = (section % COLS) * WIDTH + x * 512;
    let sy = (section / COLS) * WIDTH + y * 512;
    let rect = AABB {
        top_left: Vec2 { x: sx, y: sy },
        bottom_right: Vec2 {
            x: sx + WIDTH,
            y: sy + WIDTH,
        },
    };
    if aabb.intersects(rect) {
        if aabb.contains(rect) {
            if inner_aabb.intersects(rect) {
                Some(Priority::High)
            } else {
                Some(Priority::Medium)
            }
        } else {
            Some(Priority::Low)
        }
    } else {
        None
    }
}
pub fn get_section(
    section: usize,
    arr: &[[Option<StdBox<Cell>>; 512]; 512],
) -> impl Iterator<Item = Option<StdBox<Cell>>> + '_ {
    let sx = (section % COLS) * WIDTH;
    let sy = (section / COLS) * WIDTH;
    arr[sy..sy + WIDTH]
        .iter()
        .flat_map(move |arr_y| &arr_y[sx..sx + WIDTH])
        .copied()
}
pub fn get_section_mut_enumerate(
    section: usize,
    arr: &mut [[Option<StdBox<Cell>>; 512]; 512],
) -> impl Iterator<Item = (usize, usize, &mut Option<StdBox<Cell>>)> + '_ {
    let sx = (section % COLS) * WIDTH;
    let sy = (section / COLS) * WIDTH;
    arr[sy..sy + WIDTH]
        .iter_mut()
        .enumerate()
        .flat_map(move |(y, arr_y)| {
            arr_y[sx..sx + WIDTH]
                .iter_mut()
                .enumerate()
                .map(move |(x, p)| (sx + x, sy + y, p))
        })
}
#[derive(bitcode::Encode, bitcode::Decode, Hash, Eq, PartialEq, Clone, Copy)]
pub struct ChunkPos {
    pub x: usize,
    pub y: usize,
    pub section: u8,
}
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Chunk {
    pub pixel_run: PixelRun,
    pub pos: ChunkPos,
    pub priority: Priority,
}
#[derive(bitcode::Encode, bitcode::Decode, Clone)]
pub struct PixelRun {
    pub vec: Vec<(u16, Pixel)>,
}
impl PixelRun {
    pub fn iter(&self) -> PixelRunIter<'_> {
        PixelRunIter {
            vec: &self.vec,
            current: self.vec[0].0,
            pixel: self.vec[0].1,
        }
    }
}
pub struct PixelRunIter<'a> {
    vec: &'a [(u16, Pixel)],
    current: u16,
    pixel: Pixel,
}
impl Iterator for PixelRunIter<'_> {
    type Item = Pixel;
    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.is_empty() {
            None
        } else if self.current == 0 {
            self.vec = &self.vec[1..];
            self.current = self.vec[0].0;
            self.pixel = self.vec[0].1;
            self.current -= 1;
            Some(self.pixel)
        } else {
            self.current -= 1;
            Some(self.pixel)
        }
    }
}
pub struct PixelRunBuilder {
    vec: Vec<(u16, Pixel)>,
    current: Pixel,
    len: u16,
}
impl Extend<Option<StdBox<Cell>>> for PixelRunBuilder {
    fn extend<T: IntoIterator<Item = Option<StdBox<Cell>>>>(&mut self, iter: T) {
        for p in iter {
            self.push(p.map_or(Pixel::default(), |c| {
                if matches!(c.material.cell_type, CellType::Solid) {
                    Pixel::MAX
                } else {
                    Pixel::from(c.material.material_type)
                }
            }));
        }
    }
}
impl PixelRunBuilder {
    fn build(mut self) -> PixelRun {
        self.write();
        PixelRun { vec: self.vec }
    }
    fn write(&mut self) {
        if self.len != 0 {
            self.vec.push((self.len, self.current));
        }
    }
    fn push(&mut self, pixel: Pixel) {
        if self.current == pixel {
            self.len += 1;
        } else {
            self.write();
            self.current = pixel;
            self.len = 1;
        }
    }
}
impl Default for PixelRunBuilder {
    #[inline]
    fn default() -> Self {
        Self {
            vec: Vec::with_capacity(AREA),
            current: Pixel::default(),
            len: 0,
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq, bitcode::Encode, bitcode::Decode)]
pub struct Pixel {
    pub id: u16,
}
impl From<usize> for Pixel {
    fn from(value: usize) -> Self {
        Self {
            id: u16::try_from(value).unwrap(),
        }
    }
}
impl Pixel {
    pub const MAX: Pixel = Pixel { id: u16::MAX };
}
#[derive(bitcode::Encode, bitcode::Decode, PartialOrd, PartialEq)]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}
