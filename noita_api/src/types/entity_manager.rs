use crate::{ComponentBuffer, Entity, EntityManagerVTable, EventManager, StdBox, StdVec};
use noita_api_macros::assert_size;
#[repr(C)]
#[derive(Debug)]
#[assert_size(0x3c)]
pub struct EntityManager {
    pub vtable: StdBox<EntityManagerVTable>,
    pub max_entity_id: usize,
    pub free_ids: StdVec<usize>,
    pub entities: StdVec<Option<Entity>>,
    pub entity_buckets: StdVec<StdVec<Entity>>,
    pub component_buffers: StdVec<StdBox<ComponentBuffer<()>>>,
    pub event_manager: StdBox<EventManager>,
}
impl Default for EntityManager {
    #[inline]
    fn default() -> Self {
        let mut entity_buckets = StdVec::with_capacity(512);
        for _ in 0..512 {
            entity_buckets.push(StdVec::default());
        }
        Self {
            vtable: StdBox::new(EntityManagerVTable {}),
            max_entity_id: 0,
            free_ids: StdVec::default(),
            entities: StdVec::default(),
            entity_buckets,
            component_buffers: StdVec::default(),
            event_manager: StdBox::new(EventManager::default()),
        }
    }
}
#[test]
fn test_iter() {
    unsafe {
        let ent1 = Entity::default();
        ent1.set_tag("tag_a");
        assert!(ent1.has_tag("tag_a"));
        assert!(!ent1.has_tag("tag_b"));
        for ent in Entity::iter_with_tag("tag_a") {
            ent.set_tag("tag_b");
        }
        assert!(ent1.has_tag("tag_a"));
        assert!(ent1.has_tag("tag_b"));
        for ent in Entity::iter_with_tag("tag_a") {
            ent.unset_tag("tag_a");
            assert!(!ent1.has_tag("tag_a"));
            ent.set_tag("tag_a");
        }
        assert!(ent1.has_tag("tag_a"));
        assert!(ent1.has_tag("tag_b"));
        ent1.kill_now();
    }
}
