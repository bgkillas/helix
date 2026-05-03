#[derive(Debug, Default)]
#[repr(C)]
pub struct DebugSettings {
    vftable: *const usize,
    unk: [*const usize; 12],
}
