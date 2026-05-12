use crate::{CellTrait, Color};
#[repr(C)]
#[derive(Debug)]
pub struct Gas {
    unknown5: isize,
    unknown6: isize,
    pub x: isize,
    pub y: isize,
    unknown1: u8,
    unknown2: u8,
    unknown3: u8,
    unknown4: u8,
    pub color: Color,
    unknown7: isize,
    unknown8: isize,
    lifetime: isize,
}
impl CellTrait for Gas {}
