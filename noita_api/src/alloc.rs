use std::ffi::{c_uint, c_void};
use std::fmt::{Debug, Formatter};
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
#[derive(Debug)]
pub struct StdPtr<T: Sized> {
    ptr: NonNull<T>,
}
#[repr(transparent)]
pub struct StdBoxOwned<T: Sized> {
    ptr: StdPtr<T>,
    phantom_data: PhantomData<T>,
}
#[repr(transparent)]
pub struct StdBox<T: Sized> {
    ptr: StdPtr<T>,
}
impl<T: Sized> StdPtr<T> {
    pub fn malloc() -> Self {
        let ptr = unsafe {
            NonNull::new_unchecked((MSVCR.operator_new)(size_of::<T>() as c_uint).cast())
        };
        Self { ptr }
    }
    pub fn free(&mut self) {
        unsafe { (MSVCR.operator_delete)(self.ptr.as_ptr().cast()) }
    }
    pub const fn new(ptr: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(ptr as *mut _) };
        Self { ptr }
    }
}
impl<T: Sized> StdBox<T> {
    pub fn free(mut self) {
        self.ptr.free()
    }
    pub fn read(self) -> T {
        unsafe { self.ptr.read() }
    }
}
impl<T: Sized> From<StdPtr<T>> for StdBox<T> {
    fn from(ptr: StdPtr<T>) -> Self {
        Self { ptr }
    }
}
impl<T: Sized> From<NonNull<T>> for StdPtr<T> {
    fn from(ptr: NonNull<T>) -> Self {
        Self { ptr }
    }
}
impl<T: Sized> StdBoxOwned<T> {
    pub fn free(mut self) {
        self.ptr.free()
    }
    pub fn new(value: T) -> Self {
        let ptr = StdPtr::malloc();
        unsafe {
            ptr.write(value);
        }
        Self {
            ptr,
            phantom_data: PhantomData,
        }
    }
    pub fn read(self) -> T {
        unsafe { self.ptr.read() }
    }
}
impl<T: Sized> Copy for StdPtr<T> {}
impl<T: Sized> Copy for StdBox<T> {}
impl<T: Sized> Clone for StdPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Sized> Clone for StdBox<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Sized> Deref for StdBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T: Sized> DerefMut for StdBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}
impl<T: Sized> Deref for StdBoxOwned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T: Sized> DerefMut for StdBoxOwned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}
impl<T: Sized> Deref for StdPtr<T> {
    type Target = NonNull<T>;
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
impl<T: Sized> DerefMut for StdPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}
impl<T: Sized> From<StdBoxOwned<T>> for StdBox<T> {
    fn from(value: StdBoxOwned<T>) -> Self {
        Self { ptr: value.ptr }
    }
}
impl<T: Sized + Debug> Debug for StdBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}
impl<T: Sized + Debug> Debug for StdBoxOwned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}
