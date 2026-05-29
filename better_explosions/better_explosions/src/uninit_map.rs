use std::alloc::{Allocator as _, Global, Layout};
use std::arch::asm;
use std::ptr::NonNull;
pub struct UninitMap<T> {
    indices: NonNull<[usize]>,
    list: Vec<(usize, T)>,
}
impl<T: Copy> UninitMap<T> {
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
    pub fn insert(&mut self, index: usize, value: T) {
        let slice = unsafe { self.indices.as_uninit_slice_mut() };
        slice[index].write(self.list.len());
        self.list.push((index, value));
    }
    pub fn get(&self, index: usize) -> Option<T> {
        let slice = unsafe { self.indices.as_uninit_slice_mut() };
        let mut val = slice[index];
        unsafe {
            asm!(
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
}
impl<T> Drop for UninitMap<T> {
    fn drop(&mut self) {
        unsafe {
            Global.deallocate(
                self.indices.cast(),
                Layout::array::<usize>(self.indices.len()).unwrap(),
            );
        }
    }
}
