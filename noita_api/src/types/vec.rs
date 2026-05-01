use std::ops::{Deref, DerefMut};
use std::slice;
#[repr(C)]
#[derive(Debug)]
pub struct StdVec<T> {
    pub start: *mut T,
    pub end: *mut T,
    pub cap: *mut T,
}
impl<T> StdVec<T> {
    #[must_use]
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { self.cap.offset_from_unsigned(self.start) }
    }
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.end.offset_from_unsigned(self.start) }
    }
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
impl<T> Deref for StdVec<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &Self::Target {
        if self.start.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.start, self.len()) }
        }
    }
}
impl<T> DerefMut for StdVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.start.is_null() {
            &mut []
        } else {
            unsafe { slice::from_raw_parts_mut(self.start, self.len()) }
        }
    }
}
