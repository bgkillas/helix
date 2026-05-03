use std::ops::{Deref, DerefMut};
#[repr(transparent)]
pub struct NewGameCount {
    pub count: usize,
}
impl Deref for NewGameCount {
    type Target = usize;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.count
    }
}
impl DerefMut for NewGameCount {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.count
    }
}
