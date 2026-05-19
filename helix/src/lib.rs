#![feature(sync_unsafe_cell)]
mod text;
mod world_read;
mod world_sync;
mod world_write;
use crate::world_read::{Chunk, ChunkPos};
use crate::world_sync::WorldSync;
use crate::world_write::{ChunkWrite, WorldWrite};
use bevy_tangled::{Client, ClientTrait as _};
use noita_api::WorldSeed;
use std::collections::HashMap;
use tokio::runtime::Runtime;
const DEFAULT_PORT: u16 = 5463;
pub(crate) struct Context {
    pub world_seed: usize,
    pub runtime: Runtime,
    pub net: Client,
    pub world_init: bool,
    pub world_sync: Option<WorldSync>,
    pub world_write: WorldWrite,
    pub seen_chunks: HashMap<ChunkPos, bool>,
}
impl Context {
    pub fn is_connected(&self) -> bool {
        self.net.is_connected() && WorldSeed::global().seed == self.world_seed
    }
}
//#[noita_api::lua_module("./mod/helix.lua")]
#[noita_api::lua_module]
mod lua {
    use crate::world_sync::SendType;
    use crate::{Context, Message};
    use bevy_tangled::ClientTrait as _;
    use noita_api::{
        ConfigExplosion, DamageFun, DamageModel, DamageThing, Entity, ExplosionFun, FireWandFun,
        GameGlobal, PAUSE_SIMULATE, StdBox, StdString, Vec2, WorldSeed, disable_inventory,
        disable_item_pickup, disable_pause, game_print, new_game_pause_update,
        set_pause_no_inventory,
    };
    use std::sync::atomic::Ordering;
    impl Context {
        #[lua_function]
        fn update(&mut self) {
            if let Err(e) = self.net.update() {
                game_print!("{e:?}");
            }
            if self.is_connected() && GameGlobal::global().frame_num.is_multiple_of(10) {
                self.sync_world();
            }
            self.net.recv(|client, msg| match msg.data {
                Message::Text(s) => game_print!("{s}"),
                Message::Chunks(chunks) => self.world_sync.as_mut().unwrap().push_world(
                    SendType::Net(client),
                    msg.src,
                    chunks,
                ),
                Message::RemoveChunks(chunks) => {
                    self.world_sync.as_mut().unwrap().del_world(msg.src, chunks);
                }
                Message::ChunksWrite(chunks) => self.world_write.write_chunks(&chunks),
                Message::World(world) => {
                    self.world_seed = world;
                    game_print!("new seed: {}", self.world_seed);
                }
            });
        }
        #[lua_function]
        fn text_msg(&mut self, msg: &str) {
            self.text(msg);
        }
        #[lua_function]
        fn world_seed_init(&self) {
            if self.net.is_connected() {
                WorldSeed::global().seed = self.world_seed;
            }
        }
        #[lua_function]
        fn world_init(&mut self) {
            self.world_init = true;
        }
        #[exit_hook]
        fn on_exit(&mut self) {
            self.world_init = false;
        }
    }
    #[lua_function]
    fn on_paused_change(paused: bool, _is_wand_pickup: bool) {
        set_pause_no_inventory(paused);
    }
    #[lua_function]
    fn mod_init() {
        disable_pause();
        disable_inventory();
        disable_item_pickup();
        set_pause_no_inventory(false);
        PAUSE_SIMULATE.store(true, Ordering::Relaxed);
    }
    #[lua_function]
    fn on_pause() {
        new_game_pause_update();
    }
    #[allow(clippy::too_many_arguments)]
    #[fire_hook]
    fn on_fire(
        orig: FireWandFun,
        entity: Option<Entity>,
        varlet_parent: Option<Entity>,
        position: StdBox<Vec2<f32>>,
        projectile: Option<Entity>,
        unk1: isize,
        unk2: isize,
        unk3: u8,
        send_message: bool,
        target_x: f32,
        target_y: f32,
    ) {
        orig(
            entity,
            varlet_parent,
            position,
            projectile,
            unk1,
            unk2,
            unk3,
            send_message,
            target_x,
            target_y,
        );
    }
    #[explosion_hook]
    fn on_explosion(
        orig: ExplosionFun,
        config: StdBox<ConfigExplosion>,
        pos: StdBox<Vec2<f32>>,
        unk: isize,
    ) {
        orig(config, pos, unk);
    }
    #[damage_hook]
    fn on_damage(
        orig: DamageFun,
        entity: Option<Entity>,
        damage_model: Option<StdBox<DamageModel>>,
        description: StdBox<StdString>,
        damage_types: usize,
        damage_args: StdBox<DamageThing>,
        damage: f32,
    ) {
        orig(
            entity,
            damage_model,
            description,
            damage_types,
            damage_args,
            damage,
        );
    }
    #[open_hook]
    fn on_enter() {}
}
#[derive(bitcode::Encode, bitcode::Decode)]
pub(crate) enum Message {
    Text(String),
    World(usize),
    Chunks(Vec<Chunk>),
    RemoveChunks(Vec<ChunkPos>),
    ChunksWrite(Vec<ChunkWrite>),
}
impl Default for Context {
    #[inline]
    fn default() -> Self {
        Self {
            world_seed: 0,
            runtime: Runtime::new().unwrap(),
            net: Client::new().unwrap(),
            world_init: false,
            world_sync: None,
            world_write: WorldWrite::default(),
            seen_chunks: HashMap::new(),
        }
    }
}
