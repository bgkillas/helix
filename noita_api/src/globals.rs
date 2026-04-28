use crate::alloc::{StdBox, StdPtr};
use crate::types::death_match::DeathMatch;
use crate::types::game_global::GameGlobal;
use crate::types::game_mode::GameMode;
use crate::types::world_seed::WorldSeed;
use crate::{EntityManager, LogFlush, LogLevel};
use noita_api_macros::generate_global;
#[generate_global]
const GAME_GLOBAL: StdPtr<StdPtr<GameGlobal>> = StdPtr::new(0x0122374c);
#[generate_global]
const DEATH_MATCH: StdPtr<StdPtr<DeathMatch>> = StdPtr::new(0x01204bc0);
#[generate_global]
const ENTITY_MANAGER: StdPtr<StdPtr<EntityManager>> = StdPtr::new(0x01204b98);
#[generate_global]
const WORLD_SEED: StdPtr<WorldSeed> = StdPtr::new(0x01205004);
#[generate_global]
const GAME_MODE: StdPtr<GameMode> = StdPtr::new(0x0120761c);
#[generate_global]
const LOG_FLUSH: StdPtr<LogFlush> = StdPtr::new(0x01204fe0);
#[generate_global]
const LOG_LEVEL: StdPtr<LogLevel> = StdPtr::new(0x01152518);
