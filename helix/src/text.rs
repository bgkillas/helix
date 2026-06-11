use crate::world_sync::WorldSync;
use crate::{Context, DEFAULT_PORT, Message};
use bevy_tangled::{ClientTrait as _, Compression, Reliability};
use noita_api::{WorldSeed, game_print};
use rand::Rng as _;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
impl Context {
    pub fn text(&mut self, msg: &str) {
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
                    .unwrap_or_else(|_| rand::rng().next_u32().strict_cast());
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
                    self.world_sync = Some(WorldSync::default());
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
}
