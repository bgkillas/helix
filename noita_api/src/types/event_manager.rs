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
impl Default for EventManager {
    #[inline]
    fn default() -> Self {
        Self {
            vtable: StdBox::new(EventManagerVTable {}),
            unk1: 0,
            unk2: 0,
            functions: StdVec::default(),
        }
    }
}
