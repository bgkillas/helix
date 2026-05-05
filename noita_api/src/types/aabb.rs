use noita_api_macros::assert_size_with;
#[repr(C)]
#[assert_size_with(0x10, f32)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AABB<T> {
    pub top_left: Vec2<T>,
    pub bottom_right: Vec2<T>,
}
#[repr(C)]
#[assert_size_with(0x8, isize)]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
#[repr(C)]
#[assert_size_with(0x8, f32)]
#[derive(Debug, Default, Clone)]
pub struct ValueRange<T> {
    pub min: T,
    pub max: T,
}
#[derive(Debug, Default)]
#[repr(C)]
#[assert_size_with(0x8, bool)]
#[assert_size_with(0xc, f32)]
pub struct LensValue<T> {
    pub value: T,
    pub valueb: T,
    pub frame: isize,
}
