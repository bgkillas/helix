use crate::{
    ComponentBuffer, Entity, EntityManagerVTable, EventManager, StdBox, StdVec, TagManager,
};
#[repr(C)]
#[derive(Debug)]
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
impl EntityManager {
    #[must_use]
    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = Entity> {
        self.entities.iter().flatten().copied()
    }
    #[must_use]
    #[inline]
    pub fn iter_with_tag(&self, tag: &str) -> impl DoubleEndedIterator<Item = Entity> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied() {
            self.entity_buckets[usize::from(n)].iter().copied()
        } else {
            [].iter().copied()
        }
    }
    #[must_use]
    #[inline]
    pub fn get_id_with_tag(&self, id: usize, tag: &str) -> Option<Entity> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied() {
            self.get_id_with_tag_id(id, n)
        } else {
            None
        }
    }
    #[must_use]
    #[inline]
    pub fn iter_with_tag_id(&self, tag_id: u16) -> impl DoubleEndedIterator<Item = Entity> {
        self.entity_buckets[usize::from(tag_id)].iter().copied()
    }
    #[must_use]
    #[inline]
    pub fn get_id_with_tag_id(&self, id: usize, tag_id: u16) -> Option<Entity> {
        self.entity_buckets[usize::from(tag_id)]
            .iter()
            .find(|e| e.id == id)
            .copied()
    }
    #[must_use]
    #[inline]
    pub fn get_id(&self, id: usize) -> Option<Entity> {
        self.entities.iter().flatten().find(|e| e.id == id).copied()
    }
}
#[test]
fn test_iter() {
    unsafe {
        let em = EntityManager::global();
        let mut ent1 = Entity::default();
        ent1.set_tag("tag_a");
        assert!(ent1.has_tag("tag_a"));
        assert!(!ent1.has_tag("tag_b"));
        for mut ent in em.iter_with_tag("tag_a") {
            ent.set_tag("tag_b");
        }
        assert!(ent1.has_tag("tag_a"));
        assert!(ent1.has_tag("tag_b"));
        for mut ent in em.iter_with_tag("tag_a") {
            ent.unset_tag("tag_a");
            assert!(!ent1.has_tag("tag_a"));
            ent.set_tag("tag_a");
        }
        assert!(ent1.has_tag("tag_a"));
        assert!(ent1.has_tag("tag_b"));
    }
}
