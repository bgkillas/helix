use crate::{
    AABB, BiomeModifiersVTable, ChunkMap, GridWorldThreadedVTable, GridWorldVTable, StdBox, StdVec,
    Vec2,
};
#[repr(C)]
#[derive(Debug)]
pub struct GridWorld {
    pub vtable: StdBox<GridWorldVTable>,
    pub rng: isize,
    pub unk: [isize; 270],
    pub biome_modifiers: BiomeModifiers,
    pub unk2: [isize; 15],
    pub cam_pos: Vec2<isize>,
    pub cam_dimen: Vec2<isize>,
    pub unknown: [isize; 6],
    pub unk_cam: AABB<isize>,
    pub unk2_cam: AABB<isize>,
    pub unkown3: isize,
    pub cam: AABB<isize>,
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
    pub vftable: StdBox<BiomeModifiersVTable>,
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
    pub update_region: AABB<isize>,
}
#[repr(C)]
#[derive(Debug)]
pub struct WorldUpdateParams {
    pub update_region: AABB<isize>,
    unknown: isize,
    grid_world_threaded: StdBox<GridWorldThreaded>,
}
