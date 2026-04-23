use crate::alloc::{StdBox, StdBoxConst};
use crate::types::game_global;
use std::ops::{Deref, DerefMut};
pub static mut GAMEGLOBAL: StdBoxConst<StdBox<game_global::GameGlobal>> =
    StdBoxConst::new(0x0122374c);
#[repr(transparent)]
pub struct GameGlobal {
    ptr: StdBox<game_global::GameGlobal>,
}
impl Default for GameGlobal {
    fn default() -> Self {
        #[allow(static_mut_refs)]
        let ptr = unsafe { GAMEGLOBAL.read() };
        Self { ptr }
    }
}
impl Deref for GameGlobal {
    type Target = game_global::GameGlobal;
    fn deref(&self) -> &Self::Target {
        self.ptr.deref()
    }
}
impl DerefMut for GameGlobal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ptr.deref_mut()
    }
}
