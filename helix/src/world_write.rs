use crate::world::PixelRun;
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct ChunkWrite {
    pub pixel_run: PixelRun,
}
pub fn write_chunks(chunks: Vec<ChunkWrite>) {
    _ = chunks;
    //TODO
}
