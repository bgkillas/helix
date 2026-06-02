use crate::StdBox;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum RBColor {
    Red = 0,
    #[default]
    Black = 1,
}
#[repr(C)]
pub struct StdMap<K, V> {
    pub root: StdBox<StdMapNode<K, V>>,
    pub len: usize,
}
impl<K: Default, V: Default> Default for StdMap<K, V> {
    #[inline]
    fn default() -> Self {
        let mut root: StdBox<UninitStdMapNode<K, V>> = StdBox::default();
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
    pub color: RBColor,
    pub end: bool,
    pub key: K,
    pub value: V,
}
impl<K: Ord, V> StdMap<K, V> {
    #[inline]
    pub fn insert(&mut self, k: K, v: V) {
        self.len += 1;
        let mut node = StdMapNode {
            left: self.root,
            parent: self.root,
            right: self.root,
            color: RBColor::default(),
            end: true,
            key: k,
            value: v,
        };
        let mut y = self.root;
        let mut x = self.root.parent;
        while x.ptr != self.root.ptr {
            y = x;
            x = if node.key < x.key { x.left } else { x.right };
        }
        node.parent = y;
        if y.ptr == self.root.ptr {
            self.root.parent = StdBox::new(node);
            self.insert_fixup(self.root.parent);
        } else {
            node.color = RBColor::Red;
            if node.key < y.key {
                y.left = StdBox::new(node);
                self.insert_fixup(y.left);
            } else {
                y.right = StdBox::new(node);
                self.insert_fixup(y.right);
            }
        }
    }
    fn insert_fixup(&mut self, mut node: StdBox<StdMapNode<K, V>>) {
        let mut parent;
        let mut gparent;
        while node.parent.color == RBColor::Red {
            parent = node.parent;
            gparent = parent.parent;
            if parent == gparent.left {
                let mut uncle = gparent.right;
                if uncle.ptr != self.root.ptr && uncle.color == RBColor::Red {
                    uncle.color = RBColor::Black;
                    parent.color = RBColor::Black;
                    gparent.color = RBColor::Red;
                    node = gparent;
                    continue;
                }
                if parent.right == node {
                    self.left_rotate(parent);
                    mem::swap(&mut parent, &mut node);
                }
                parent.color = RBColor::Black;
                gparent.color = RBColor::Red;
                self.right_rotate(gparent);
            } else {
                let mut uncle = gparent.left;
                if uncle.ptr != self.root.ptr && uncle.color == RBColor::Red {
                    uncle.color = RBColor::Black;
                    parent.color = RBColor::Black;
                    gparent.color = RBColor::Red;
                    node = gparent;
                    continue;
                }
                if parent.left == node {
                    self.right_rotate(parent);
                    mem::swap(&mut parent, &mut node);
                }
                parent.color = RBColor::Black;
                gparent.color = RBColor::Red;
                self.left_rotate(gparent);
            }
        }
        self.root.parent.color = RBColor::Black;
    }
    fn left_rotate(&mut self, mut node: StdBox<StdMapNode<K, V>>) {
        let mut temp = node.right;
        node.right = temp.left;
        if temp.left.ptr != self.root.ptr {
            temp.left.parent = node;
        }
        temp.parent = node.parent;
        if node == self.root.parent {
            self.root.parent = temp;
        } else if node == node.parent.left {
            node.parent.left = temp;
        } else {
            node.parent.right = temp;
        }
        temp.left = node;
        node.parent = temp;
    }
    fn right_rotate(&mut self, mut node: StdBox<StdMapNode<K, V>>) {
        let mut temp = node.left;
        node.left = temp.right;
        if temp.right.ptr != self.root.ptr {
            temp.right.parent = node;
        }
        temp.parent = node.parent;
        if node == self.root.parent {
            self.root.parent = temp;
        } else if node == node.parent.right {
            node.parent.right = temp;
        } else {
            node.parent.left = temp;
        }
        temp.right = node;
        node.parent = temp;
    }
}
#[repr(C)]
#[derive(Default)]
struct UninitStdMapNode<K, V> {
    pub left: Option<StdBox<UninitStdMapNode<K, V>>>,
    pub parent: Option<StdBox<UninitStdMapNode<K, V>>>,
    pub right: Option<StdBox<UninitStdMapNode<K, V>>>,
    pub color: RBColor,
    pub end: bool,
    pub key: K,
    pub value: V,
}
impl<K: Debug, V: Debug> Debug for StdMap<K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
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
            let next = match key.cmp(&node.key) {
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
