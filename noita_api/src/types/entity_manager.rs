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
