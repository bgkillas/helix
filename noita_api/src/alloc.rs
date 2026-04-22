use std::ffi::{c_uint, c_void};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::LazyLock;
struct Msvcr {
    operator_new: unsafe extern "C" fn(n: c_uint) -> *mut c_void,
    operator_delete: unsafe extern "C" fn(*mut c_void),
}
static MSVCR: LazyLock<Msvcr> = LazyLock::new(|| unsafe {
    let lib = libloading::Library::new("./msvcr120.dll").expect("library to exist");
    let operator_new = *lib.get(b"??2@YAPAXI@Z\0").expect("symbol to exist");
    let operator_delete = *lib.get(b"??3@YAXPAX@Z\0").expect("symbol to exist");
    Msvcr {
        operator_new,
        operator_delete,
    }
});
#[repr(transparent)]
pub struct Ptr<T: Sized> {
    ptr: NonNull<T>,
}
#[repr(transparent)]
pub struct NoitaBoxOwned<T: Sized> {
    ptr: Ptr<T>,
    phantom_data: PhantomData<T>,
}
#[repr(transparent)]
pub struct NoitaBox<T: Sized> {
    ptr: Ptr<T>,
}
impl<T: Sized> Ptr<T> {
    pub fn malloc() -> Self {
        let ptr = unsafe {
            NonNull::new_unchecked((MSVCR.operator_new)(size_of::<T>() as c_uint).cast())
        };
        Self { ptr }
    }
    pub fn free(&mut self) {
        unsafe { (MSVCR.operator_delete)(self.ptr.as_ptr().cast()) }
    }
}
impl<T: Sized> NoitaBox<T> {
    pub fn free(&mut self) {
        self.ptr.free()
    }
}
impl<T: Sized> NoitaBoxOwned<T> {
    pub fn free(&mut self) {
        self.ptr.free()
    }
    pub fn new(value: T) -> Self {
        let ptr = Ptr::malloc();
        unsafe {
            ptr.write(value);
        }
        Self {
            ptr,
            phantom_data: PhantomData,
        }
    }
}
impl<T: Sized> Deref for NoitaBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T: Sized> DerefMut for NoitaBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}
impl<T: Sized> Deref for NoitaBoxOwned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T: Sized> DerefMut for NoitaBoxOwned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}
impl<T: Sized> Deref for Ptr<T> {
    type Target = NonNull<T>;
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
impl<T: Sized> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}
impl<T: Sized> From<NoitaBoxOwned<T>> for NoitaBox<T> {
    fn from(value: NoitaBoxOwned<T>) -> Self {
        Self { ptr: value.ptr }
    }
}
