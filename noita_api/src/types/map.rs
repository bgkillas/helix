use crate::StdBox;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
#[repr(C)]
pub struct StdMap<K, V> {
    pub root: StdBox<StdMapNode<K, V>>,
    pub len: usize,
}
impl<K: Default, V: Default> Default for StdMap<K, V> {
    #[inline]
    fn default() -> Self {
        let mut root = StdBox::new(UninitStdMapNode::<K, V>::default());
        root.left = Some(root);
        root.parent = Some(root);
        root.right = Some(root);
        Self {
            root: root.cast(),
            len: 0,
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct StdMapNode<K, V> {
    pub left: StdBox<StdMapNode<K, V>>,
    pub parent: StdBox<StdMapNode<K, V>>,
    pub right: StdBox<StdMapNode<K, V>>,
    pub color: bool,
    pub end: bool,
    unk: [u8; 2],
    pub key: K,
    pub value: V,
}
#[repr(C)]
#[derive(Default)]
struct UninitStdMapNode<K, V> {
    pub left: Option<StdBox<UninitStdMapNode<K, V>>>,
    pub parent: Option<StdBox<UninitStdMapNode<K, V>>>,
    pub right: Option<StdBox<UninitStdMapNode<K, V>>>,
    pub color: bool,
    pub end: bool,
    unk: [u8; 2],
    pub key: K,
    pub value: V,
}
impl<K: Debug, V: Debug> Debug for StdMap<K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StdMap")
            .field(&self.iter().collect::<Vec<_>>())
            .finish()
    }
}
#[derive(Debug)]
pub struct StdMapIter<'a, K, V> {
    pub root: StdBox<StdMapNode<K, V>>,
    pub current: StdBox<StdMapNode<K, V>>,
    pub parents: Vec<StdBox<StdMapNode<K, V>>>,
    phantom: PhantomData<(&'a K, &'a V)>,
}
impl<K, V> StdMap<K, V> {
    #[inline]
    #[must_use]
    pub fn iter(&self) -> StdMapIter<'_, K, V> {
        StdMapIter {
            root: self.root,
            current: self.root.parent,
            parents: Vec::with_capacity(self.len),
            phantom: PhantomData,
        }
    }
}
impl<'a, K, V> IntoIterator for &'a StdMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = StdMapIter<'a, K, V>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, K, V> Iterator for StdMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.root {
            return None;
        }
        let tag = self.current;
        self.current = if tag.right != self.root {
            if tag.left == self.root {
                tag.right
            } else {
                self.parents.push(tag.right);
                tag.left
            }
        } else if tag.left == self.root {
            self.parents.pop().unwrap_or(self.root)
        } else {
            tag.left
        };
        let tag_ref = tag.as_ref();
        Some((&tag_ref.key, &tag_ref.value))
    }
}
impl<L: ?Sized + Ord, K: Deref<Target = L>, V> StdMap<K, V> {
    #[inline]
    pub fn get(&self, key: &L) -> Option<&V> {
        let mut node = self.root.parent;
        if self.root.ptr == node.ptr {
            return None;
        }
        loop {
            let next = match key.cmp(&*node.key) {
                Ordering::Less => node.left,
                Ordering::Greater => node.right,
                Ordering::Equal => return Some(&node.as_ref().value),
            };
            if next.ptr == self.root.ptr {
                return None;
            }
            node = next;
        }
    }
}
