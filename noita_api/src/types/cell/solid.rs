use crate::CellTrait;
#[repr(C)]
#[derive(Debug)]
pub struct Solid {
    unk: [usize; 20],
}
impl CellTrait for Solid {}
