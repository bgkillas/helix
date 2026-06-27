use std::alloc::{Allocator as _, Global, Layout};
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::NonNull;
pub struct UninitMap<T> {
    indices: NonNull<[usize]>,
    list: Vec<(usize, T)>,
}
impl<T> UninitMap<T> {
    #[inline]
    #[must_use]
    pub fn new(n: usize) -> Self {
        let alloc = Global
            .allocate(Layout::array::<usize>(n).unwrap())
            .unwrap()
            .as_ptr();
        Self {
            #[allow(clippy::cast_ptr_alignment)]
            indices: NonNull::new(ptr::slice_from_raw_parts_mut(
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
    pub fn get(&self, index: usize) -> Option<&T> {
        let slice = unsafe { self.indices.as_uninit_slice_mut() };
        #[cfg(not(target_family = "wasm"))]
        let mut val = slice[index];
        #[cfg(target_family = "wasm")]
        let val = slice[index];
        #[cfg(not(target_family = "wasm"))]
        unsafe {
            std::arch::asm!(
                "/* {} */",
                inout(reg) * val.as_mut_ptr(),
                options(nostack, pure, nomem)
            );
        }
        let list_index = unsafe { val.assume_init() };
        if let Some((indice_index, val)) = self.list.get(list_index)
            && *indice_index == index
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
    #[inline]
    pub fn clear(&mut self) {
        self.list.clear();
    }
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = usize> {
        self.list.iter().map(|(i, _)| *i)
    }
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.list.iter().map(|(_, v)| v)
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.list.iter().map(|(a, b)| (*a, b))
    }
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.list.iter_mut().map(|(_, v)| v)
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.list.iter_mut().map(|(a, b)| (*a, b))
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
impl<T, const N: usize> Default for UninitMapArray<T, N> {
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
impl<T, const N: usize> UninitMapArray<T, N> {
    #[inline]
    #[must_use]
    pub fn default_box() -> Box<Self> {
        let mut res = Box::<UninitMapArray<T, N>>::new_uninit();
        unsafe {
            (*res.as_mut_ptr()).len = 0;
            res.assume_init()
        }
    }
    #[inline]
    pub fn insert(&mut self, index: usize, value: T) {
        self.indices[index].write(self.len);
        self.list[self.len].write((index, value));
        self.len += 1;
    }
    #[cfg(not(miri))]
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        #[cfg(not(target_family = "wasm"))]
        let mut val = self.indices[index];
        #[cfg(target_family = "wasm")]
        let val = self.indices[index];
        #[cfg(not(target_family = "wasm"))]
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
        let (indice_index, val) = unsafe { self.list[list_index].assume_init_ref() };
        if *indice_index == index {
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
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = usize> {
        self.list[..self.len]
            .iter()
            .map(|m| unsafe { m.assume_init_ref() })
            .map(|(i, _)| *i)
    }
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.list[..self.len]
            .iter()
            .map(|m| unsafe { m.assume_init_ref() })
            .map(|(_, v)| v)
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.list[..self.len]
            .iter()
            .map(|m| unsafe { m.assume_init_ref() })
            .map(|(a, b)| (*a, b))
    }
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.list[..self.len]
            .iter_mut()
            .map(|m| unsafe { m.assume_init_mut() })
            .map(|(_, v)| v)
    }
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.list[..self.len]
            .iter_mut()
            .map(|m| unsafe { m.assume_init_mut() })
            .map(|(a, b)| (*a, b))
    }
}
impl<T, const N: usize> Drop for UninitMapArray<T, N> {
    #[inline]
    fn drop(&mut self) {
        for v in self.values_mut() {
            unsafe {
                ptr::drop_in_place(v);
            }
        }
    }
}
