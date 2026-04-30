#![feature(sync_unsafe_cell)]
use noita_api::lua_module;
#[lua_module(true)]
mod lua {
    use crate::Message;
    use bevy_tangled::{Client, ClientTrait, Compression, Reliability};
    use noita_api::{
        PAUSE_SIMULATE, WorldSeed, disable_inventory, disable_item_pickup, disable_pause,
        game_print, new_game_pause_update, set_pause_no_inventory,
    };
    use rand::Rng;
    use std::net::{IpAddr, Ipv6Addr};
    use std::sync::atomic::Ordering;
    use tokio::runtime::Runtime;
    pub struct Context {
        pub world_seed: usize,
        pub runtime: Runtime,
        pub net: Client,
    }
    impl Context {
        #[lua_function]
        fn update(&mut self) {
            if let Err(e) = self.net.update() {
                game_print!("{e:?}");
            }
            self.net.recv(|_, msg| match msg.data {
                Message::Text(s) => game_print!("{s}"),
                Message::World(world) => {
                    self.world_seed = world;
                    game_print!("new seed: {}", self.world_seed);
                }
            });
        }
        #[lua_function]
        fn text_msg(&mut self, msg: &str) {
            if let Some(cmd) = msg.strip_prefix("/") {
                if let Some(host) = cmd.strip_prefix("join") {
                    let host = host.trim();
                    let addr = host.parse().unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST));
                    if let Err(e) = self.net.join_ip_runtime(addr, None, None, &self.runtime) {
                        game_print!("{e:?}");
                    } else {
                        game_print!("joining session");
                    }
                } else if let Some(seed) = cmd.strip_prefix("new")
                    && self.net.is_host()
                {
                    let seed = seed.trim();
                    self.world_seed = seed
                        .parse()
                        .unwrap_or_else(|_| rand::rng().next_u32() as usize);
                    if let Err(e) = self.net.broadcast(
                        &Message::World(self.world_seed),
                        Reliability::Reliable,
                        Compression::Uncompressed,
                    ) {
                        game_print!("{e:?}");
                    }
                    game_print!("new seed: {}", self.world_seed);
                } else if cmd == "host" {
                    if let Err(e) = self.net.host_ip_runtime(
                        Some(Box::new(|client, peer| {
                            let world = WorldSeed::global();
                            if let Err(e) = client.send(
                                peer,
                                &Message::World(world.seed),
                                Reliability::Reliable,
                                Compression::Uncompressed,
                            ) {
                                game_print!("{e:?}");
                            }
                        })),
                        None,
                        &self.runtime,
                    ) {
                        game_print!("{e:?}");
                    } else {
                        self.world_seed = WorldSeed::global().seed;
                        game_print!("hosting session");
                    }
                }
            } else {
                game_print!("{msg}");
                let msg = Message::Text(msg.to_string());
                if let Err(e) =
                    self.net
                        .broadcast(&msg, Reliability::Reliable, Compression::Uncompressed)
                {
                    game_print!("{e:?}");
                }
            }
        }
        #[lua_function]
        fn world_seed_init(&self) {
            if self.net.is_connected() {
                WorldSeed::global().seed = self.world_seed;
            }
        }
    }
    impl Default for Context {
        fn default() -> Self {
            disable_pause();
            disable_inventory();
            disable_item_pickup();
            Self {
                world_seed: 0,
                runtime: Runtime::new().unwrap(),
                net: Client::new().unwrap(),
            }
        }
    }
    #[lua_function]
    fn on_paused_change(paused: bool, _is_wand_pickup: bool) {
        set_pause_no_inventory(paused);
    }
    #[lua_function]
    fn init() {
        set_pause_no_inventory(false);
        PAUSE_SIMULATE.store(true, Ordering::Relaxed);
    }
    #[lua_function]
    fn on_pause() {
        new_game_pause_update();
    }
}
#[derive(bitcode::Encode, bitcode::Decode)]
pub enum Message {
    Text(String),
    World(usize),
}
