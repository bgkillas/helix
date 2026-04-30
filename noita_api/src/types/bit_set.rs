use crate::assert_size_with;
#[repr(C)]
#[assert_size_with(0x4, 1)]
#[assert_size_with(0x10, 4)]
#[derive(Debug)]
pub struct BitSet<const N: usize>(pub [isize; N]);
