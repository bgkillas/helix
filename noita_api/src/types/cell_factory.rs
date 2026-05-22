use crate::{CellData, ConfigExplosion, StdBox, StdMap, StdPtr, StdString, StdVec};
use nxml_rs::{ElementRef, NxmlError};
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
impl CellFactory {
    pub(crate) fn global() -> StdBox<Self> {
        static GLOBAL: std::sync::LazyLock<StdBox<CellFactory>> =
            std::sync::LazyLock::new(|| StdBox::new(CellFactory::default()));
        *GLOBAL
    }
    #[inline]
    pub fn generate_cell_data(&mut self, s: &str) -> Result<(), NxmlError> {
        self.generate_cell_data_nxml(nxml_rs::parse(s)?);
        Ok(())
    }
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn generate_cell_data_nxml(&mut self, xml: ElementRef<'_>) {
        for cell_data in xml.children {
            if matches!(cell_data.name, "CellData" | "CellDataChild") {
                if let Some(parent) = cell_data.attr("_parent") {
                    let Some(mut data) = self
                        .cell_data
                        .iter()
                        .find(|c| c.name.as_str() == parent)
                        .cloned()
                    else {
                        continue;
                    };
                    data.material_type = self.cell_data.len();
                    data.id_2 = self.cell_data.len();
                    data.name = cell_data.attr("name").map(Into::into).unwrap_or(data.name);
                    data.ui_name = cell_data
                        .attr("ui_name")
                        .map(Into::into)
                        .unwrap_or(data.ui_name);
                    data.wang_color = cell_data
                        .attr("wang_color")
                        .map_or(data.wang_color, |c| c.parse().unwrap());
                    data.durability = cell_data
                        .attr("durability")
                        .map_or(data.durability, |c| c.parse().unwrap());
                    data.hp = cell_data.attr("hp").map_or(data.hp, |c| c.parse().unwrap());
                    data.cell_type = cell_data
                        .attr("cell_type")
                        .map_or(data.cell_type, |c| c.parse().unwrap());
                    data.liquid_sand = cell_data
                        .attr("liquid_sane")
                        .map_or(data.liquid_sand, |c| c.parse::<u8>().unwrap() == 1);
                    data.liquid_static = cell_data
                        .attr("liquid_static")
                        .map_or(data.liquid_static, |c| c.parse::<u8>().unwrap() == 1);
                    if let Some(graphics) = cell_data.child("Graphics") {
                        data.graphics.color = graphics
                            .attr("color")
                            .map_or(data.graphics.color, |c| c.parse().unwrap());
                    }
                    self.material_names.push(data.name.clone());
                    self.material_ids
                        .insert(data.name.clone(), data.material_type);
                    self.cell_data.push(data);
                } else {
                    let mut data = CellData::default();
                    data.material_type = self.cell_data.len();
                    data.id_2 = self.cell_data.len();
                    data.name = cell_data.attr("name").unwrap_or_default().into();
                    data.ui_name = cell_data.attr("ui_name").unwrap_or_default().into();
                    data.wang_color = cell_data
                        .attr("wang_color")
                        .unwrap_or("00000000")
                        .parse()
                        .unwrap();
                    data.durability = cell_data.attr("durability").unwrap_or("0").parse().unwrap();
                    data.hp = cell_data.attr("hp").unwrap_or("0").parse().unwrap();
                    data.cell_type = cell_data
                        .attr("cell_type")
                        .unwrap_or("none")
                        .parse()
                        .unwrap();
                    data.liquid_sand = cell_data
                        .attr("liquid_sane")
                        .unwrap_or("0")
                        .parse::<u8>()
                        .unwrap()
                        == 1;
                    data.liquid_static = cell_data
                        .attr("liquid_static")
                        .unwrap_or("0")
                        .parse::<u8>()
                        .unwrap()
                        == 1;
                    if let Some(graphics) = cell_data.child("Graphics") {
                        data.graphics.color = graphics
                            .attr("color")
                            .unwrap_or("00000000")
                            .parse()
                            .unwrap();
                    }
                    self.material_names.push(data.name.clone());
                    self.material_ids
                        .insert(data.name.clone(), data.material_type);
                    self.cell_data.push(data);
                }
            }
        }
    }
}
impl Default for CellFactory {
    #[inline]
    fn default() -> Self {
        Self {
            unknown1: 0,
            material_names: StdVec::default(),
            material_ids: StdMap::default(),
            cell_data: StdVec::default(),
            material_count: 0,
            unknown2: 0,
            reaction_lookup: ReactionLookupTable::default(),
            fast_reaction_lookup: ReactionLookupTable::default(),
            req_reactions: StdVec::default(),
            materials_by_tag: StdMap::default(),
            unknown3: StdVec::default(),
            fire_cell_data: StdBox::new(CellData::default()),
            unknown4: [0; 4],
            fire_material_id: 0,
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
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
