use crate::*;
use noita_api_macros::assert_size;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::slice;
#[repr(C)]
#[assert_size(0x10)]
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
            let slice = unsafe { slice::from_raw_parts_mut(buffer.as_ptr(), value.len()) };
            slice.copy_from_slice(value.as_bytes());
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
    pub fn get(&self, index: usize) -> u8 {
        unsafe {
            if self.capacity <= 16 {
                self.buffer.sso_buffer[index]
            } else {
                self.buffer.buffer.add(index).read()
            }
        }
    }
}
impl Ord for StdString {
    fn cmp(&self, other: &Self) -> Ordering {
        let smallest = self.size.min(other.size);
        for i in 0..smallest {
            match self.get(i).cmp(&other.get(i)) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        self.size.cmp(&other.size)
    }
}
impl Eq for StdString {}
impl PartialEq for StdString {
    fn eq(&self, other: &Self) -> bool {
        if self.size == other.size {
            self.as_str() == other.as_str()
        } else {
            false
        }
    }
}
impl PartialOrd for StdString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
