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
#[derive(Debug, Clone, Copy)]
pub struct StdPtr<T: Sized> {
    ptr: NonNull<T>,
}
#[repr(transparent)]
pub struct StdBoxOwned<T: Sized> {
    ptr: StdPtr<T>,
    phantom_data: PhantomData<T>,
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct StdBox<T: Sized> {
    ptr: StdPtr<T>,
}
#[repr(transparent)]
pub struct StdBoxConst<T: Sized> {
    ptr: StdPtr<T>,
}
impl<T: Sized> StdBoxConst<T> {
    pub const fn new(ptr: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(ptr as *mut c_uint).cast() };
        let ptr = StdPtr { ptr };
        Self { ptr }
    }
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
}
impl<T: Sized> StdBox<T> {
    pub fn free(mut self) {
        self.ptr.free()
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
impl<T: Sized> Deref for StdBoxConst<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
impl<T: Sized> DerefMut for StdBoxConst<T> {
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
impl<T: Sized + Debug> Debug for StdBoxConst<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}
impl<T: Sized + Debug> Debug for StdBoxOwned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}
