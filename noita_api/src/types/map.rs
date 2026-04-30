use crate::StdBox;
use std::cmp::Ordering;
use std::ops::Deref;
#[repr(C)]
#[derive(Debug)]
pub struct StdMap<K, V> {
    pub root: StdBox<StdMapNode<K, V>>,
    pub len: usize,
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
impl<L: ?Sized + Ord, K: Deref<Target = L>, V> StdMap<K, V> {
    pub fn get(&self, key: &L) -> Option<&V> {
        let mut node = self.root.parent;
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
