use crate::alloc::StdBox;
use crate::types::config::{ConfigExplosion, ConfigGridCosmeticParticle};
use crate::types::string::StdString;
use crate::types::textures::TextureInfo;
use crate::types::vec::{StdVec, Vec2i};
use std::ffi::c_void;
#[repr(C)]
#[derive(Debug)]
pub struct ChunkMap {
    pub len: usize,
    pub unknown: isize,
    pub chunk_array: StdBox<[Option<StdBox<Chunk>>; 512 * 512]>,
    pub chunk_count: usize,
    pub min_chunk: Vec2i,
    pub max_chunk: Vec2i,
    pub min_pixel: Vec2i,
    pub max_pixel: Vec2i,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chunk {
    pub data: StdBox<[Option<StdBox<Cell>>; 512 * 512]>,
}
#[repr(C)]
#[derive(Debug)]
pub struct Cell {
    pub vtable: StdBox<CellVTable>,
    pub hp: isize,
    unknown1: [isize; 2],
    pub is_burning: bool,
    pub temperature_of_fire: u8,
    unknown2: [u8; 2],
    pub material: StdBox<CellData>,
}
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct CellVTable {}
#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum CellType {
    #[default]
    None = 0,
    Liquid = 1,
    Gas = 2,
    Solid = 3,
    Fire = 4,
}
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct CellGraphics {
    pub texture_file: StdString,
    pub color: Color,
    pub fire_colors_index: u32,
    pub randomize_colors: bool,
    pub normal_mapped: bool,
    pub is_grass: bool,
    pub is_grass_hashed: bool,
    pub pixel_info: *const c_void,
    unknown: [isize; 5],
    pub texture_info: Option<StdBox<TextureInfo>>,
}
#[repr(C)]
#[derive(Debug)]
pub struct CellData {
    pub name: StdString,
    pub ui_name: StdString,
    pub material_type: isize,
    pub id_2: isize,
    pub cell_type: CellType,
    pub platform_type: isize,
    pub wang_color: Color,
    pub gfx_glow: isize,
    pub gfx_glow_color: Color,
    pub graphics: CellGraphics,
    pub cell_holes_in_texture: bool,
    pub stainable: bool,
    pub burnable: bool,
    pub on_fire: bool,
    pub fire_hp: isize,
    pub autoignition_temperature: isize,
    pub minus_100_autoignition_temp: isize,
    pub temperature_of_fire: isize,
    pub generates_smoke: isize,
    pub generates_flames: isize,
    pub requires_oxygen: bool,
    padding1: [u8; 3],
    pub on_fire_convert_to_material: StdString,
    pub on_fire_convert_to_material_id: isize,
    pub on_fire_flame_material: StdString,
    pub on_fire_flame_material_id: isize,
    pub on_fire_smoke_material: StdString,
    pub on_fire_smoke_material_id: isize,
    pub explosion_config: Option<StdBox<ConfigExplosion>>,
    pub durability: isize,
    pub crackability: isize,
    pub electrical_conductivity: bool,
    pub slippery: bool,
    padding2: [u8; 2],
    pub stickyness: f32,
    pub cold_freezes_to_material: StdString,
    pub warmth_melts_to_material: StdString,
    pub warmth_melts_to_material_id: isize,
    pub cold_freezes_to_material_id: isize,
    pub cold_freezes_chance_rev: i16,
    pub warmth_melts_chance_rev: i16,
    pub cold_freezes_to_dont_do_reverse_reaction: bool,
    padding3: [u8; 3],
    pub lifetime: isize,
    pub hp: isize,
    pub density: f32,
    pub liquid_sand: bool,
    pub liquid_slime: bool,
    pub liquid_static: bool,
    pub liquid_stains_self: bool,
    pub liquid_sticks_to_ceiling: isize,
    pub liquid_gravity: f32,
    pub liquid_viscosity: isize,
    pub liquid_stains: isize,
    pub liquid_stains_custom_color: Color,
    pub liquid_sprite_stain_shaken_drop_chance: f32,
    pub liquid_sprite_stain_ignited_drop_chance: f32,
    pub liquid_sprite_stains_check_offset: i8,
    padding4: [u8; 3],
    pub liquid_sprite_stains_status_threshold: f32,
    pub liquid_damping: f32,
    pub liquid_flow_speed: f32,
    pub liquid_sand_never_box2d: bool,
    padding5: [u8; 3],
    pub gas_speed: i8,
    pub gas_upwards_speed: i8,
    pub gas_horizontal_speed: i8,
    pub gas_downwards_speed: i8,
    pub solid_friction: f32,
    pub solid_restitution: f32,
    pub solid_gravity_scale: f32,
    pub solid_static_type: isize,
    pub solid_on_collision_splash_power: f32,
    pub solid_on_collision_explode: bool,
    pub solid_on_sleep_convert: bool,
    pub solid_on_collision_convert: bool,
    pub solid_on_break_explode: bool,
    pub solid_go_through_sand: bool,
    pub solid_collide_with_self: bool,
    padding6: [u8; 2],
    pub solid_on_collision_material: StdString,
    pub solid_on_collision_material_id: isize,
    pub solid_break_to_type: StdString,
    pub solid_break_to_type_id: isize,
    pub convert_to_box2d_material: StdString,
    pub convert_to_box2d_material_id: isize,
    pub vegetation_full_lifetime_growth: isize,
    pub vegetation_sprite: StdString,
    pub vegetation_random_flip_x_scale: bool,
    padding7: [u8; 3],
    pub max_reaction_probability: u32,
    pub max_fast_reaction_probability: u32,
    unknown11: isize,
    pub wang_noise_percent: f32,
    pub wang_curvature: f32,
    pub wang_noise_type: isize,
    pub tags: StdVec<StdString>,
    pub danger_fire: bool,
    pub danger_radioactive: bool,
    pub danger_poison: bool,
    pub danger_water: bool,
    pub stain_effects: StdVec<StdString>,
    pub ingestion_effects: StdVec<StdString>,
    pub always_ignites_damagemodel: bool,
    pub ignore_self_reaction_warning: bool,
    padding8: [u8; 2],
    pub audio_physics_material_event_idx: isize,
    pub audio_physics_material_wall_idx: isize,
    pub audio_physics_material_solid_idx: isize,
    pub audio_size_multiplier: f32,
    pub audio_is_soft: bool,
    padding9: [u8; 3],
    pub audio_material_audio_type: isize,
    pub audio_material_breakaudio_type: isize,
    pub show_in_creative_mode: bool,
    pub is_just_particle_fx: bool,
    padding10: [u8; 2],
    pub grid_cosmetic_particle_config: Option<StdBox<ConfigGridCosmeticParticle>>,
}
