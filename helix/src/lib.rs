#![feature(sync_unsafe_cell)]
use bevy_tangled::Client;
use noita_api::{disable_inventory, disable_item_pickup, disable_pause, lua_module};
use tokio::runtime::Runtime;
const DEFAULT_PORT: u16 = 5463;
pub struct Context {
    pub world_seed: usize,
    pub runtime: Runtime,
    pub net: Client,
}
//#[lua_module(true, "./mod/helix.lua")]
#[lua_module(true)]
mod lua {
    use crate::{Context, DEFAULT_PORT, Message};
    use bevy_tangled::{ClientTrait as _, Compression, Reliability};
    use noita_api::{
        PAUSE_SIMULATE, WorldSeed, game_print, new_game_pause_update, set_pause_no_inventory,
    };
    use rand::Rng as _;
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    use std::sync::atomic::Ordering;
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
                    let addr_str = host.trim();
                    let addr = addr_str.parse().map_or_else(
                        |_| {
                            host.parse().unwrap_or(SocketAddr::new(
                                IpAddr::V6(Ipv6Addr::LOCALHOST),
                                DEFAULT_PORT,
                            ))
                        },
                        |ip| SocketAddr::new(ip, DEFAULT_PORT),
                    );
                    if let Err(e) = self.net.join_ip_runtime(addr, None, None, &self.runtime) {
                        game_print!("{e:?}");
                    } else {
                        game_print!("joining session");
                    }
                } else if let Some(seed) = cmd.strip_prefix("new")
                    && self.net.is_host()
                {
                    let seed_str = seed.trim();
                    self.world_seed = seed_str
                        .parse()
                        .unwrap_or_else(|_| usize::try_from(rand::rng().next_u32()).unwrap());
                    if let Err(e) = self.net.broadcast(
                        &Message::World(self.world_seed),
                        Reliability::Reliable,
                        Compression::Uncompressed,
                    ) {
                        game_print!("{e:?}");
                    }
                    game_print!("new seed: {}", self.world_seed);
                } else if let Some(port) = cmd.strip_prefix("host") {
                    let port_str = port.trim();
                    if let Err(e) = self.net.host_ip_runtime(
                        port_str.parse().unwrap_or(DEFAULT_PORT),
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
                if let Err(e) = self.net.broadcast(
                    &Message::Text(msg.to_owned()),
                    Reliability::Reliable,
                    Compression::Uncompressed,
                ) {
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
    #[lua_function]
    fn test2(a: &mut [isize]) {
        noita_api::log_println!("vec: {a:?}");
    }
    #[lua_function]
    fn on_paused_change(paused: bool, _is_wand_pickup: bool) {
        set_pause_no_inventory(paused);
    }
    #[lua_function]
    fn mod_init() {
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
impl Default for Context {
    #[inline]
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
