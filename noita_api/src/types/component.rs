pub mod variable_storage;
pub mod world_state;
use crate::{
    BitSet, CStrPtr, ComponentBuffer, ComponentBufferInitVTable, ComponentSystemVTable,
    ComponentUpdaterVTable, ComponentVTable, Entity, EntityManager, StdBox, StdMap, StdString,
    StdVec,
};
use noita_api_macros::assert_size_with;
use std::ffi::CStr;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
pub use variable_storage::*;
pub use world_state::*;
pub trait ComponentTrait: Debug + Default {
    const NAME: &'static CStr;
}
impl ComponentTrait for () {
    const NAME: &'static CStr = c"ERROR";
}
#[repr(C)]
#[assert_size_with(0x48, ())]
#[derive(Debug)]
pub struct ComponentInner<T: ComponentTrait> {
    pub vtable: StdBox<ComponentVTable<T>>,
    pub entry: usize,
    pub type_name: CStrPtr,
    pub type_id: usize,
    pub id: usize,
    pub enabled: bool,
    unk2: [u8; 3],
    pub tags: BitSet<u8>,
    unk3: StdVec<bool>,
    unk4: usize,
    data: T,
}
#[repr(transparent)]
#[derive(Debug)]
pub struct Component<T: ComponentTrait> {
    pub ptr: StdBox<ComponentInner<T>>,
}
impl<T: ComponentTrait + 'static> Component<T> {
    #[inline]
    #[must_use]
    pub fn iter() -> impl DoubleEndedIterator<Item = Self> {
        let coms = ComponentTypeManager::global();
        if let Some(index) = coms
            .component_buffer_indices
            .get(T::NAME.to_str().unwrap())
            .copied()
        {
            let em = EntityManager::global();
            let buffer = em.component_buffers[index]
                .cast::<StdBox<ComponentBuffer<T>>>()
                .as_ref();
            buffer.component_list.iter().flatten().copied()
        } else {
            [].iter().flatten().copied()
        }
    }
    #[inline]
    #[must_use]
    pub fn iter_with_entities() -> impl DoubleEndedIterator<Item = (Entity, Self)> {
        let coms = ComponentTypeManager::global();
        let c = |(a, b): (&Option<Entity>, &Option<Component<_>>)| a.zip(*b);
        if let Some(index) = coms
            .component_buffer_indices
            .get(T::NAME.to_str().unwrap())
            .copied()
        {
            let em = EntityManager::global();
            let buffer = em.component_buffers[index]
                .cast::<StdBox<ComponentBuffer<T>>>()
                .as_ref();
            buffer
                .entities
                .iter()
                .zip(buffer.component_list.iter())
                .filter_map(c)
        } else {
            [].iter().zip([].iter()).filter_map(c)
        }
    }
}
impl<T: ComponentTrait> Clone for Component<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ComponentTrait> Copy for Component<T> {}
impl<T: ComponentTrait> PartialEq for Component<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl<T: ComponentTrait> Deref for Component<T> {
    type Target = ComponentInner<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
impl<T: ComponentTrait> DerefMut for Component<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}
impl<T: ComponentTrait> Default for Component<T> {
    #[inline]
    fn default() -> Self {
        todo!()
    }
}
#[repr(transparent)]
#[derive(Default)]
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
impl Default for ComponentTypeManager {
    #[inline]
    fn default() -> Self {
        todo!()
    }
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
impl Default for ComponentSystemManager {
    #[inline]
    fn default() -> Self {
        todo!()
    }
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
