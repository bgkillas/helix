use crate::{
    DeathMatch, EntityManager, GameGlobal, GameMode, LogFlush, LogLevel, StdBox, StdPtr,
    TagManager, WorldSeed,
};
use noita_api_macros::generate_global;
#[generate_global]
const GAME_GLOBAL: StdPtr<StdPtr<GameGlobal>> = StdPtr::new(0x0122_374c);
#[generate_global]
const DEATH_MATCH: StdPtr<StdPtr<DeathMatch>> = StdPtr::new(0x0120_4bc0);
#[generate_global]
const ENTITY_MANAGER: StdPtr<StdPtr<EntityManager>> = StdPtr::new(0x0120_4b98);
#[generate_global]
const WORLD_SEED: StdPtr<WorldSeed> = StdPtr::new(0x0120_5004);
#[generate_global]
const GAME_MODE: StdPtr<GameMode> = StdPtr::new(0x0120_761c);
#[generate_global]
const LOG_FLUSH: StdPtr<LogFlush> = StdPtr::new(0x0120_4fe0);
#[generate_global]
const LOG_LEVEL: StdPtr<LogLevel> = StdPtr::new(0x0115_2518);
#[generate_global]
const ENTITY_TAG_MANAGER: StdPtr<StdPtr<TagManager<u16>>> = StdPtr::new(0x0120_6fac);
#[generate_global]
const COMPONENT_TAG_MANAGER: StdPtr<StdPtr<TagManager<u8>>> = StdPtr::new(0x0120_4b30);
