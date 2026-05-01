use std::ops::{Deref, DerefMut};
#[repr(transparent)]
pub struct GameMode {
    pub mode: usize,
}
impl Deref for GameMode {
    type Target = usize;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.mode
    }
}
impl DerefMut for GameMode {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mode
    }
}
