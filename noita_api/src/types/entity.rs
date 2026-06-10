use crate::{
    BitSet, Component, ComponentBuffer, ComponentIter, ComponentTrait, EntityManager, FileNames,
    StdBox, StdString, StdVec, TagManager, Transform,
};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
#[repr(C)]
#[derive(Default)]
pub struct EntityInner {
    pub id: usize,
    pub entry: usize,
    pub filename_index: FileNameIndex,
    pub kill_flag: bool,
    unknown1: isize,
    pub name: StdString,
    unknown2: isize,
    pub tags: BitSet<u16>,
    pub transform: Transform,
    pub children: Option<StdBox<StdVec<Entity>>>,
    pub parent: Option<Entity>,
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Default)]
pub struct FileNameIndex {
    pub index: usize,
}
impl Debug for FileNameIndex {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FileNameIndex")
            .field(&self.index)
            .field(&self.get_file_name())
            .finish()
    }
}
impl FileNameIndex {
    #[inline]
    #[must_use]
    pub fn get_file_name<'a>(self) -> Option<&'a str> {
        if self.index != 0 {
            let filenames = FileNames::global().as_ref();
            Some(filenames[self.index - 1].as_str())
        } else {
            None
        }
    }
}
impl EntityInner {
    #[inline]
    #[must_use]
    pub fn debug_children(&self) -> impl Debug {
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
    #[inline]
    #[must_use]
    pub fn debug_parent(&self) -> impl Debug {
        fmt::from_fn(|f| {
            if let Some(ent) = self.parent {
                f.debug_struct("EntityInner")
                    .field("ptr", &ent.ptr.ptr)
                    .field("id", &ent.id)
                    .field("entry", &ent.entry)
                    .field("filename_index", &ent.filename_index)
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
    #[inline]
    #[must_use]
    pub fn debug_self(&self) -> impl Debug {
        fmt::from_fn(|f| {
            f.debug_struct("EntityInner")
                .field("id", &self.id)
                .field("entry", &self.entry)
                .field("filename_index", &self.filename_index)
                .field("kill_flag", &self.kill_flag)
                .field("unknown1", &self.unknown1)
                .field("name", &self.name)
                .field("unknown2", &self.unknown2)
                .field("tags", &self.tags)
                .field("transform", &self.transform)
                .finish()
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
    pub fn kill_now(mut self) {
        if let Some(children) = self.children {
            for ent in children.iter() {
                ent.kill_children_now();
            }
        }
        if let Some(parent) = self.parent {
            parent.kill_parent_now(self);
        }
        let mut em = EntityManager::global();
        em.entities[self.entry] = None;
        em.free_ids.push(self.entry);
        self.name.free();
        self.ptr.free();
    }
    #[inline]
    pub fn kill_parent_now(mut self, not: Self) {
        if let Some(children) = self.children {
            for ent in children.iter().copied() {
                if not != ent {
                    ent.kill_children_now();
                }
            }
        }
        if let Some(parent) = self.parent {
            parent.kill_parent_now(self);
        }
        let mut em = EntityManager::global();
        em.entities[self.entry] = None;
        em.free_ids.push(self.entry);
        self.name.free();
        self.ptr.free();
    }
    #[inline]
    pub fn kill_children_now(mut self) {
        if let Some(children) = self.children {
            for ent in children.iter() {
                ent.kill_now();
            }
        }
        let mut em = EntityManager::global();
        em.entities[self.entry] = None;
        em.free_ids.push(self.entry);
        self.name.free();
        self.ptr.free();
    }
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
            em.entity_buckets[n.strict_cast::<usize>()].push(self);
            true
        } else {
            let stdstring = StdString::from(tag);
            let n = tag_manager.insert_new(stdstring);
            self.tags.set(n, true);
            em.entity_buckets[n.strict_cast::<usize>()].push(self);
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
            let eb = &mut em.entity_buckets[n.strict_cast::<usize>()];
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
            em.as_ref().entity_buckets[n.strict_cast::<usize>()]
                .iter()
                .copied()
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
        if let Some((_, buffer)) = ComponentBuffer::<T>::global()
            && buffer.entities[entry].is_some_and(|e| e == self)
        {
            buffer.component_list[entry]
        } else {
            None
        }
    }
    #[must_use]
    #[inline]
    pub fn iter_components<'a, T: ComponentTrait>(self) -> ComponentIter<'a, T> {
        if let Some((_, buffer)) = ComponentBuffer::<T>::global()
            && let Some(current) = buffer.entity_entry.get(self.entry).copied()
        {
            let buffer_ref = buffer.as_ref();
            ComponentIter {
                current,
                next: &buffer_ref.next,
                components: &buffer_ref.component_list,
            }
        } else {
            ComponentIter {
                current: usize::MAX,
                next: &[],
                components: &[],
            }
        }
    }
    #[inline]
    pub fn retain_components<T: ComponentTrait>(self, mut f: impl FnMut(Component<T>) -> bool) {
        let (_, buffer) = ComponentBuffer::<T>::global().unwrap();
        let mut start = buffer.entity_entry[self.entry];
        while start != usize::MAX {
            let com = buffer.component_list[start].unwrap();
            if !f(com) {
                self.remove_component_inner(com, buffer);
            }
            start = buffer.next[start];
        }
    }
    #[inline]
    pub fn remove_component<T: ComponentTrait>(self, com: Component<T>) {
        let (_, buffer) = ComponentBuffer::<T>::global().unwrap();
        self.remove_component_inner(com, buffer);
    }
    fn remove_component_inner<T: ComponentTrait>(
        self,
        com: Component<T>,
        mut buffer: StdBox<ComponentBuffer<T>>,
    ) {
        buffer.free_ids += 1;
        buffer.len -= 1;
        if buffer.entity_entry[self.entry] == com.entry {
            buffer.entity_entry[self.entry] = buffer.next[com.entry];
        }
        buffer.entities[com.entry] = None;
        if buffer.prev[com.entry] != usize::MAX {
            let prev = buffer.prev[com.entry];
            buffer.next[prev] = buffer.next[com.entry];
        }
        if buffer.next[com.entry] != usize::MAX {
            let next = buffer.next[com.entry];
            buffer.prev[next] = buffer.prev[com.entry];
        }
        buffer.component_list[com.entry] = None;
        let local_id = buffer.entry_to_local_id[com.entry];
        buffer.entry_to_local_id[com.entry] = 0;
        buffer.is_local_id_killed[local_id] = true;
        buffer.local_id_to_entry[local_id] = usize::MAX;
        com.free();
    }
    #[must_use]
    #[inline]
    pub fn add_component<T: ComponentTrait>(self) -> Component<T> {
        let (type_id, mut buffer) = ComponentBuffer::<T>::new_global();
        let mut com = Component::<T>::new(type_id);
        if buffer.free_ids == 0 {
            com.entry = buffer.len;
            buffer.component_list.push(Some(com));
            buffer.entities.push(Some(self));
            let local_id = buffer.is_local_id_killed.len();
            if buffer.next.len() <= buffer.len {
                buffer.next.push(usize::MAX);
                buffer.prev.push(usize::MAX);
                buffer.entry_to_local_id.push(local_id);
            } else {
                buffer.entry_to_local_id[com.entry] = local_id;
            }
            buffer.is_local_id_killed.push(false);
            buffer.local_id_to_entry.push(com.entry);
        } else {
            let entry = buffer
                .component_list
                .iter()
                .position(Option::is_none)
                .unwrap();
            com.entry = entry;
            buffer.component_list[com.entry] = Some(com);
            buffer.entities[com.entry] = Some(self);
            let local_id = buffer.is_local_id_killed.len();
            buffer.entry_to_local_id[com.entry] = local_id;
            buffer.is_local_id_killed.push(false);
            buffer.local_id_to_entry.push(com.entry);
            buffer.free_ids -= 1;
        }
        buffer.len += 1;
        buffer.entity_entry.resize(self.entry + 1, usize::MAX);
        if buffer.entity_entry[self.entry] == usize::MAX {
            buffer.entity_entry[self.entry] = com.entry;
        } else {
            let mut end = buffer.entity_entry[self.entry];
            while buffer.next[end] != usize::MAX {
                end = buffer.next[end];
            }
            buffer.next[end] = com.entry;
            buffer.prev[com.entry] = end;
        }
        com
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
#[test]
fn test_coms() {
    use crate::VariableStorage;
    let ent0 = Entity::default();
    let ent1 = Entity::default();
    let ent2 = Entity::default();
    let com0 = ent0.add_component::<VariableStorage>();
    let com1 = ent1.add_component::<VariableStorage>();
    let com2 = ent1.add_component::<VariableStorage>();
    let com3 = ent0.add_component::<VariableStorage>();
    let com4 = ent0.add_component::<VariableStorage>();
    let com5 = ent2.add_component::<VariableStorage>();
    let com6 = ent0.add_component::<VariableStorage>();
    assert_eq!(
        ent0.iter_components::<VariableStorage>()
            .collect::<Vec<_>>(),
        vec![com0, com3, com4, com6]
    );
    assert_eq!(
        ent1.iter_components::<VariableStorage>()
            .collect::<Vec<_>>(),
        vec![com1, com2]
    );
    assert_eq!(
        ent2.iter_components::<VariableStorage>()
            .collect::<Vec<_>>(),
        vec![com5]
    );
    ent0.retain_components(|c| c != com3);
    assert_eq!(
        ent0.iter_components::<VariableStorage>()
            .collect::<Vec<_>>(),
        vec![com0, com4, com6]
    );
    ent0.remove_component(com4);
    assert_eq!(
        ent0.iter_components::<VariableStorage>()
            .collect::<Vec<_>>(),
        vec![com0, com6]
    );
    ent0.retain_components::<VariableStorage>(|_| false);
    ent1.retain_components::<VariableStorage>(|_| false);
    ent2.retain_components::<VariableStorage>(|_| false);
    assert_eq!(Entity::iter().collect::<Vec<_>>(), vec![ent0, ent1, ent2]);
    ent0.kill_now();
    assert_eq!(Entity::iter().collect::<Vec<_>>(), vec![ent1, ent2]);
    ent1.kill_now();
    assert_eq!(Entity::iter().collect::<Vec<_>>(), vec![ent2]);
    ent2.kill_now();
    assert_eq!(Entity::iter().collect::<Vec<_>>(), vec![]);
}
