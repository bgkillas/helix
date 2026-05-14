use crate::{
    Component, ComponentBufferVTable, ComponentTrait, Entity, EntityManager, EventManager, StdBox,
    StdMap, StdPtr, StdVec,
};
use noita_api_macros::assert_size_with;
use std::ops::{Deref, DerefMut};
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
    unk1: [usize; 2],
    pub free_ids: usize,
    pub entry_to_local_id: StdVec<usize>, //0 means entry is unoccupied
    pub is_local_id_killed: StdVec<bool>, //len of this determines next local id
    pub local_id_to_entry: StdVec<usize>, //-1 means local id died
    unk2r: *const u64,
    unk3r: *const [*const [usize; 4]; 8],
    unk2: [usize; 3],
    pub len: usize,
    unk4r: [usize; 2],
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
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct ComponentIdMap {
    map: StdMap<usize, Component<()>>,
}
impl Deref for ComponentIdMap {
    type Target = StdMap<usize, Component<()>>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl DerefMut for ComponentIdMap {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
