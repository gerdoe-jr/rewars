use redengine::prelude::client::*; // netcore

use crate::shared::gamemessage::GameMessage;

pub struct RewarsClient {

}

impl Default for RewarsClient {
    fn default() -> Self {
        Self {

        }
    }
}

impl GameTick for RewarsClient {
    type G = GameMessage;

    fn on_pre_tick(&mut self) {
    }

    fn on_tick(&mut self) {
    }

    fn on_post_tick(&mut self) {
    }
}

impl GameClient for RewarsClient {
    fn on_player_connect(&mut self, client_id: u64) {
    }

    fn on_player_disconnect(&mut self) {
    }

    fn on_player_packet(&mut self, net_packet: NetPacket<<Self as GameTick>::G>) {
    }

    fn finalize_state(&mut self) -> Option<NetPacket<<Self as GameTick>::G>> {
        None
    }
}
