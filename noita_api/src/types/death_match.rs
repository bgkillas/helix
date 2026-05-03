use crate::{Entity, StdBox, StdVec};
use noita_api_macros::assert_size;
#[repr(C)]
#[derive(Debug)]
#[assert_size(0xc8)]
pub struct DeathMatch {
    pub application_vtable: *const usize,
    pub mouse_listener_vtable: *const usize,
    pub keyboard_listener_vtable: *const usize,
    unk: *const usize,
    pub joystick_listener_vtable: *const usize,
    pub simple_ui_listener_vtable: *const usize,
    pub event_listener_vtable: *const usize,
    unk1: [*const usize; 15],
    pub entities: StdVec<StdBox<Entity>>,
    unk2: [*const usize; 25],
}
