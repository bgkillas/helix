use std::ops::{Deref, DerefMut};
#[repr(transparent)]
pub struct WorldSeed {
    pub seed: usize,
}
impl Deref for WorldSeed {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.seed
    }
}
impl DerefMut for WorldSeed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seed
    }
}
