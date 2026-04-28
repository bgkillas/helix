use crate::*;
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
#[generate_global]
const ENTITY_TAG_MANAGER: StdPtr<StdPtr<TagManager<u16>>> = StdPtr::new(0x01206fac);
#[generate_global]
const COMPONENT_TAG_MANAGER: StdPtr<StdPtr<TagManager<u8>>> = StdPtr::new(0x01204b30);
