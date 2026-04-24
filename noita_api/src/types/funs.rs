#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
#[cfg(target_os = "windows")]
pub struct StdCall<T, K>(pub extern "stdcall" fn(T) -> K);
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
#[cfg(not(target_os = "windows"))]
pub struct StdCall<T, K>(pub fn(T) -> K);
impl<T, K> StdCall<T, K> {
    pub fn call(&self, a: T) -> K {
        #[cfg(target_os = "windows")]
        {
            self.0(a)
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.0(a)
        }
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
#[cfg(target_os = "windows")]
pub struct FastCall<T, K>(pub extern "fastcall" fn(T) -> K);
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
#[cfg(not(target_os = "windows"))]
pub struct FastCall<T, K>(pub fn(T) -> K);
impl<T, K> FastCall<T, K> {
    pub fn call(&self, a: T) -> K {
        #[cfg(target_os = "windows")]
        {
            self.0(a)
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.0(a)
        }
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
#[cfg(target_os = "windows")]
pub struct ThisCall<T, K>(pub extern "thiscall" fn(T) -> K);
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
#[cfg(not(target_os = "windows"))]
pub struct ThisCall<T, K>(pub fn(T) -> K);
impl<T, K> ThisCall<T, K> {
    pub fn call(&self, a: T) -> K {
        #[cfg(target_os = "windows")]
        {
            self.0(a)
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.0(a)
        }
    }
}
