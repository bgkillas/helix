use crate::{
    BitSet, ComponentBufferInitVTable, ComponentSystemVTable, ComponentUpdaterVTable,
    ComponentVTable, StdBox, StdMap, StdString, StdVec,
};
use std::ffi::CStr;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
pub trait ComponentTrait: Debug + Default {
    const NAME: &'static CStr;
}
impl ComponentTrait for () {
    const NAME: &'static CStr = c"ERROR";
}
#[repr(C)]
#[derive(Debug)]
pub struct Component<T: ComponentTrait> {
    pub vtable: StdBox<ComponentVTable<T>>,
    pub local_id: usize,
    pub type_name: *const CStr,
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
#[repr(C)]
#[derive(Debug)]
pub struct ComponentTypeManager {
    pub next_id: usize,
    pub component_buffer_indices: StdMap<StdString, usize>,
}
#[repr(C)]
#[derive(Debug)]
pub struct ComponentSystemManager {
    pub update_order: StdVec<StdBox<ComponentSystem>>,
    pub component_updaters: StdVec<StdBox<ComponentUpdater>>,
    pub component_vtables: StdMap<StdString, ComponentBufferInitVTable>,
    pub unk: [*const usize; 8],
    pub unk2: StdVec<*const usize>,
    pub unk3: [*const usize; 6],
}
#[repr(C)]
#[derive(Debug)]
pub struct ComponentSystem {
    pub vtable: StdBox<ComponentSystemVTable>,
    pub unk: [*const usize; 2],
    pub name: StdString,
}
#[repr(C)]
#[derive(Debug)]
pub struct ComponentUpdater {
    pub vtable: StdBox<ComponentUpdaterVTable>,
    pub name: StdString,
    pub unk: [*const usize; 8],
}
