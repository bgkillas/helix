use crate::Vec2;
#[repr(C)]
#[derive(Debug)]
pub struct Transform {
    pub pos: Vec2,
    pub angle: Vec2,
    pub rot90: Vec2,
    pub scale: Vec2,
}
