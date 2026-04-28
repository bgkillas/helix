use crate::*;
#[repr(C)]
#[derive(Debug)]
pub struct TagManager<T> {
    pub tags: StdVec<StdString>,
    pub tag_indices: StdMap<StdString, T>,
    pub max_tag_count: usize,
    pub name: StdString,
}
