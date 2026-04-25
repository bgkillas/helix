use crate::alloc::StdBox;
use crate::types::cell_factory::CellFactory;
use crate::types::game_world::GameWorld;
use crate::types::grid_world::GridWorld;
use crate::types::textures::Textures;
use noita_api_macros::assert_size;
use std::ffi::c_void;
#[repr(C)]
#[assert_size(0x1a0)]
#[derive(Debug)]
pub struct GameGlobal {
    pub frame_num: usize,
    pub frame_num_start: usize,
    unknown1: isize,
    pub m_game_world: StdBox<GameWorld>,
    pub m_grid_world: StdBox<GridWorld>,
    pub m_textures: StdBox<Textures>,
    pub m_cell_factory: StdBox<CellFactory>,
    unknown2: isize,
    unknown3: [isize; 4],
    pub game_print: Option<StdBox<c_void>>,
    unknown5: [isize; 5],
    pub pause_state: StdBox<isize>,
    unk: [isize; 5],
    pub inventory_open: usize,
    unk4: [isize; 79],
}
impl GameGlobal {
    pub fn unpause(&mut self) {
        *self.pause_state = 0;
    }
    pub fn pause(&mut self) {
        *self.pause_state = 4;
    }
    pub fn is_paused(&self) -> bool {
        *self.pause_state != 0
    }
}
