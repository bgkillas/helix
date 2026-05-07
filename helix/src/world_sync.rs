use crate::Message;
use crate::world::{Chunk, ChunkPos, PixelRun, Priority, SECTIONS};
use crate::world_write::{ChunkWrite, WorldWrite};
use bevy_tangled::{ClientTrait as _, ClientTypeRef, Compression, PeerId, Reliability};
use noita_api::game_print;
pub struct ChunkVal {
    pixel_run: PixelRun,
    priority: Priority,
    peer: PeerId,
}
#[allow(clippy::type_complexity)]
pub struct WorldSync {
    chunks: Box<[[[Option<Box<ChunkVal>>; SECTIONS]; 512]; 512]>,
}
impl Default for WorldSync {
    fn default() -> Self {
        Self {
            chunks: unsafe { Box::new_zeroed().assume_init() },
        }
    }
}
pub enum SendType<'a> {
    Net(ClientTypeRef<'a>),
    World(WorldWrite),
}
impl WorldSync {
    pub fn push_world(&mut self, send_type: SendType, peer: PeerId, mut chunks: Vec<Chunk>) {
        let mut send_back = Vec::with_capacity(chunks.len());
        for chunk in chunks.drain(..) {
            let section = usize::from(chunk.pos.section);
            if let Some(prev) = &mut self.chunks[chunk.pos.y][chunk.pos.x][section] {
                if prev.peer == peer {
                    prev.pixel_run = chunk.pixel_run;
                    prev.priority = chunk.priority;
                } else if chunk.priority > prev.priority {
                    prev.pixel_run = chunk.pixel_run;
                    prev.priority = chunk.priority;
                    prev.peer = peer;
                } else {
                    send_back.push(ChunkWrite {
                        pixel_run: prev.pixel_run.clone(),
                        pos: chunk.pos,
                    });
                }
            } else {
                self.chunks[chunk.pos.y][chunk.pos.x][section] = Some(Box::new(ChunkVal {
                    pixel_run: chunk.pixel_run,
                    priority: chunk.priority,
                    peer,
                }));
            }
        }
        if !send_back.is_empty() {
            match send_type {
                SendType::Net(client) => {
                    if let Err(e) = client.send(
                        peer,
                        &Message::ChunksWrite(send_back),
                        Reliability::Reliable,
                        Compression::Compressed,
                    ) {
                        game_print!("{e:?}");
                    }
                }
                SendType::World(world_write) => world_write.write_chunks(&send_back),
            }
        }
    }
    pub fn del_world(&mut self, src: PeerId, chunks: Vec<ChunkPos>) {
        for pos in chunks {
            let section = usize::from(pos.section);
            if let Some(prev) = &mut self.chunks[pos.y][pos.x][section]
                && prev.peer == src
            {
                prev.priority = Priority::None;
            }
        }
    }
}
