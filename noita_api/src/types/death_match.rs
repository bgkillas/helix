#[repr(C)]
#[derive(Debug)]
pub struct DeathMatch {
    vtables: [*const usize; 7],
    unk: [*const usize; 45],
}
