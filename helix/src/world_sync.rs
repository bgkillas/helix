use crate::Message;
use crate::world::{Chunk, PixelRun, Priority, SECTIONS};
use crate::world_write::ChunkWrite;
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
impl WorldSync {
    pub fn push_world(&mut self, client: &ClientTypeRef, peer: PeerId, mut chunks: Vec<Chunk>) {
        let mut send_back = Vec::with_capacity(chunks.len());
        for chunk in chunks.drain(..) {
            let section = usize::from(chunk.section);
            if let Some(prev) = &mut self.chunks[chunk.y][chunk.x][section] {
                if chunk.priority > prev.priority || prev.peer == peer {
                    prev.pixel_run = chunk.pixel_run;
                    prev.priority = chunk.priority;
                    prev.peer = peer;
                } else {
                    send_back.push(ChunkWrite {
                        pixel_run: prev.pixel_run.clone(),
                    });
                }
            } else {
                self.chunks[chunk.y][chunk.x][section] = Some(Box::new(ChunkVal {
                    pixel_run: chunk.pixel_run,
                    priority: chunk.priority,
                    peer,
                }));
            }
        }
        if !send_back.is_empty()
            && let Err(e) = client.send(
                peer,
                &Message::ChunksWrite(send_back),
                Reliability::Reliable,
                Compression::Compressed,
            )
        {
            game_print!("{e:?}");
        }
    }
}
