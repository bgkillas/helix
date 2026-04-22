#[repr(C)]
#[derive(Debug, Default)]
pub struct StdSet<T: Default + Debug> {
    pub a: T,
    pub b: T,
}
