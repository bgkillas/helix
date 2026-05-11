use crate::{
    Component, ComponentBufferVTable, ComponentTrait, Entity, EntityManager, EventManager, StdBox,
    StdPtr, StdVec,
};
use noita_api_macros::assert_size_with;
#[repr(C)]
#[derive(Debug)]
#[assert_size_with(0xc4, ())]
pub struct ComponentBuffer<T: ComponentTrait> {
    pub vtable: StdBox<ComponentBufferVTable>,
    pub end: usize,
    unk: [usize; 2],
    pub entity_entry: StdVec<usize>,
    pub entities: StdVec<Option<Entity>>,
    pub prev: StdVec<usize>,
    pub next: StdVec<usize>,
    pub component_list: StdVec<Option<Component<T>>>,
    unk1r: *const u64,
    unk4: *const [*const [usize; 4]; 8],
    unk1: [usize; 3],
    unk1_vec: StdVec<usize>,
    unk2_vec: StdVec<bool>,
    unk3_vec: StdVec<usize>,
    unk2r: *const u64,
    unk3r: *const [*const [usize; 4]; 8],
    unk2: [usize; 6],
    pub entity_manager: StdPtr<EntityManager>,
    pub event_manager: StdPtr<EventManager>,
    unk3: [usize; 6],
}
#[derive(Debug)]
pub struct ComponentIter<'a, T: ComponentTrait> {
    pub current: usize,
    pub next: &'a [usize],
    pub components: &'a [Option<Component<T>>],
}
impl<T: ComponentTrait> Iterator for ComponentIter<'_, T> {
    type Item = Component<T>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == usize::MAX {
            return None;
        }
        let com = self.components[self.current];
        self.current = self.next[self.current];
        com
    }
}
