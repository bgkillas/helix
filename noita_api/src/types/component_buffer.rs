use crate::{
    Component, ComponentBufferVTable, ComponentTrait, ComponentTypeManager, Entity, EntityManager,
    EventManager, StdBox, StdMap, StdPtr, StdVec,
};
use noita_api_macros::assert_size_with;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
#[repr(C)]
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
    pub entry_to_local_id: StdVec<usize>,
    pub is_local_id_killed: StdVec<bool>,
    pub local_id_to_entry: StdVec<usize>,
    unk2r: *const u64,
    unk3r: *const [*const [usize; 4]; 8],
    unk2: [usize; 3],
    pub len: usize,
    unk4r: [usize; 2],
    pub entity_manager: StdPtr<EntityManager>,
    pub event_manager: StdPtr<EventManager>,
    unk3: [usize; 6],
}
impl<T: ComponentTrait> Debug for ComponentBuffer<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentBuffer")
            .field("TYPE", &T::NAME.to_str().unwrap())
            .field("end", &self.end)
            .field_with("entity_entry", |fi| {
                fi.debug_list()
                    .entries(
                        self.entity_entry
                            .iter()
                            .enumerate()
                            .filter(|(_, c)| **c != usize::MAX),
                    )
                    .finish()
            })
            .field_with("entities", |fi| {
                fi.debug_list()
                    .entries(self.entities.iter().map(|e| e.map(|ei| (ei.entry, ei.id))))
                    .finish()
            })
            .field("prev", &self.prev)
            .field("next", &self.next)
            .field_with("component_list", |fi| {
                fi.debug_list()
                    .entries(
                        self.component_list
                            .iter()
                            .map(|c| c.map(|ci| (ci.entry, ci.id))),
                    )
                    .finish()
            })
            .field("free_ids", &self.free_ids)
            .field("entry_to_local_id", &self.entry_to_local_id)
            .field("is_local_id_killed", &self.is_local_id_killed)
            .field("local_id_to_entry", &self.local_id_to_entry)
            .field("len", &self.len)
            .finish()
    }
}
impl<T: ComponentTrait> ComponentBuffer<T> {
    #[must_use]
    #[inline]
    pub fn global() -> Option<(usize, StdBox<ComponentBuffer<T>>)> {
        let coms = ComponentTypeManager::global();
        if let Some(type_id) = coms
            .component_buffer_indices
            .get(T::NAME.to_str().unwrap())
            .copied()
        {
            let em = EntityManager::global();
            Some((
                type_id,
                em.component_buffers[type_id].cast::<ComponentBuffer<T>>(),
            ))
        } else {
            None
        }
    }
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
