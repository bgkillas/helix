use crate::CellTrait;
#[repr(C)]
#[derive(Debug)]
pub struct Fire {
    pub x: isize,
    pub y: isize,
    pub lifetime: isize,
    unknown: isize,
}
impl CellTrait for Fire {}
