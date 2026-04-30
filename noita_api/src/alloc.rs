use noita_api_macros::assert_size_with;
#[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
use std::alloc::Global;
#[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
use std::alloc::{Allocator, Layout};
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
use std::ffi::{c_uint, c_void};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
struct Msvcr {
    operator_new: unsafe extern "C" fn(n: c_uint) -> *mut c_void,
    operator_delete: unsafe extern "C" fn(*mut c_void),
}
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
static MSVCR: std::sync::LazyLock<Msvcr> = std::sync::LazyLock::new(|| unsafe {
    let lib = libloading::Library::new("./msvcr120.dll").expect("library to exist");
    let operator_new = *lib.get(b"??2@YAPAXI@Z\0").expect("symbol to exist");
    let operator_delete = *lib.get(b"??3@YAXPAX@Z\0").expect("symbol to exist");
    Msvcr {
        operator_new,
        operator_delete,
    }
});
#[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
const ALLOC: Global = Global;
#[repr(transparent)]
#[assert_size_with(0x4, ())]
pub struct StdPtr<T: Sized> {
    pub ptr: NonNull<T>,
}
#[repr(transparent)]
#[assert_size_with(0x4, ())]
pub struct StdBox<T: Sized> {
    pub ptr: StdPtr<T>,
}
impl<T: Sized> StdPtr<T> {
    #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
    pub fn malloc() -> Self {
        let ptr = unsafe {
            NonNull::new_unchecked((MSVCR.operator_new)(size_of::<T>() as c_uint).cast())
        };
        Self { ptr }
    }
    #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
    pub fn malloc_array(n: usize) -> Self {
        let ptr = unsafe {
            NonNull::new_unchecked((MSVCR.operator_new)((size_of::<T>() * n) as c_uint).cast())
        };
        Self { ptr }
    }
    #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
    pub fn malloc() -> Self {
        let layout = Layout::new::<T>();
        let ptr = ALLOC.allocate(layout).unwrap().cast();
        Self { ptr }
    }
    #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
    pub fn malloc_array(n: usize) -> Self {
        let layout = Layout::array::<T>(n).unwrap();
        let ptr = ALLOC.allocate(layout).unwrap().cast();
        Self { ptr }
    }
    #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
    pub fn free(&mut self) {
        unsafe { (MSVCR.operator_delete)(self.ptr.as_ptr().cast()) }
    }
    #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
    pub fn free_array(&mut self, _: usize) {
        unsafe { (MSVCR.operator_delete)(self.ptr.as_ptr().cast()) }
    }
    #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
    pub fn free(&mut self) {
        let layout = Layout::new::<T>();
        unsafe { ALLOC.deallocate(self.ptr.cast(), layout) };
    }
    #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
    pub fn free_array(&mut self, n: usize) {
        let layout = Layout::array::<T>(n).unwrap();
        unsafe { ALLOC.deallocate(self.ptr.cast(), layout) };
    }
    pub const fn new(value: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(value as *mut T) };
        Self { ptr }
    }
    pub(crate) const unsafe fn new_ptr(value: *mut T) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(value) };
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
    pub fn new(value: T) -> Self {
        let ptr = StdPtr::malloc();
        unsafe {
            ptr.write(value);
        }
        Self { ptr }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn as_ref<'a>(self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }
    pub fn as_mut<'a>(mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }
}
impl<T> PartialEq for StdPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl<T> Debug for StdPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ptr)
    }
}
impl<T: Sized> From<StdPtr<T>> for StdBox<T> {
    fn from(ptr: StdPtr<T>) -> Self {
        Self { ptr }
    }
}
impl<T: Sized> From<StdBox<T>> for StdPtr<T> {
    fn from(ptr: StdBox<T>) -> Self {
        Self { ptr: ptr.ptr.ptr }
    }
}
impl<T: Sized> From<NonNull<T>> for StdPtr<T> {
    fn from(ptr: NonNull<T>) -> Self {
        Self { ptr }
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
        self.as_ref()
    }
}
impl<T: Sized> DerefMut for StdBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
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
impl<T: Sized + Debug> Debug for StdBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}
