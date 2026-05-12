use crate::{CellTrait, Color};
use std::ffi::c_void;
#[repr(C)]
#[derive(Debug)]
pub struct Liquid {
    pub x: isize,
    pub y: isize,
    unknown1: u8,
    unknown2: u8,
    pub is_static: bool,
    unknown3: u8,
    unknown4: isize,
    unknown5: isize,
    unknown6: isize,
    pub color: Color,
    pub original_color: Color,
    lifetime: isize,
    vegetation_sprite: *mut c_void,
}
impl CellTrait for Liquid {}
