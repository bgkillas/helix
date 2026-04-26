use std::ops::{Deref, DerefMut};
#[repr(transparent)]
pub struct LogPrint {
    pub do_print: bool,
}
impl Deref for LogPrint {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.do_print
    }
}
impl DerefMut for LogPrint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.do_print
    }
}
