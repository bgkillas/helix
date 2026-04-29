use std::ops::{Index, IndexMut};
use std::slice;
#[repr(C)]
#[derive(Debug)]
pub struct StdVec<T> {
    pub start: *mut T,
    pub end: *mut T,
    pub cap: *mut T,
}
impl<T> StdVec<T> {
    pub fn capacity(&self) -> usize {
        unsafe { self.cap.offset_from_unsigned(self.start) }
    }
    pub fn len(&self) -> usize {
        unsafe { self.end.offset_from_unsigned(self.start) }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        let ptr = unsafe { self.start.add(index) };
        if self.end > ptr {
            unsafe { ptr.as_ref() }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let ptr = unsafe { self.start.add(index) };
        if self.end > ptr {
            unsafe { ptr.as_mut() }
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
impl<T> Index<usize> for StdVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<T> IndexMut<usize> for StdVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}
impl<T> AsRef<[T]> for StdVec<T> {
    fn as_ref(&self) -> &[T] {
        if self.start.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.start, self.len()) }
        }
    }
}
impl<T> AsMut<[T]> for StdVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.start, self.len()) }
    }
}
