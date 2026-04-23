use crate::alloc::{StdBox, StdPtr};
use crate::types::game_global;
use std::ops::{Deref, DerefMut};
const GAME_GLOBAL: StdPtr<StdPtr<game_global::GameGlobal>> = StdPtr::new(0x0122374c);
const WORLD_SEED: StdPtr<usize> = StdPtr::new(0x01205004);
#[repr(transparent)]
pub struct GameGlobal {
    ptr: StdBox<game_global::GameGlobal>,
}
impl Default for GameGlobal {
    fn default() -> Self {
        let ptr = unsafe { GAME_GLOBAL.read() };
        let ptr = StdBox::from(ptr);
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
#[repr(transparent)]
pub struct WorldSeed {
    ptr: StdBox<usize>,
}
impl Default for WorldSeed {
    fn default() -> Self {
        let ptr = WORLD_SEED;
        let ptr = StdBox::from(ptr);
        Self { ptr }
    }
}
impl Deref for WorldSeed {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        self.ptr.deref()
    }
}
impl DerefMut for WorldSeed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ptr.deref_mut()
    }
}
