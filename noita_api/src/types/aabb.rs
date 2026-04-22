use crate::types::vec::{Vec2, Vec2i};
#[repr(C)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AABB {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}

#[repr(C)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default)]
pub struct IAABB {
    pub top_left: Vec2i,
    pub bottom_right: Vec2i,
}
