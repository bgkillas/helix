use crate::{StdMap, StdString, StdVec};
use std::ptr;
#[repr(C)]
#[derive(Debug)]
pub struct TagManager<T> {
    pub tags: StdVec<StdString>,
    pub tag_indices: StdMap<StdString, T>,
    pub max_tag_count: usize,
    pub name: StdString,
}
impl<T: From<usize> + Copy> TagManager<T> {
    #[inline]
    pub fn insert(&mut self, tag: StdString) -> T {
        if let Some(n) = self.tag_indices.get(&tag) {
            *n
        } else {
            let index = T::from(self.tags.len());
            let tag_copy = unsafe { ptr::from_ref(&tag).read() };
            self.tags.push(tag_copy);
            self.tag_indices.insert(tag, index);
            index
        }
    }
}
