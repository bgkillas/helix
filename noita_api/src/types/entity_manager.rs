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
    pub component_buffers: StdVec<Option<StdBox<ComponentBuffer<()>>>>,
    pub event_manager: StdBox<EventManager>,
}
impl EntityManager {
    #[must_use]
    #[inline]
    pub fn iter_with_tag(&self, tag: &str) -> impl DoubleEndedIterator<Item = &StdBox<Entity>> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied()
            && let Some(vec) = self.entity_buckets.get(usize::from(n))
        {
            vec.iter()
        } else {
            [].iter()
        }
    }
    #[must_use]
    #[inline]
    pub fn iter_mut_with_tag(
        &mut self,
        tag: &str,
    ) -> impl DoubleEndedIterator<Item = &mut StdBox<Entity>> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied()
            && let Some(vec) = self.entity_buckets.get_mut(usize::from(n))
        {
            vec.iter_mut()
        } else {
            [].iter_mut()
        }
    }
}
