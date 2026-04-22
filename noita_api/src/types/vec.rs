#[repr(C)]
#[derive(Debug)]
pub struct StdVec<T> {
    pub start: *mut T,
    pub end: *mut T,
    pub cap: *mut T,
}
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Vec2i {
    pub x: isize,
    pub y: isize,
}
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct ValueRange {
    pub min: f32,
    pub max: f32,
}
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct ValueRangeInt {
    pub min: isize,
    pub max: isize,
}
