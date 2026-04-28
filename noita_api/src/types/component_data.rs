use crate::*;
use std::ffi::CString;
#[repr(C)]
#[derive(Debug)]
pub struct ComponentData<T> {
    pub vtable: StdBox<ComponentVFTable>,
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
#[repr(C)]
#[derive(Debug)]
pub struct ComponentVFTable {}
