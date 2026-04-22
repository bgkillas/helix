use crate::alloc::StdBox;
use crate::types::aabb::AABB;
use crate::types::grid_world::GridWorld;
use crate::types::pixel_scene::PixelScene;
use crate::types::vec::StdVec;
#[repr(C)]
#[derive(Debug)]
pub struct GameWorld {
    pub cam: AABB,
    unknown1: [isize; 13],
    pub grid_world: StdBox<GridWorld>,
    pub pixel_scenes: StdBox<StdVec<PixelScene>>,
    //likely more data
}
