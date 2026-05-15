use crate::StdPtr;
use noita_api_macros::assert_size;
use std::cmp::Ordering;
use std::ffi::{CStr, c_char};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::slice;
#[repr(C)]
#[assert_size(0x10)]
union Buffer {
    buffer: StdPtr<u8>,
    sso_array: [u8; 16],
}
#[repr(C)]
#[assert_size(0x18)]
pub struct StdStringRef<'a> {
    buffer: Buffer,
    size: usize,
    capacity: usize,
    lifetime: PhantomData<&'a u8>,
}
impl Eq for StdStringRef<'_> {}
impl PartialEq<Self> for StdStringRef<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialOrd<Self> for StdStringRef<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for StdStringRef<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}
impl Default for StdString {
    #[inline]
    fn default() -> Self {
        Self {
            buffer: Buffer { sso_array: [0; 16] },
            size: 0,
            capacity: 16,
            lifetime: PhantomData,
        }
    }
}
pub type StdString = StdStringRef<'static>;
impl Debug for StdStringRef<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}
impl Drop for StdStringRef<'_> {
    #[inline]
    fn drop(&mut self) {
        if self.capacity != usize::MAX {
            self.free();
        }
    }
}
impl From<&str> for StdStringRef<'static> {
    #[inline]
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
            capacity: value.len().max(16),
            size: value.len(),
            lifetime: PhantomData,
        }
    }
}
impl<'a> StdStringRef<'a> {
    #[inline]
    pub fn clear(&mut self) {
        self.size = 0;
    }
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        let new = s.len() + self.size;
        if new > self.capacity {
            let new_buffer = StdPtr::malloc_array(new);
            let slice = unsafe { slice::from_raw_parts_mut(new_buffer.as_ptr(), self.size) };
            slice.copy_from_slice(self.as_bytes());
            let slice_s =
                unsafe { slice::from_raw_parts_mut(new_buffer.as_ptr().add(self.size), s.len()) };
            slice_s.copy_from_slice(s.as_bytes());
            if self.capacity > 16 {
                self.free();
            }
            self.capacity = new;
            self.buffer.buffer = new_buffer;
        } else {
            unsafe { &mut self.buffer.sso_array[self.size..self.size + s.len()] }
                .copy_from_slice(s.as_bytes());
        }
        self.size = new;
    }
    #[inline]
    pub fn free(&mut self) {
        if self.capacity > 16 {
            unsafe { self.buffer.buffer }.free_array(self.capacity);
        }
    }
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        let ptr = if self.capacity > 16 {
            unsafe { self.buffer.buffer.as_ptr() }
        } else {
            unsafe { self.buffer.sso_array.as_ptr() }
        };
        unsafe { str::from_utf8(slice::from_raw_parts(ptr, self.size)).unwrap() }
    }
    #[must_use]
    #[inline]
    pub fn as_str_mut(&mut self) -> &mut str {
        let ptr = if self.capacity > 16 {
            unsafe { self.buffer.buffer.as_ptr() }
        } else {
            unsafe { self.buffer.sso_array.as_mut_ptr() }
        };
        unsafe { str::from_utf8_mut(slice::from_raw_parts_mut(ptr, self.size)).unwrap() }
    }
    #[must_use]
    #[inline]
    pub(crate) unsafe fn no_alloc(value: &'a str) -> Self {
        let buffer = unsafe {
            Buffer {
                buffer: StdPtr::new_ptr(value.as_ptr().cast_mut()),
            }
        };
        Self {
            buffer,
            capacity: usize::MAX,
            size: value.len(),
            lifetime: PhantomData,
        }
    }
}
impl Deref for StdStringRef<'_> {
    type Target = str;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
impl DerefMut for StdStringRef<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_str_mut()
    }
}
#[test]
fn test_stdstring() {
    let str = "abcdefghijklmnopqrstuvwxyz";
    let std = StdString::from(str);
    assert_eq!(str, std.as_str());
    let str = "abcdef";
    let mut std = StdString::from(str);
    assert_eq!(str, std.as_str());
    std.push_str("ghijklmnopqrstuvwxyz");
    assert_eq!("abcdefghijklmnopqrstuvwxyz", std.as_str());
    let str = "abcdef";
    let mut std = StdString::from(str);
    std.push_str("ghi");
    assert_eq!("abcdefghi", std.as_str());
    let str = "abcdefghijklmnopqrstuvwxyz";
    let mut std = StdString::from(str);
    std.push_str("abcdefghijklmnopqrstuvwxyz");
    assert_eq!(
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz",
        std.as_str()
    );
    unsafe {
        let str = "abcdefghijklmnopqrstuvwxyz";
        let std = StdStringRef::no_alloc(str);
        assert_eq!(str, std.as_str());
        let str = "abcdef";
        let std = StdStringRef::no_alloc(str);
        assert_eq!(str, std.as_str());
    }
}
#[derive(Default, Copy, Clone)]
pub struct CStrPtr {
    pub ptr: *const c_char,
}
impl CStrPtr {
    #[inline]
    #[must_use]
    pub fn as_cstr<'a>(self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}
impl Debug for CStrPtr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            unsafe { CStr::from_ptr(self.ptr) }.to_str().unwrap()
        )
    }
}
impl From<*const c_char> for CStrPtr {
    #[inline]
    fn from(value: *const c_char) -> Self {
        Self { ptr: value }
    }
}
