#[repr(C)]
#[derive(Debug)]
pub struct BitSet<const N: usize>(pub [isize; N]);
