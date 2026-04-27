use crate::alloc::StdBox;
use crate::get_this_call;
use crate::types::death_match::DeathMatch;
use crate::types::game_global::GameGlobal;
use retour::static_detour;
use std::ffi::c_void;
pub static mut PAUSE_SIMULATE: bool = false;
#[cfg(target_os = "windows")]
static_detour! {
  static PAUSE: extern "thiscall" fn(StdBox<DeathMatch>, f32);
}
#[cfg(not(target_os = "windows"))]
static_detour! {
  static PAUSE: extern "C" fn(StdBox<DeathMatch>, f32);
}
fn pause(this: StdBox<DeathMatch>, dt: f32) {
    PAUSE.call(this, dt);
    if unsafe { !PAUSE_SIMULATE } {
        let mut game_global = GameGlobal::global();
        if game_global.is_paused() {
            let state = *game_global.pause_state;
            game_global.unpause();
            PAUSE.call(this, dt);
            *game_global.pause_state = state;
        }
    }
}
pub fn disable_pause() {
    unsafe {
        let old_pause = get_this_call!(0x006b26f0, fn(StdBox<DeathMatch>, f32));
        PAUSE.initialize(old_pause, pause).unwrap();
        PAUSE.enable().unwrap();
    }
}
pub static mut DISABLE_INVENTORY: bool = false;
#[cfg(target_os = "windows")]
static_detour! {
  static INVENTORY: extern "thiscall" fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>);
}
#[cfg(not(target_os = "windows"))]
static_detour! {
  static INVENTORY: extern "C" fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>);
}
fn inventory(this: StdBox<c_void>, entity: StdBox<c_void>, component: StdBox<c_void>) {
    if unsafe { !DISABLE_INVENTORY } {
        INVENTORY.call(this, entity, component);
    }
}
pub fn disable_inventory() {
    unsafe {
        let old_inv = get_this_call!(
            0x00b7d8d0,
            fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>)
        );
        INVENTORY.initialize(old_inv, inventory).unwrap();
        INVENTORY.enable().unwrap();
    }
}
pub static mut DISABLE_ITEM_PICKUP: bool = false;
#[cfg(target_os = "windows")]
static_detour! {
  static ITEM_PICKUP: extern "thiscall" fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>);
}
#[cfg(not(target_os = "windows"))]
static_detour! {
  static ITEM_PICKUP: extern "C" fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>);
}
fn item_pickup(this: StdBox<c_void>, entity: StdBox<c_void>, component: StdBox<c_void>) {
    //TODO check if entity is player
    if unsafe { !DISABLE_ITEM_PICKUP } {
        ITEM_PICKUP.call(this, entity, component);
    }
}
pub fn disable_item_pickup() {
    unsafe {
        let old_item = get_this_call!(
            0x00b90480,
            fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>)
        );
        ITEM_PICKUP.initialize(old_item, item_pickup).unwrap();
        ITEM_PICKUP.enable().unwrap();
    }
}
