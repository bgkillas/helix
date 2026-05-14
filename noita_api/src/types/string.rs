use crate::StdPtr;
use noita_api_macros::assert_size;
use std::cmp::Ordering;
use std::ffi::{CStr, c_char};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
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
            capacity: 0,
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
        self.free();
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
            capacity: value.len(),
            size: value.len(),
            lifetime: PhantomData,
        }
    }
}
impl<'a> StdStringRef<'a> {
    #[inline]
    pub fn free(&mut self) {
        if self.capacity > 16 && self.capacity != usize::MAX {
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
#[test]
fn test_stdstring() {
    let str = "abcdefghijklmnopqrstuvwxyz";
    let std = StdString::from(str);
    assert_eq!(str, std.as_str());
    let str = "abcdef";
    let std = StdString::from(str);
    assert_eq!(str, std.as_str());
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
