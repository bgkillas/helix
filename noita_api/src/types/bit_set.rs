use crate::*;
#[repr(C)]
#[assert_size_with(0x4, 1)]
#[derive(Debug)]
pub struct BitSet<const N: usize>(pub [isize; N]);
