use crate::alloc::{StdBox, StdPtr};
use crate::types::game_global;
use noita_api_macros::generate_global;
use std::ops::{Deref, DerefMut};
#[generate_global]
const GAME_GLOBAL: StdPtr<StdPtr<game_global::GameGlobal>> = StdPtr::new(0x0122374c);
#[generate_global]
const WORLD_SEED: StdPtr<usize> = StdPtr::new(0x01205004);
