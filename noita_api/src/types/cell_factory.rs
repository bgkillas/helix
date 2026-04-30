use crate::{CellData, ConfigExplosion, StdBox, StdMap, StdPtr, StdString, StdVec};
use std::ffi::c_void;
#[repr(C)]
#[derive(Debug)]
#[allow(clippy::type_complexity)]
pub struct CellFactory {
    unknown1: isize,
    pub material_names: StdVec<StdString>,
    pub material_ids: StdMap<StdString, usize>,
    pub cell_data: StdVec<CellData>,
    pub material_count: usize,
    unknown2: isize,
    pub reaction_lookup: ReactionLookupTable,
    pub fast_reaction_lookup: ReactionLookupTable,
    pub req_reactions: StdVec<CellReactionBuf>,
    pub materials_by_tag: StdMap<StdString, StdVec<StdBox<CellData>>>,
    unknown3: StdVec<Option<StdBox<StdVec<Option<StdPtr<c_void>>>>>>,
    pub fire_cell_data: StdBox<CellData>,
    unknown4: [usize; 4],
    pub fire_material_id: usize,
}
#[repr(C)]
#[derive(Debug)]
pub struct ReactionLookupTable {
    pub width: usize,
    pub height: usize,
    pub len: usize,
    unknown: [usize; 5],
    pub storage: Option<StdBox<CellReactionBuf>>,
    unk_len: usize,
    unknown3: usize,
}
#[repr(C)]
#[derive(Debug)]
pub struct CellReactionBuf {
    pub base: *mut CellReaction,
    pub len: usize,
    pub cap: usize,
}
#[repr(C)]
#[derive(Debug)]
pub struct CellReaction {
    pub fast_reaction: bool,
    padding: [u8; 3],
    pub probability_times_100: usize,
    pub input_cell1: isize,
    pub input_cell2: isize,
    pub output_cell1: isize,
    pub output_cell2: isize,
    pub has_input_cell3: bool,
    padding2: [u8; 3],
    pub input_cell3: isize,
    pub output_cell3: isize,
    pub cosmetic_particle: isize,
    pub req_lifetime: isize,
    pub blob_radius1: u8,
    pub blob_radius2: u8,
    pub blob_restrict_to_input_material1: bool,
    pub blob_restrict_to_input_material2: bool,
    pub destroy_horizontally_lonely_pixels: bool,
    pub convert_all: bool,
    padding3: [u8; 2],
    pub entity_file_idx: usize,
    pub direction: ReactionDir,
    pub explosion_config: Option<StdBox<ConfigExplosion>>,
    pub audio_fx_volume_1: f32,
}
#[derive(Debug)]
#[repr(isize)]
pub enum ReactionDir {
    None = -1,
    Top,
    Bottom,
    Left,
    Right,
}
