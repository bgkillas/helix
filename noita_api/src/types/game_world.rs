use crate::{AABB, GridWorld, PixelScene, StdBox, StdVec};
#[repr(C)]
#[derive(Debug)]
pub struct GameWorld {
    pub cam: AABB<f32>,
    unknown1: [isize; 13],
    pub grid_world: StdBox<GridWorld>,
    pub pixel_scenes: StdBox<StdVec<PixelScene>>,
    //likely more data
}
impl GameWorld {
    pub(crate) fn global() -> StdBox<Self> {
        static GLOBAL: std::sync::LazyLock<StdBox<GameWorld>> = std::sync::LazyLock::default();
        *GLOBAL
    }
}
impl Default for GameWorld {
    #[inline]
    fn default() -> Self {
        Self {
            cam: AABB::default(),
            unknown1: [0; 13],
            grid_world: GridWorld::global(),
            pixel_scenes: StdBox::default(),
        }
    }
}
