use std::ops::{Deref, DerefMut};
#[repr(transparent)]
pub struct LogFlush {
    pub flush: bool,
}
#[repr(transparent)]
pub struct LogLevel {
    pub level: isize,
}
impl Deref for LogFlush {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.flush
    }
}
impl DerefMut for LogFlush {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flush
    }
}
impl Deref for LogLevel {
    type Target = isize;
    fn deref(&self) -> &Self::Target {
        &self.level
    }
}
impl DerefMut for LogLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.level
    }
}
