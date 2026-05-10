use crate::{StdMap, StdString, StdVec};
use std::fmt::Debug;
use std::ptr;
#[repr(C)]
#[derive(Debug)]
pub struct TagManager<T> {
    pub tags: StdVec<StdString>,
    pub tag_indices: StdMap<StdString, T>,
    pub max_tag_count: usize,
    pub name: StdString,
}
impl<T: TryFrom<usize> + Copy> TagManager<T>
where
    <T as TryFrom<usize>>::Error: Debug,
{
    #[inline]
    pub fn insert(&mut self, tag: StdString) -> T {
        if let Some(n) = self.tag_indices.get(&tag) {
            *n
        } else {
            self.insert_new(tag)
        }
    }
    #[inline]
    pub fn insert_new(&mut self, tag: StdString) -> T {
        if self.max_tag_count == self.tags.len() {
            panic!()
        } else {
            let index = T::try_from(self.tags.len()).unwrap();
            let tag_copy = unsafe { ptr::from_ref(&tag).read() };
            self.tags.push(tag_copy);
            self.tag_indices.insert(tag, index);
            index
        }
    }
}
