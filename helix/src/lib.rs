use bevy_tangled::Client;
use noita_api::*;
use tokio::runtime::Runtime;
#[lua_module(true)]
mod lua {
    use crate::{ConnectionType, Message};
    use bevy_tangled::{Client, ClientTrait, Compression, Reliability};
    use noita_api::*;
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::atomic::Ordering;
    use tokio::runtime::Runtime;
    pub struct Context {
        pub world_seed: Option<usize>,
        pub runtime: Runtime,
        pub connection_type: ConnectionType,
        pub net: Client,
    }
    impl Context {
        #[lua_function]
        fn update(&mut self) {
            self.net.update().unwrap();
            self.net.recv(|_, msg| match msg.data {
                Message::Text(s) => game_print!("{s}"),
                Message::World(world) => {
                    self.world_seed = Some(world);
                    delay_new_game();
                }
            });
        }
        #[lua_function]
        fn text_msg(&mut self, msg: &str) {
            if let Some(cmd) = msg.strip_prefix("/") {
                if let Some(host) = cmd.strip_prefix("connect") {
                    let host = host.trim();
                    let addr = host
                        .parse()
                        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
                    if let Err(e) = self.net.join_ip_runtime(addr, None, None, &self.runtime) {
                        game_print!("{e:?}");
                    } else {
                        self.connection_type = ConnectionType::Host;
                        self.world_seed = Some(WorldSeed::global().seed);
                    }
                } else if cmd == "new" {
                    delay_new_game();
                } else if cmd == "host" {
                    if let Err(e) = self.net.host_ip_runtime(
                        Some(Box::new(|client, peer| {
                            let world = WorldSeed::global();
                            client
                                .send(
                                    peer,
                                    &Message::World(world.seed),
                                    Reliability::Reliable,
                                    Compression::Uncompressed,
                                )
                                .unwrap();
                        })),
                        None,
                        &self.runtime,
                    ) {
                        game_print!("{e:?}");
                    } else {
                        self.connection_type = ConnectionType::Client;
                    }
                }
            } else {
                game_print!("{msg}");
                let msg = Message::Text(msg.to_string());
                self.net
                    .broadcast(&msg, Reliability::Reliable, Compression::Uncompressed)
                    .unwrap()
            }
        }
        #[lua_function]
        fn world_seed_init(&self) {
            if let Some(seed) = self.world_seed {
                WorldSeed::global().seed = seed;
            }
        }
    }
    #[lua_function]
    fn on_paused_change(paused: bool, _: bool) {
        DISABLE_INVENTORY.store(paused, Ordering::Relaxed);
        DISABLE_ITEM_PICKUP.store(paused, Ordering::Relaxed);
        let player = EntityManager::global()
            .iter_with_tag("player_unit")
            .next()
            .map(|p| p.id)
            .unwrap_or_default();
        PLAYER_ID.store(player, Ordering::Relaxed);
    }
    #[lua_function]
    fn init() {
        PAUSE_SIMULATE.store(true, Ordering::Relaxed);
    }
    #[lua_function]
    fn on_pause() {
        new_game_pause_update()
    }
}
impl Default for lua::Context {
    fn default() -> Self {
        disable_pause();
        disable_inventory();
        disable_item_pickup();
        Self {
            world_seed: None,
            runtime: Runtime::new().unwrap(),
            connection_type: ConnectionType::None,
            net: Client::new().unwrap(),
        }
    }
}
#[derive(bitcode::Encode, bitcode::Decode)]
pub enum Message {
    Text(String),
    World(usize),
}
#[repr(u8)]
pub enum ConnectionType {
    None = 0,
    Host = 1,
    Client = 2,
}
