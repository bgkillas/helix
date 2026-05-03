use crate::{StdBox, StdMap, StdString, StdVec, TranslationManagerVTable};
#[derive(Debug)]
#[repr(C)]
pub struct TranslationManager {
    pub vftable: StdBox<TranslationManagerVTable>,
    pub unknown_strings: StdVec<StdString>,
    pub languages: StdVec<Language>,
    pub key_to_index: StdMap<StdString, usize>,
    pub extra_lang_files: StdVec<StdString>,
    pub current_lang_idx: usize,
    pub unknown: usize,
    pub unknown_float: f32,
    pub unknown_primitive_vec: StdVec<usize>,
    pub unknown_map: StdMap<StdString, StdString>,
}

#[derive(Debug)]
#[repr(C)]
pub struct Language {
    pub id: StdString,
    pub name: StdString,
    pub font_default: StdString,
    pub font_inventory_title: StdString,
    pub font_important_message_title: StdString,
    pub font_world_space_message: StdString,
    pub fonts_utf8: bool,
    pub fonts_pixel_font: bool,
    padding1: [u8; 2],
    pub fonts_dpi: f32,
    pub ui_wand_info_offset1: f32,
    pub ui_wand_info_offset2: f32,
    pub ui_action_info_offset2: f32,
    pub ui_configurecontrols_offset2: f32,
    pub strings: StdVec<StdString>,
}
