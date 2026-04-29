use crate::*;
use std::ops::Deref;
#[repr(C)]
#[derive(Debug)]
pub struct EntityManager {
    pub vtable: StdBox<EntityManagerVTable>,
    pub max_entity_id: usize,
    pub free_ids: StdVec<usize>,
    pub entities: StdVec<Option<StdBox<Entity>>>,
    pub entity_buckets: StdVec<StdVec<Option<StdBox<Entity>>>>,
    pub component_buffers: StdVec<Option<StdBox<ComponentBuffer<()>>>>,
    pub event_manager: StdBox<EventManager>,
}
impl EntityManager {
    pub fn iter_with_tag(
        &self,
        tag: impl Into<StdString>,
    ) -> impl DoubleEndedIterator<Item = StdBox<Entity>> {
        if let Some(n) = TagManager::<u16>::global()
            .tag_indices
            .get(&tag.into())
            .copied()
            && let Some(vec) = self.entity_buckets.get(n as usize)
        {
            vec.deref()
        } else {
            &[]
        }
        .iter()
        .copied()
        .flatten()
    }
}
