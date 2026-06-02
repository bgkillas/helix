use crate::{
    AABB, BiomeModifiersVTable, ChunkMap, GridWorldThreadedVTable, GridWorldVTable, StdBox, StdVec,
    Vec2,
};
use std::ptr;
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
    pub m_thread_impl: *mut GridWorldThreadImpl,
}
impl GridWorld {
    pub(crate) fn global() -> StdBox<Self> {
        static GLOBAL: std::sync::LazyLock<StdBox<GridWorld>> = std::sync::LazyLock::default();
        *GLOBAL
    }
}
impl Default for GridWorld {
    #[inline]
    fn default() -> Self {
        Self {
            vtable: StdBox::new(GridWorldVTable {}),
            rng: 0,
            unk: [0; 270],
            biome_modifiers: BiomeModifiers {
                vftable: StdBox::new(BiomeModifiersVTable {}),
                unk: [0; 6],
            },
            unk2: [0; 15],
            cam_pos: Vec2::default(),
            cam_dimen: Vec2::default(),
            unknown: [0; 6],
            unk_cam: AABB::default(),
            unk2_cam: AABB::default(),
            unkown3: 0,
            cam: AABB::default(),
            unkown2: 0,
            unk_counter: 0,
            world_update_count: 0,
            chunk_map: ChunkMap::default(),
            unknown2: [0; 40],
            m_thread_impl: ptr::null_mut(),
        }
    }
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
