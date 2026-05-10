use crate::{BitSet, EntityManager, StdBox, StdString, StdVec, TagManager, Transform};
#[repr(C)]
#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub entry: usize,
    pub filename_index: usize,
    pub kill_flag: bool,
    padding: [u8; 3],
    unknown1: isize,
    pub name: StdString,
    unknown2: isize,
    pub tags: BitSet<u16>,
    pub transform: Transform,
    pub children: Option<StdBox<StdVec<StdBox<Entity>>>>,
    pub parent: Option<StdBox<Entity>>,
}
impl Default for Entity {
    #[inline]
    fn default() -> Self {
        todo!()
    }
}
impl StdBox<Entity> {
    #[inline]
    pub unsafe fn set_tag(&mut self, tag: &str) -> bool {
        let mut tag_manager = TagManager::<u16>::global();
        let mut em = EntityManager::global();
        if let Some(n) = tag_manager.tag_indices.get(tag).copied() {
            self.tags.set(n, true);
            em.entity_buckets[usize::from(n)].push(*self);
            true
        } else {
            let stdstring = StdString::from(tag);
            let n = tag_manager.insert_new(stdstring);
            self.tags.set(n, true);
            em.entity_buckets[usize::from(n)].push(*self);
            false
        }
    }
    #[inline]
    pub unsafe fn unset_tag(&mut self, tag: &str) -> bool {
        let tag_manager = TagManager::<u16>::global();
        if let Some(n) = tag_manager.tag_indices.get(tag).copied() {
            self.tags.set(n, false);
            let mut em = EntityManager::global();
            let eb = &mut em.entity_buckets[usize::from(n)];
            let i = eb.iter().position(|e| e.ptr == self.ptr).unwrap();
            eb.swap_remove(i);
            true
        } else {
            false
        }
    }
    #[inline]
    #[must_use]
    pub fn new_entity() -> Self {
        fn new(id: usize, entry: usize) -> Entity {
            Entity {
                id,
                entry,
                filename_index: 0,
                kill_flag: false,
                padding: [0; 3],
                unknown1: 0,
                name: StdString::default(),
                unknown2: 0,
                tags: BitSet::default(),
                transform: Transform::default(),
                children: None,
                parent: None,
            }
        }
        let mut em = EntityManager::global();
        if em.free_ids.is_empty() {
            let entry = em.entities.len();
            let ent = new(em.max_entity_id, entry);
            em.max_entity_id += 1;
            let ent_box = StdBox::new(ent);
            em.entities.push(Some(ent_box));
            ent_box
        } else {
            let entry = em.free_ids.pop();
            let ent = new(em.max_entity_id, entry);
            em.max_entity_id += 1;
            let ent_box = StdBox::new(ent);
            em.entities[entry] = Some(ent_box);
            ent_box
        }
    }
}
