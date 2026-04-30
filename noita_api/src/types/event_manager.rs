use crate::{EventManagerVTable, StdBox, StdVec};
#[repr(C)]
#[derive(Debug)]
pub struct EventManager {
    pub vtable: StdBox<EventManagerVTable>,
    pub unk1: usize,
    pub unk2: usize,
    pub functions: StdVec<Event>,
}
#[repr(C)]
#[derive(Debug)]
pub struct Event {
    unk: usize,
    func: *const usize,
}
