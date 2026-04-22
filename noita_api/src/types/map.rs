#[repr(C)]
#[derive(Debug)]
pub struct StdMap<K: 'static, V: 'static> {
    pub root: &'static mut StdMapNode<K, V>,
    pub len: usize,
}
#[repr(C)]
#[derive(Debug)]
pub struct StdMapNode<K, V> {
    pub left: *mut StdMapNode<K, V>,
    pub parent: *mut StdMapNode<K, V>,
    pub right: *mut StdMapNode<K, V>,
    pub color: bool,
    pub end: bool,
    unk: [u8; 2],
    pub key: K,
    pub value: V,
}
