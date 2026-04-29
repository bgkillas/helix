use crate::*;
use std::cmp::Ordering;
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
impl<K: Ord, V: Copy> StdMap<K, V> {
    pub fn get(&self, key: &K) -> Option<V> {
        let mut node = self.root.parent;
        loop {
            let next = match key.cmp(&node.key) {
                Ordering::Less => node.left,
                Ordering::Greater => node.right,
                Ordering::Equal => return Some(node.value),
            };
            if next.ptr == self.root.ptr {
                return None;
            }
            node = next;
        }
    }
}
