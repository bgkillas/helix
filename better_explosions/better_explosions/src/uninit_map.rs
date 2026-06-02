use std::alloc::{Allocator as _, Global, Layout};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
pub struct UninitMap<T> {
    indices: NonNull<[usize]>,
    list: Vec<(usize, T)>,
}
impl<T: Copy> UninitMap<T> {
    #[inline]
    #[must_use]
    pub fn new(n: usize) -> Self {
        let alloc = Global
            .allocate(Layout::array::<usize>(n).unwrap())
            .unwrap()
            .as_ptr();
        Self {
            #[allow(clippy::cast_ptr_alignment)]
            indices: NonNull::new(std::ptr::slice_from_raw_parts_mut(
                alloc.as_mut_ptr().cast::<usize>(),
                n,
            ))
            .unwrap(),
            list: Vec::with_capacity(n),
        }
    }
    #[inline]
    pub fn insert(&mut self, index: usize, value: T) {
        let slice = unsafe { self.indices.as_uninit_slice_mut() };
        slice[index].write(self.list.len());
        self.list.push((index, value));
    }
    #[cfg(not(miri))]
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T> {
        let slice = unsafe { self.indices.as_uninit_slice_mut() };
        let mut val = slice[index];
        unsafe {
            std::arch::asm!(
                "/* {} */",
                inout(reg) * val.as_mut_ptr(),
                options(nostack, pure, nomem)
            );
        }
        let list_index = unsafe { val.assume_init() };
        if let Some((indice_index, val)) = self.list.get(list_index).copied()
            && indice_index == index
        {
            Some(val)
        } else {
            None
        }
    }
    #[cfg(miri)]
    #[inline]
    #[must_use]
    pub fn get(&self, _: usize) -> Option<T> {
        None
    }
}
impl<T> Drop for UninitMap<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            Global.deallocate(
                self.indices.cast(),
                Layout::array::<usize>(self.indices.len()).unwrap(),
            );
        }
    }
}
pub struct UninitMapArray<T, const N: usize> {
    indices: [MaybeUninit<usize>; N],
    list: [MaybeUninit<(usize, T)>; N],
    len: usize,
}
impl<T: Copy, const N: usize> Default for UninitMapArray<T, N> {
    #[inline]
    fn default() -> Self {
        Self {
            #[allow(clippy::cast_ptr_alignment)]
            indices: unsafe { MaybeUninit::uninit().assume_init() },
            list: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }
}
impl<T: Copy, const N: usize> UninitMapArray<T, N> {
    #[inline]
    pub fn insert(&mut self, index: usize, value: T) {
        self.indices[index].write(self.len);
        self.list[self.len].write((index, value));
        self.len += 1;
    }
    #[cfg(not(miri))]
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T> {
        let mut val = self.indices[index];
        unsafe {
            std::arch::asm!(
                "/* {} */",
                inout(reg) * val.as_mut_ptr(),
                options(nostack, pure, nomem)
            );
        }
        let list_index = unsafe { val.assume_init() };
        if list_index >= self.len {
            return None;
        }
        let (indice_index, val) = unsafe { self.list[list_index].assume_init() };
        if indice_index == index {
            Some(val)
        } else {
            None
        }
    }
    #[cfg(miri)]
    #[inline]
    #[must_use]
    pub fn get(&self, _: usize) -> Option<T> {
        None
    }
}
