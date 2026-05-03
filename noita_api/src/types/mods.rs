use crate::lua::LuaState;
use crate::{ModVTable, StdBox, StdString, StdVec};
#[derive(Debug)]
#[repr(C)]
pub struct ModListEntry {
    pub name: StdString,
    pub steam_id: usize,
    unk1: [u8; 4],
    pub enabled: bool,
    unk1_bool: bool,
    unk2_bool: bool,
    unk2: u8,
    unk3: [u8; 4],
}
#[derive(Debug)]
#[repr(C)]
pub struct Mods {
    pub names: StdVec<ModListEntry>,
    pub list: StdVec<Mod>,
}
#[derive(Debug)]
#[repr(C)]
pub struct Mod {
    unk: [usize; 14],
    pub lua_data: StdBox<ModLua>,
    pub vtable: StdBox<ModVTable>,
    unk2: [usize; 8],
}
#[derive(Debug)]
#[repr(C)]
pub struct ModLua {
    unk: [usize; 14],
    pub lua_state: LuaState,
}
