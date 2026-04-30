use crate::assert_size;
#[repr(C)]
#[assert_size(0x10)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AABB {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}
#[repr(C)]
#[assert_size(0x10)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy)]
pub struct IAABB {
    pub top_left: Vec2i,
    pub bottom_right: Vec2i,
}
#[repr(C)]
#[assert_size(0x8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[assert_size(0x8)]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Vec2i {
    pub x: isize,
    pub y: isize,
}
#[repr(C)]
#[assert_size(0x8)]
#[derive(Debug, Default, Clone)]
pub struct ValueRange {
    pub min: f32,
    pub max: f32,
}
#[repr(C)]
#[assert_size(0x8)]
#[derive(Debug, Default, Clone)]
pub struct ValueRangeInt {
    pub min: isize,
    pub max: isize,
}
