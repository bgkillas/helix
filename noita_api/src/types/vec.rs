#[repr(C)]
#[derive(Debug)]
pub struct StdVec<T> {
    pub start: *mut T,
    pub end: *mut T,
    pub cap: *mut T,
}
