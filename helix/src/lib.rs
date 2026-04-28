use bevy_tangled::Client;
use bitcode::{Decode, Encode};
use std::sync::{LazyLock, Mutex};
//const APPID: u32 = 881100;
pub static NET: LazyLock<Mutex<Client>> = LazyLock::new(|| Mutex::new(Client::new().unwrap()));
#[noita_api::lua_module(true)]
mod lua {
    use crate::{Message, NET};
    use bevy_tangled::{ClientTrait, Compression, Reliability};
    use noita_api::*;
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::atomic::Ordering;
    use std::sync::{LazyLock, Once};
    use tokio::runtime::Runtime;
    static ON_INIT: Once = Once::new();
    static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());
    fn init_once() {
        disable_pause();
        disable_inventory();
        disable_item_pickup();
    }
    #[lua_function]
    fn update() {
        let mut net = NET.lock().unwrap();
        net.update().unwrap();
        net.recv(|_, msg| match msg.data {
            Message::Text(s) => {
                game_print!("{s}");
            }
        });
    }
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn world_init() {}
    #[lua_function]
    fn on_paused_change(paused: bool, _: bool) {
        DISABLE_INVENTORY.store(paused, Ordering::Relaxed);
        DISABLE_ITEM_PICKUP.store(paused, Ordering::Relaxed);
        //TODO PLAYER_ID.store(id, Ordering::Relaxed);
    }
    #[lua_function]
    fn init() {
        ON_INIT.call_once(init_once);
        PAUSE_SIMULATE.store(false, Ordering::Relaxed);
    }
    #[lua_function]
    fn world_seed_init() {}
    #[lua_function]
    fn on_pause() {
        new_game_pause_update()
    }
    #[lua_function]
    fn player_spawn(_: usize) {}
    #[lua_function]
    fn text_msg(msg: &str) {
        if let Some(cmd) = msg.strip_prefix("/") {
            if let Some(host) = cmd.strip_prefix("connect") {
                let host = host.trim();
                let addr = host
                    .parse()
                    .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
                let mut net = NET.lock().unwrap();
                if let Err(e) = net.join_ip_runtime(addr, None, None, &RUNTIME) {
                    game_print!("{e:?}");
                }
            } else if cmd == "new" {
                delay_new_game();
            } else if cmd == "host" {
                let mut net = NET.lock().unwrap();
                if let Err(e) = net.host_ip_runtime(None, None, &RUNTIME) {
                    game_print!("{e:?}");
                }
            }
        } else {
            game_print!("{msg}");
            let net = NET.lock().unwrap();
            let msg = Message::Text(msg.to_string());
            net.broadcast(&msg, Reliability::Reliable, Compression::Uncompressed)
                .unwrap()
        }
    }
}
#[derive(Encode, Decode)]
pub enum Message {
    Text(String),
}
