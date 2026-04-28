use crate::*;
use retour::static_detour;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
pub static PAUSE_SIMULATE: AtomicBool = AtomicBool::new(true);
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
    if PAUSE_SIMULATE.load(Ordering::Relaxed) {
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
pub static DISABLE_INVENTORY: AtomicBool = AtomicBool::new(false);
#[cfg(target_os = "windows")]
static_detour! {
  static INVENTORY: extern "thiscall" fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>);
}
#[cfg(not(target_os = "windows"))]
static_detour! {
  static INVENTORY: extern "C" fn(StdBox<c_void>, StdBox<c_void>, StdBox<c_void>);
}
fn inventory(this: StdBox<c_void>, entity: StdBox<c_void>, component: StdBox<c_void>) {
    if !DISABLE_INVENTORY.load(Ordering::Relaxed) {
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
pub static DISABLE_ITEM_PICKUP: AtomicBool = AtomicBool::new(false);
#[cfg(target_os = "windows")]
static_detour! {
  static ITEM_PICKUP: extern "thiscall" fn(StdBox<c_void>, StdBox<Entity>, StdBox<c_void>);
}
#[cfg(not(target_os = "windows"))]
static_detour! {
  static ITEM_PICKUP: extern "C" fn(StdBox<c_void>, StdBox<Entity>, StdBox<c_void>);
}
pub static PLAYER_ID: AtomicUsize = AtomicUsize::new(0);
fn item_pickup(this: StdBox<c_void>, entity: StdBox<Entity>, component: StdBox<c_void>) {
    if !DISABLE_ITEM_PICKUP.load(Ordering::Relaxed)
        || entity.id != PLAYER_ID.load(Ordering::Relaxed)
    {
        ITEM_PICKUP.call(this, entity, component);
    }
}
pub fn disable_item_pickup() {
    unsafe {
        let old_item = get_this_call!(
            0x00b90480,
            fn(StdBox<c_void>, StdBox<Entity>, StdBox<c_void>)
        );
        ITEM_PICKUP.initialize(old_item, item_pickup).unwrap();
        ITEM_PICKUP.enable().unwrap();
    }
}
