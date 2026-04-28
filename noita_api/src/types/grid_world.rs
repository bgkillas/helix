use crate::*;
#[repr(C)]
#[derive(Debug)]
pub struct GridWorld {
    pub vtable: StdBox<GridWorldVTable>,
    pub rng: isize,
    pub unk: [isize; 270],
    pub biome_modifiers: BiomeModifiers,
    pub unk2: [isize; 15],
    pub cam_pos: Vec2i,
    pub cam_dimen: Vec2i,
    pub unknown: [isize; 6],
    pub unk_cam: IAABB,
    pub unk2_cam: IAABB,
    pub unkown3: isize,
    pub cam: IAABB,
    pub unkown2: isize,
    pub unk_counter: isize,
    pub world_update_count: isize,
    pub chunk_map: ChunkMap,
    pub unknown2: [isize; 40],
    pub m_thread_impl: StdBox<GridWorldThreadImpl>,
}
#[repr(C)]
#[derive(Debug)]
pub struct BiomeModifiers {
    pub vftable: StdBox<BiomeModifiersVFTable>,
    pub unk: [usize; 6],
}
#[repr(C)]
#[derive(Debug)]
pub struct GridWorldThreadImpl {
    pub chunk_update_count: usize,
    pub updated_grid_worlds: StdVec<StdBox<GridWorldThreaded>>,
    pub world_update_params_count: usize,
    pub world_update_params: StdVec<WorldUpdateParams>,
}
#[repr(C)]
#[derive(Debug)]
pub struct GridWorldThreaded {
    pub grid_world_threaded_vtable: StdBox<GridWorldThreadedVTable>,
    pub unknown: [isize; 287],
    pub update_region: IAABB,
}
#[repr(C)]
#[derive(Debug)]
pub struct WorldUpdateParams {
    pub update_region: IAABB,
    unknown: isize,
    grid_world_threaded: &'static GridWorldThreaded,
}
