use crate::*;
#[repr(C)]
#[derive(Debug)]
pub struct GameWorld {
    pub cam: AABB,
    unknown1: [isize; 13],
    pub grid_world: StdBox<GridWorld>,
    pub pixel_scenes: StdBox<StdVec<PixelScene>>,
    //likely more data
}
