use std::ops::{Deref, DerefMut};
#[repr(transparent)]
pub struct WorldSeed {
    pub seed: usize,
}
impl Deref for WorldSeed {
    type Target = usize;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.seed
    }
}
impl DerefMut for WorldSeed {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seed
    }
}
