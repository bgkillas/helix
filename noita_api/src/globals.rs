use crate::alloc::{StdBox, StdPtr};
use crate::types::game_global::GameGlobal;
use crate::types::world_seed::WorldSeed;
use noita_api_macros::generate_global;
#[generate_global]
const GAME_GLOBAL: StdPtr<StdPtr<GameGlobal>> = StdPtr::new(0x0122374c);
#[generate_global]
const WORLD_SEED: StdPtr<WorldSeed> = StdPtr::new(0x01205004);
