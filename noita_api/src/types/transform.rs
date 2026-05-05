use crate::Vec2;
#[repr(C)]
#[derive(Debug)]
pub struct Transform {
    pub pos: Vec2<f32>,
    pub angle: Vec2<f32>,
    pub rot90: Vec2<f32>,
    pub scale: Vec2<f32>,
}
