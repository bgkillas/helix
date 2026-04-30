use crate::{BitSet, StdString, StdVec, Transform};
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
    pub tags: BitSet<16>,
    pub transform: Transform,
    pub children: *mut StdVec<*mut Entity>,
    pub parent: *mut Entity,
}
