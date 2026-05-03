#[noita_api_macros::generate_globals]
mod inner {
    use crate::{
        DeathMatch, Entity, EntityManager, EventManager, GameGlobal, GameMode, LogFlush, LogLevel,
        MaxComponent, NewGameCount, Platform, StdBox, StdPtr, StdString, StdVec, TagManager,
        WorldSeed,
    };
    const GAME_GLOBAL: StdPtr<StdPtr<GameGlobal>> = StdPtr::new(0x0122_374c);
    const DEATH_MATCH: StdPtr<StdPtr<DeathMatch>> = StdPtr::new(0x0120_4bc0);
    const ENTITY_MANAGER: StdPtr<StdPtr<EntityManager>> = StdPtr::new(0x0120_4b98);
    const ENTITY_TAG_MANAGER: StdPtr<StdPtr<TagManager<u16>>> = StdPtr::new(0x0120_6fac);
    const COMPONENT_TAG_MANAGER: StdPtr<StdPtr<TagManager<u8>>> = StdPtr::new(0x0120_4b30);
    const WORLD_STATE: StdPtr<StdPtr<Entity>> = StdPtr::new(0x0120_4bd0);
    //const WORLD_STATE_COMPONENT: StdPtr<StdPtr<Component<WorldState>>> = StdPtr::new(0x0120_5010);
    const EVENT_MANAGER: StdPtr<StdPtr<EventManager>> = StdPtr::new(0x0120_4b34);
    const WORLD_SEED: StdPtr<WorldSeed> = StdPtr::new(0x0120_5004);
    const GAME_MODE: StdPtr<GameMode> = StdPtr::new(0x0120_761c);
    const LOG_FLUSH: StdPtr<LogFlush> = StdPtr::new(0x0120_4fe0);
    const LOG_LEVEL: StdPtr<LogLevel> = StdPtr::new(0x0115_2518);
    const PLATFORM: StdPtr<Platform> = StdPtr::new(0x0122_1bc0);
    const NEW_GAME_COUNT: StdPtr<NewGameCount> = StdPtr::new(0x0120_5024);
    //const GLOBAL_STATS: StdPtr<GlobalStats> = StdPtr::new(0x0120_8940),
    //const COMPONENT_TYPE_MANAGER: StdPtr<ComponentTypeManager> = StdPtr::new(0x0122_3c88);
    //const TRANSLATION_MANAGER: StdPtr<TranslationManager> = StdPtr::new(0x0120_7c28);
    const FILENAMES: StdPtr<StdVec<StdString>> = StdPtr::new(0x0120_7bd4);
    //const INVENTORY: StdPtr<Inventory> = StdPtr::new(0x0122_24f0);
    //const MODS: StdPtr<Mods> = StdPtr::new(0x0120_7e90);
    const MAX_COMPONENT: StdPtr<MaxComponent> = StdPtr::new(0x0115_2ff0);
    //const COMPONENT_MANAGER: StdPtr<ComponentSystemManager> = StdPtr::new(0x0122_36e8);
    //const DEBUG_SETTINGS: StdPtr<DebugSettings> = StdPtr::new(0x0120_7e20);
}
