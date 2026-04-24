use bevy_tangled::Client;
use bevy_tangled::bitcode::{Decode, Encode};
use std::sync::{LazyLock, Mutex};
use tokio::runtime::Runtime;
//const APPID: u32 = 881100;
pub static NET: LazyLock<Mutex<Client>> = LazyLock::new(|| Mutex::new(Client::new().unwrap()));
pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());
#[noita_api::lua_module(true)]
mod lua {
    use crate::{Message, NET, RUNTIME};
    use bevy_tangled::{ClientTrait, Compression, Reliability};
    use std::net::{IpAddr, Ipv4Addr};
    #[lua_function]
    fn update() {
        let mut net = NET.lock().unwrap();
        net.update().unwrap();
        net.recv(|_, msg| match msg.data {
            Message::Text(s) => {
                noita_api::game_print!("{s}");
            }
        })
    }
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn world_init() {}
    #[lua_function]
    fn init() {}
    #[lua_function]
    fn world_seed_init() {}
    #[lua_function]
    fn text_msg(msg: &str) {
        if let Some(host) = msg.strip_prefix("/join ") {
            if host == "localhost" {
                let mut net = NET.lock().unwrap();
                noita_api::print!(
                    "{:?}",
                    net.join_ip_runtime(
                        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                        None,
                        None,
                        &RUNTIME,
                    )
                );
            }
        } else if msg == "/new" {
            noita_api::new_game();
        } else if msg == "/host" {
            let mut net = NET.lock().unwrap();
            noita_api::print!("{:?}", net.host_ip_runtime(None, None, &RUNTIME));
        } else {
            noita_api::game_print!("{msg}");
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
