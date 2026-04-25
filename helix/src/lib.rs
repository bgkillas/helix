use bevy_tangled::Client;
use bevy_tangled::bitcode::{Decode, Encode};
use std::sync::{LazyLock, Mutex};
//const APPID: u32 = 881100;
pub static NET: LazyLock<Mutex<Client>> = LazyLock::new(|| Mutex::new(Client::new().unwrap()));
#[noita_api::lua_module(true)]
mod lua {
    use crate::{Message, NET};
    use bevy_tangled::{ClientTrait, Compression, Reliability};
    use noita_api::pause::{PAUSE_SIMULATE, disable_pause};
    use noita_api::types::game_global::GameGlobal;
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::{LazyLock, Once};
    use tokio::runtime::Runtime;
    static mut DO_RESTART: u8 = 0;
    static ON_INIT: Once = Once::new();
    static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());
    fn init_once() {
        disable_pause()
    }
    #[lua_function]
    fn update() {
        let mut net = NET.lock().unwrap();
        net.update().unwrap();
        net.recv(|_, msg| match msg.data {
            Message::Text(s) => {
                noita_api::game_print!("{s}");
            }
        });
    }
    #[lua_function]
    fn post_update() {}
    #[lua_function]
    fn world_init() {}
    #[lua_function]
    fn init() {
        unsafe {
            PAUSE_SIMULATE = false;
        }
        ON_INIT.call_once(init_once);
    }
    #[lua_function]
    fn world_seed_init() {}
    #[lua_function]
    fn on_pause() {
        if unsafe { DO_RESTART == 1 } {
            let mut game_global = GameGlobal::global();
            if game_global.is_paused() {
                unsafe {
                    DO_RESTART = 0;
                }
                noita_api::new_game();
            } else {
                unsafe {
                    DO_RESTART = 8;
                }
                game_global.pause();
            }
        } else if unsafe { DO_RESTART > 1 } {
            unsafe { DO_RESTART -= 1 }
        }
    }
    #[lua_function]
    fn text_msg(msg: &str) {
        if let Some(host) = msg.strip_prefix("/connect") {
            let host = host.trim();
            let addr = host
                .parse()
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
            let mut net = NET.lock().unwrap();
            noita_api::println!("{:?}", net.join_ip_runtime(addr, None, None, &RUNTIME));
        } else if msg == "/new" {
            unsafe {
                DO_RESTART = 8;
                PAUSE_SIMULATE = true;
            }
            GameGlobal::global().pause();
        } else if msg == "/host" {
            let mut net = NET.lock().unwrap();
            noita_api::println!("{:?}", net.host_ip_runtime(None, None, &RUNTIME));
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
