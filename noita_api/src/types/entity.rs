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
impl StdBox<Entity> {
    #[inline]
    pub fn set_tag(&mut self, tag: &str) -> bool {
        let mut tag_manager = TagManager::<u16>::global();
        let mut em = EntityManager::global();
        if let Some(n) = tag_manager.tag_indices.get(tag).copied() {
            self.tags.set(n, true);
            em.entity_buckets[usize::from(n)].push(*self);
            true
        } else {
            let stdstring = StdString::from(tag);
            tag_manager.insert(stdstring);
            let mut vec = StdVec::default();
            vec.push(*self);
            em.entity_buckets.push(vec);
            false
        }
    }
    #[inline]
    pub fn unset_tag(&mut self, tag: &str) -> bool {
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
}
