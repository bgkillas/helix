use crate::{
    ComponentBuffer, Entity, EntityManagerVTable, EventManager, StdBox, StdVec, TagManager,
};
#[repr(C)]
#[derive(Debug)]
pub struct EntityManager {
    pub vtable: StdBox<EntityManagerVTable>,
    pub max_entity_id: usize,
    pub free_ids: StdVec<usize>,
    pub entities: StdVec<Option<StdBox<Entity>>>,
    pub entity_buckets: StdVec<StdVec<StdBox<Entity>>>,
    pub component_buffers: StdVec<StdBox<ComponentBuffer<()>>>,
    pub event_manager: StdBox<EventManager>,
}
impl EntityManager {
    #[must_use]
    #[inline]
    pub fn iter_with_tag(&self, tag: &str) -> impl DoubleEndedIterator<Item = StdBox<Entity>> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied() {
            self.entity_buckets[usize::from(n)].iter().copied()
        } else {
            [].iter().copied()
        }
    }
    #[must_use]
    #[inline]
    pub fn get_id_with_tag(&mut self, id: usize, tag: &str) -> Option<StdBox<Entity>> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied() {
            self.entity_buckets[usize::from(n)]
                .iter()
                .find(|e| e.id == id)
                .copied()
        } else {
            None
        }
    }
    #[must_use]
    #[inline]
    pub fn get_id(&mut self, id: usize) -> Option<StdBox<Entity>> {
        self.entities.iter().flatten().find(|e| e.id == id).copied()
    }
}
