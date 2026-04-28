use crate::*;
#[repr(C)]
#[derive(Debug)]
pub struct ComponentBuffer<T> {
    pub vtable: StdBox<ComponentBufferVTable>,
    pub end: usize,
    unk: [isize; 2],
    pub entity_entry: StdVec<usize>,
    pub entities: StdVec<Option<StdBox<Entity>>>,
    pub prev: StdVec<usize>,
    pub next: StdVec<usize>,
    pub component_list: StdVec<Option<StdBox<ComponentData<T>>>>,
    unk1r: *const u64,
    unk1: [*const usize; 4],
    unk1_vec: StdVec<*const usize>,
    unk2_vec: StdVec<*const usize>,
    unk3_vec: StdVec<*const usize>,
    unk2r: *const u64,
    unk2: [*const usize; 7],
    pub entity_manager: StdBox<EntityManager>,
    pub event_manager: StdBox<EventManager>,
    unk3: [*const usize; 6],
}
