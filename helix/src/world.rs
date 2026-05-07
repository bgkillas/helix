use crate::{Context, Message};
use bevy_tangled::{ClientTrait as _, Compression, Reliability};
use noita_api::{AABB, Cell, GameGlobal, StdBox, Vec2, game_print};
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
        for y in (map.min_chunk.y + 256).cast_unsigned()..=(map.max_chunk.y + 256).cast_unsigned() {
            for x in
                (map.min_chunk.x + 256).cast_unsigned()..=(map.max_chunk.x + 256).cast_unsigned()
            {
                if let Some(chunk) = map.chunk_array[y][x] {
                    for (section, priority, chunk_section) in
                        get_sections(aabb, inner_aabb, x, y, chunk.data.as_ref())
                    {
                        let mut pixel_run = PixelRunBuilder::default();
                        pixel_run.extend(chunk_section);
                        chunks.push(Chunk {
                            pixel_run: pixel_run.build(),
                            x,
                            y,
                            section,
                            priority,
                        });
                    }
                }
            }
        }
        if let Err(e) = self.net.send(
            self.net.host_id(),
            &Message::Chunks(chunks),
            Reliability::Reliable,
            Compression::Compressed,
        ) {
            game_print!("{e:?}");
        }
    }
}
fn get_sections(
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
fn get_section(
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
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Chunk {
    pub pixel_run: PixelRun,
    pub x: usize,
    pub y: usize,
    pub section: u8,
    pub priority: Priority,
}
#[derive(bitcode::Encode, bitcode::Decode, Clone)]
pub struct PixelRun {
    pub vec: Vec<(u16, Pixel)>,
}
pub struct PixelRunBuilder {
    vec: Vec<(u16, Pixel)>,
    current: Pixel,
    len: u16,
}
impl Extend<Option<StdBox<Cell>>> for PixelRunBuilder {
    fn extend<T: IntoIterator<Item = Option<StdBox<Cell>>>>(&mut self, iter: T) {
        for p in iter {
            self.push(p.map_or(Pixel::default(), |c| Pixel::from(c.material.material_type)));
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
#[derive(bitcode::Encode, bitcode::Decode, PartialOrd, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}
