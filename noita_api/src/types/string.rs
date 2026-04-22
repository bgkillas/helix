use std::fmt::{Debug, Formatter};
#[repr(C)]
union Buffer {
    buffer: *const u8,
    sso_buffer: [u8; 16],
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            sso_buffer: [0; 16],
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct StdString {
    buffer: Buffer,
    size: usize,
    capacity: usize,
}
impl Debug for StdString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdString")
            .field("buffer", &unsafe { self.buffer.sso_buffer })
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            .finish()
    }
}
