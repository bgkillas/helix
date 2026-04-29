use crate::*;
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
    pub fn get_entities_with_tag(
        &self,
        tag: &StdString,
    ) -> impl DoubleEndedIterator<Item = StdBox<Entity>> {
        let n = TagManager::<u16>::global().tag_indices.get(tag).unwrap();
        self.entity_buckets
            .get(n as usize)
            .unwrap()
            .as_ref()
            .iter()
            .copied()
            .flatten()
    }
}
