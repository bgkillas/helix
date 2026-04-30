use crate::StdPtr;
use noita_api_macros::assert_size;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::slice;
#[repr(C)]
#[assert_size(0x10)]
union Buffer {
    buffer: StdPtr<u8>,
    sso_array: [u8; 16],
}
#[repr(C)]
#[assert_size(0x18)]
pub struct StdString {
    buffer: Buffer,
    size: usize,
    capacity: usize,
}
impl Debug for StdString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdString")
            .field("value", &self.as_str())
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            .finish()
    }
}
impl StdString {
    pub fn free(self) {
        if self.capacity > 16 {
            unsafe { self.buffer.buffer }.free_array(self.capacity);
        }
    }
}
impl From<&str> for StdString {
    fn from(value: &str) -> Self {
        let buffer = if value.len() > 16 {
            let buffer = StdPtr::malloc_array(value.len());
            let slice = unsafe { slice::from_raw_parts_mut(buffer.as_ptr(), value.len()) };
            slice.copy_from_slice(value.as_bytes());
            Buffer { buffer }
        } else {
            let mut iter = value.as_bytes().iter().copied();
            let sso_array = std::array::from_fn(|_| iter.next().unwrap_or(0));
            Buffer { sso_array }
        };
        Self {
            buffer,
            capacity: value.len(),
            size: value.len(),
        }
    }
}
impl StdString {
    #[must_use]
    pub fn as_str(&self) -> &str {
        let ptr = if self.capacity > 16 {
            unsafe { self.buffer.buffer.as_ptr() }
        } else {
            unsafe { self.buffer.sso_array.as_ptr() }
        };
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, self.size)) }
    }
    #[must_use]
    pub fn no_alloc(value: &str) -> Self {
        let buffer = unsafe { StdPtr::new_ptr(value.as_ptr().cast_mut()) };
        let buffer = Buffer { buffer };
        Self {
            buffer,
            capacity: value.len().max(32),
            size: value.len(),
        }
    }
}
impl Deref for StdString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
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
