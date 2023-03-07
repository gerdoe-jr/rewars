use redengine::prelude::server::*; // netcore

use crate::shared::gamemessage::GameMessage;

pub struct RewarsServer {

}

impl Default for RewarsServer {
    fn default() -> Self {
        Self {

        }
    }
}

impl GameTick for RewarsServer {
    type G = GameMessage;

    fn on_pre_tick(&mut self) {
    }

    fn on_tick(&mut self) {
    }

    fn on_post_tick(&mut self) {
    }
}

impl GameServer for RewarsServer {
    fn on_player_connect(&mut self, client_id: u64, user_data: Box<[u8]>) {
    }

    fn on_player_disconnect(&mut self, client_id: u64) {
    }

    fn on_player_packet(&mut self, client_id: u64, net_packet: NetPacket<Self::G>) {
    }

    fn finalize_state(&mut self) {
    }

    fn finalize_packet(&mut self, client_id: u64) -> Option<NetPacket<Self::G>> {
        None
    }
}
