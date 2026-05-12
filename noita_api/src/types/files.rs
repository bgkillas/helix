use crate::{StdString, StdVec};
use std::ops::{Deref, DerefMut};
#[repr(transparent)]
#[derive(Debug)]
pub struct FileNames {
    file_names: StdVec<StdString>,
}
impl Default for FileNames {
    #[inline]
    fn default() -> Self {
        todo!()
    }
}
impl Deref for FileNames {
    type Target = [StdString];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.file_names
    }
}
impl DerefMut for FileNames {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file_names
    }
}
