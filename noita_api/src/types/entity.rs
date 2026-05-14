use crate::{
    BitSet, Component, ComponentBuffer, ComponentIter, ComponentTrait, ComponentTypeManager,
    EntityManager, FileNames, MaybeUninitComponentInner, StdBox, StdString, StdVec, TagManager,
    Transform,
};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
#[repr(C)]
#[derive(Default)]
pub struct EntityInner {
    pub id: usize,
    pub entry: usize,
    pub filename_index: usize,
    pub kill_flag: bool,
    unknown1: isize,
    pub name: StdString,
    unknown2: isize,
    pub tags: BitSet<u16>,
    pub transform: Transform,
    pub children: Option<StdBox<StdVec<Entity>>>,
    pub parent: Option<Entity>,
}
impl EntityInner {
    fn debug_children(&self) -> impl Debug {
        fmt::from_fn(|f| {
            if let Some(ents) = self.children {
                let mut dl = f.debug_list();
                for ent in ents.iter() {
                    dl.entry_with(|fs| {
                        fs.debug_struct("EntityInner")
                            .field("ptr", &ent.ptr.ptr)
                            .field("id", &ent.id)
                            .field("entry", &ent.entry)
                            .field("filename_index", &ent.filename_index)
                            .field("filename", &FileNames::global().get(ent.filename_index))
                            .field("kill_flag", &ent.kill_flag)
                            .field("unknown1", &ent.unknown1)
                            .field("name", &ent.name)
                            .field("unknown2", &ent.unknown2)
                            .field("tags", &ent.tags)
                            .field("transform", &ent.transform)
                            .field_with("children", |fi| Debug::fmt(&ent.debug_children(), fi))
                            .finish()
                    });
                }
                dl.finish()
            } else {
                write!(f, "None")
            }
        })
    }
    fn debug_parent(&self) -> impl Debug {
        fmt::from_fn(|f| {
            if let Some(ent) = self.parent {
                f.debug_struct("EntityInner")
                    .field("ptr", &ent.ptr.ptr)
                    .field("id", &ent.id)
                    .field("entry", &ent.entry)
                    .field("filename_index", &ent.filename_index)
                    .field("filename", &FileNames::global().get(ent.filename_index))
                    .field("kill_flag", &ent.kill_flag)
                    .field("unknown1", &ent.unknown1)
                    .field("name", &ent.name)
                    .field("unknown2", &ent.unknown2)
                    .field("tags", &ent.tags)
                    .field("transform", &ent.transform)
                    .field_with("parent", |fi| Debug::fmt(&ent.debug_parent(), fi))
                    .finish()
            } else {
                write!(f, "None")
            }
        })
    }
}
impl Debug for EntityInner {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntityInner")
            .field("id", &self.id)
            .field("entry", &self.entry)
            .field("filename_index", &self.filename_index)
            .field("filename", &FileNames::global().get(self.filename_index))
            .field("kill_flag", &self.kill_flag)
            .field("unknown1", &self.unknown1)
            .field("name", &self.name)
            .field("unknown2", &self.unknown2)
            .field("tags", &self.tags)
            .field("transform", &self.transform)
            .field_with("children", |fi| Debug::fmt(&self.debug_children(), fi))
            .field_with("parent", |fi| Debug::fmt(&self.debug_parent(), fi))
            .finish()
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entity {
    pub ptr: StdBox<EntityInner>,
}
impl Default for Entity {
    #[inline]
    fn default() -> Self {
        let mut em = EntityManager::global();
        let mut ent = EntityInner::default();
        em.max_entity_id += 1;
        ent.id = em.max_entity_id;
        if let Some(entry) = em.free_ids.pop() {
            ent.entry = entry;
            let ent_box = StdBox::new(ent);
            let ret = Self { ptr: ent_box };
            em.entities[entry] = Some(ret);
            ret
        } else {
            ent.entry = em.entities.len();
            let ent_box = StdBox::new(ent);
            let ret = Self { ptr: ent_box };
            em.entities.push(Some(ret));
            ret
        }
    }
}
impl Entity {
    #[inline]
    #[must_use]
    pub fn has_tag(self, tag: &str) -> bool {
        let tag_manager = TagManager::<u16>::global();
        if let Some(n) = tag_manager.tag_indices.get(tag).copied() {
            self.tags.get(n)
        } else {
            false
        }
    }
    #[inline]
    #[allow(clippy::must_use_candidate)]
    pub unsafe fn set_tag(mut self, tag: &str) -> bool {
        let mut tag_manager = TagManager::<u16>::global();
        let mut em = EntityManager::global();
        if let Some(n) = tag_manager.tag_indices.get(tag).copied() {
            self.tags.set(n, true);
            em.entity_buckets[usize::from(n)].push(self);
            true
        } else {
            let stdstring = StdString::from(tag);
            let n = tag_manager.insert_new(stdstring);
            self.tags.set(n, true);
            em.entity_buckets[usize::from(n)].push(self);
            false
        }
    }
    #[inline]
    #[allow(clippy::must_use_candidate)]
    pub unsafe fn unset_tag(mut self, tag: &str) -> bool {
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
    pub fn push_child(self, child: Self) {
        let mut vec = if let Some(children) = self.children {
            children
        } else {
            StdBox::new(StdVec::with_capacity(8))
        };
        vec.push(child);
    }
    #[inline]
    pub fn remove_child(self, child: Self) {
        if let Some(mut children) = self.children {
            if let Some(pos) = children.iter().position(|e| e.ptr == child.ptr) {
                children.swap_remove(pos);
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
    #[inline]
    pub fn set_parent(mut self, new_parent: Self) {
        if let Some(parent) = self.parent {
            parent.remove_child(self);
        }
        self.parent = Some(new_parent);
    }
    #[inline]
    pub fn remove_parent(mut self) {
        if let Some(parent) = self.parent {
            parent.remove_child(self);
        }
        self.parent = None;
    }
    #[must_use]
    #[inline]
    pub fn iter() -> impl DoubleEndedIterator<Item = Self> {
        let em = EntityManager::global();
        em.as_ref().entities.iter().flatten().copied()
    }
    #[must_use]
    #[inline]
    pub fn iter_with_tag(tag: &str) -> impl DoubleEndedIterator<Item = Self> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied() {
            let em = EntityManager::global();
            em.as_ref().entity_buckets[usize::from(n)].iter().copied()
        } else {
            [].iter().copied()
        }
    }
    #[must_use]
    #[inline]
    pub fn get_with_tag(id: usize, tag: &str) -> Option<Self> {
        if let Some(n) = TagManager::<u16>::global().tag_indices.get(tag).copied() {
            Entity::get_with_tag_id(id, n)
        } else {
            None
        }
    }
    #[must_use]
    #[inline]
    pub fn iter_with_tag_id(tag_id: u16) -> impl DoubleEndedIterator<Item = Self> {
        let em = EntityManager::global();
        em.as_ref().entity_buckets[usize::from(tag_id)]
            .iter()
            .copied()
    }
    #[must_use]
    #[inline]
    pub fn get_with_tag_id(id: usize, tag_id: u16) -> Option<Self> {
        let em = EntityManager::global();
        em.entity_buckets[usize::from(tag_id)]
            .iter()
            .find(|e| e.id == id)
            .copied()
    }
    #[must_use]
    #[inline]
    pub fn get(id: usize) -> Option<Self> {
        let em = EntityManager::global();
        em.entities.iter().flatten().find(|e| e.id == id).copied()
    }
    #[must_use]
    #[inline]
    pub fn get_entry(entry: usize) -> Option<Self> {
        let em = EntityManager::global();
        em.entities[entry]
    }
    #[must_use]
    #[inline]
    pub fn get_component<T: ComponentTrait>(self, id: usize) -> Option<Component<T>> {
        self.iter_components().find(|c| c.id == id)
    }
    #[must_use]
    #[inline]
    pub fn get_component_entry<T: ComponentTrait>(self, entry: usize) -> Option<Component<T>> {
        self.iter_components().find(|c| c.entry == entry)
    }
    #[must_use]
    #[inline]
    pub fn iter_components<'a, T: ComponentTrait>(self) -> ComponentIter<'a, T> {
        let coms = ComponentTypeManager::global();
        if let Some(index) = coms
            .component_buffer_indices
            .get(T::NAME.to_str().unwrap())
            .copied()
        {
            let em = EntityManager::global();
            let buffer = em.component_buffers[index].cast::<ComponentBuffer<T>>();
            if let Some(current) = buffer.entity_entry.get(self.entry).copied() {
                let buffer_ref = buffer.as_ref();
                return ComponentIter {
                    current,
                    next: &buffer_ref.next,
                    components: &buffer_ref.component_list,
                };
            }
        }
        ComponentIter {
            current: usize::MAX,
            next: &[],
            components: &[],
        }
    }
    #[must_use]
    #[inline]
    pub fn add_component<T: ComponentTrait>(self) -> Component<T> {
        let coms = ComponentTypeManager::global();
        if let Some(index) = coms
            .component_buffer_indices
            .get(T::NAME.to_str().unwrap())
            .copied()
        {
            let em = EntityManager::global();
            let mut buffer: StdBox<ComponentBuffer<T>> = em.component_buffers[index].cast();
            let mut maybe_com_inner = MaybeUninitComponentInner::<T>::default();
            maybe_com_inner.vtable = Some(T::vtable());
            maybe_com_inner.type_id = index;
            maybe_com_inner.type_name = T::NAME.as_ptr().into();
            let maybe_com = StdBox::new(maybe_com_inner);
            let mut com: Component<T> = Component {
                ptr: maybe_com.cast(),
            };
            if let Some(entry) = buffer.component_list.iter().position(Option::is_none) {
                com.entry = entry;
                buffer.component_list[com.entry] = Some(com);
                buffer.entities[com.entry] = Some(self);
            } else {
                com.entry = buffer.component_list.len();
                buffer.component_list.push(Some(com));
                buffer.entities.push(Some(self));
                buffer.next.push(usize::MAX);
                buffer.prev.push(usize::MAX);
            }
            buffer.entity_entry.resize(self.entry, usize::MAX);
            if buffer.entity_entry[self.entry] == usize::MAX {
                buffer.entity_entry[self.entry] = com.entry;
            } else {
                todo!()
            }
            com
        } else {
            todo!()
        }
    }
}
impl Deref for Entity {
    type Target = EntityInner;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
impl DerefMut for Entity {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}
