use crate::{
    ComponentBufferVTable, ComponentTrait, ComponentVTable, CutThroughWorld, LensValue, NpcParty,
    PendingPortal, StdBox, StdMap, StdPtr, StdString, StdVec, Vec2,
};
use noita_api_macros::assert_size_com;
use std::ffi::CStr;
impl ComponentTrait for WorldState {
    const NAME: &'static CStr = c"WorldStateComponent";
    #[inline]
    fn vtable() -> StdBox<ComponentVTable<Self>> {
        StdBox::from(StdPtr::new(0x00ff_3ffc))
    }
    #[inline]
    fn buffer_vtable() -> StdBox<ComponentBufferVTable<Self>> {
        StdBox::from(StdPtr::new(0x00fe_c770))
    }
    #[inline]
    fn free(&mut self) {
        todo!()
    }
}
#[derive(Debug, Default)]
#[assert_size_com(0x1c8)]
#[repr(C)]
pub struct WorldState {
    pub is_initialized: bool,
    field2_0x49: u8,
    field3_0x4a: u8,
    field4_0x4b: u8,
    field5_0x4c: u8,
    field6_0x4d: u8,
    field7_0x4e: u8,
    field8_0x4f: u8,
    pub time_total: f32,
    pub time_dt: f32,
    pub day_count: isize,
    field12_0x5c: u8,
    field13_0x5d: u8,
    field14_0x5e: u8,
    field15_0x5f: u8,
    pub rain_target: f32,
    field17_0x64: u8,
    field18_0x65: u8,
    field19_0x66: u8,
    field20_0x67: u8,
    pub fog_target: f32,
    pub intro_weather: bool,
    field23_0x6d: u8,
    field24_0x6e: u8,
    field25_0x6f: u8,
    field26_0x70: u8,
    field27_0x71: u8,
    field28_0x72: u8,
    field29_0x73: u8,
    pub wind_speed: f32,
    pub wind_speed_sin_t: f32,
    pub wind_speed_sin: f32,
    pub clouds_01_target: f32,
    pub clouds_02_target: f32,
    pub gradient_sky_alpha_target: f32,
    pub sky_sunset_alpha_target: f32,
    pub lightning_count: isize,
    pub player_spawn_location: Vec2<f32>,
    pub lua_globals: StdMap<StdString, StdString>,
    pub pending_portals: StdVec<PendingPortal>,
    pub next_portal_id: usize,
    pub apparitions_per_level: StdVec<isize>,
    pub npc_parties: StdVec<NpcParty>,
    pub session_stat_file: StdString,
    pub orbs_found_thisrun: StdVec<isize>,
    pub flags: StdVec<StdString>,
    pub changed_materials: StdVec<StdString>,
    pub player_polymorph_count: isize,
    pub player_polymorph_random_count: isize,
    pub player_did_infinite_spell_count: isize,
    pub player_did_damage_over_1milj: isize,
    pub player_living_with_minus_hp: isize,
    pub global_genome_relations_modifier: f32,
    pub mods_have_been_active_during_this_run: bool,
    pub twitch_has_been_active_during_this_run: bool,
    field56_0x122: u8,
    field57_0x123: u8,
    pub next_cut_through_world_id: usize,
    pub cuts_through_world: StdVec<CutThroughWorld>,
    pub gore_multiplier: LensValue<isize>,
    pub trick_kill_gold_multiplier: LensValue<isize>,
    pub damage_flash_multiplier: LensValue<f32>,
    pub open_fog_of_war_everywhere: LensValue<bool>,
    pub consume_actions: LensValue<bool>,
    pub perk_infinite_spells: bool,
    pub perk_trick_kills_blood_money: bool,
    field67_0x16a: u8,
    field68_0x16b: u8,
    pub perk_hp_drop_chance: isize,
    pub perk_gold_is_forever: bool,
    pub perk_rats_player_friendly: bool,
    pub everything_to_gold: bool,
    field73_0x173: u8,
    pub material_everything_to_gold: StdString,
    pub material_everything_to_gold_static: StdString,
    pub infinite_gold_happening: bool,
    pub ending_happiness_happening: bool,
    field78_0x1a6: u8,
    field79_0x1a7: u8,
    pub ending_happiness_frames: isize,
    pub ending_happiness: bool,
    field82_0x1ad: u8,
    field83_0x1ae: u8,
    field84_0x1af: u8,
    pub m_flash_alpha: f32,
    pub debug_loaded_from_autosave: isize,
    pub debug_loaded_from_old_version: isize,
    pub rain_target_extra: f32,
    pub fog_target_extra: f32,
    pub perk_rats_player_friendly_prev: bool,
    field91_0x1c5: u8,
    field92_0x1c6: u8,
    field93_0x1c7: u8,
}
