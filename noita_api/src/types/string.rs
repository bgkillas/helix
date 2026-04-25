use crate::alloc::StdPtr;
use std::fmt::{Debug, Formatter};
use std::slice;
#[repr(C)]
union Buffer {
    buffer: StdPtr<u8>,
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
impl StdString {
    pub fn free(self) {
        if self.capacity > 16 {
            unsafe { self.buffer.buffer }.free_array(self.capacity)
        }
    }
}
impl From<&str> for StdString {
    fn from(value: &str) -> Self {
        let mut res = StdString {
            buffer: Default::default(),
            capacity: value.len(),
            size: value.len(),
        };
        if res.capacity > 16 {
            let buffer = StdPtr::malloc_array(value.len());
            let mut ptr = buffer.ptr;
            for b in value.as_bytes().iter().copied() {
                unsafe {
                    ptr.write(b);
                    ptr = ptr.offset(1)
                }
            }
            res.buffer.buffer = buffer;
        } else {
            let mut iter = value.as_bytes().iter().copied();
            res.buffer.sso_buffer = std::array::from_fn(|_| iter.next().unwrap_or(0))
        }
        res
    }
}
impl StdString {
    pub fn as_str(&self) -> &str {
        let ptr = if self.capacity > 16 {
            unsafe { self.buffer.buffer.as_ptr() }
        } else {
            unsafe { self.buffer.sso_buffer.as_ptr() }
        };
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, self.size)) }
    }
}
#[test]
fn test_stdstring() {
    let str = "abcdefghijklmnopqrstuvwxyz";
    let std = StdString::from(str);
    assert_eq!(str, std.as_str());
    std.free();
    let str = "abcdef";
    let std = StdString::from(str);
    assert_eq!(str, std.as_str());
    std.free();
}
