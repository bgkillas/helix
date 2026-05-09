use crate::StdPtr;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{ptr, slice};
#[repr(C)]
pub struct StdVec<T> {
    pub start: *mut T,
    pub end: *mut T,
    pub cap: *mut T,
}
impl<T: Debug> Debug for StdVec<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
impl<T> Default for StdVec<T> {
    #[inline]
    fn default() -> Self {
        Self {
            start: ptr::null_mut(),
            end: ptr::null_mut(),
            cap: ptr::null_mut(),
        }
    }
}
impl<T> Drop for StdVec<T> {
    #[inline]
    fn drop(&mut self) {
        if let Some(ptr) = NonNull::new(self.start) {
            StdPtr::from(ptr).free_array(self.capacity());
        }
    }
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
    #[inline]
    pub fn push(&mut self, value: T) {
        self.alloc(1);
        unsafe {
            self.end.write(value);
            self.end = self.end.add(1);
        }
    }
    fn alloc(&mut self, n: usize) {
        if self.capacity() < self.len() + n {
            let old_len = self.len();
            let new_cap = (old_len + n).next_power_of_two();
            let new_ptr = StdPtr::<T>::malloc_array(new_cap).as_ptr();
            if old_len > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(self.start, new_ptr, old_len);
                }
            }
            if let Some(ptr) = NonNull::new(self.start) {
                StdPtr::from(ptr).free();
            }
            self.start = new_ptr;
            self.end = unsafe { new_ptr.add(old_len) };
            self.cap = unsafe { new_ptr.add(new_cap) };
        }
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
