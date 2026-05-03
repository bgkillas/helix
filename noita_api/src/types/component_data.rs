use crate::{BitSet, ComponentVTable, StdBox, StdVec};
use std::ffi::CString;
use std::ops::{Deref, DerefMut};
#[repr(C)]
#[derive(Debug)]
pub struct ComponentData<T> {
    pub vtable: StdBox<ComponentVTable>,
    pub local_id: usize,
    pub type_name: CString,
    pub type_id: usize,
    pub id: usize,
    pub enabled: bool,
    unk2: [u8; 3],
    pub tags: BitSet<8>,
    unk3: StdVec<usize>,
    unk4: usize,
    data: T,
}
#[repr(transparent)]
pub struct MaxComponent {
    pub max: usize,
}
impl Deref for MaxComponent {
    type Target = usize;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.max
    }
}
impl DerefMut for MaxComponent {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.max
    }
}
