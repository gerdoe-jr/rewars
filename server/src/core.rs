pub mod clientdata;

use std::collections::HashMap;
use tracing::info;

use rena::prelude::server::*; // netcore

use shared::*;
use self::clientdata::{ClientData, ArrayString};

pub struct RewarsServer {
    clients: HashMap<u64, (ClientData, Vec<<Self as GameTick>::G>)>,
}

impl Default for RewarsServer {
    fn default() -> Self {
        Self {
            clients: HashMap::with_capacity(MAX_CLIENTS_PER_SERVER)
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
        for (_, (_, game_messages)) in self.clients.iter_mut() {
            game_messages.clear();
        }
    }
}

impl GameServer for RewarsServer {
    fn on_player_connect(&mut self, client_id: u64, user_data: Box<[u8]>) {
        let _user_data = user_data;
        
        self.clients.insert(client_id, (ClientData::new(), Vec::new()));

        info!("[{}] connected to the server", client_id);
    }

    fn on_player_disconnect(&mut self, client_id: u64) {
        info!("[{}] disconnected from the server", client_id);
    }

    fn on_player_packet(&mut self, client_id: u64, game_messages: Vec<<Self as GameTick>::G>) {
        for message in game_messages.iter() {
            match message {
                GameMessage::Client(message) => {
                    if let Some((data, _game_messages)) = self.clients.get_mut(&client_id) {
                        match message {
                            ClientMessage::ClientData { nickname, clan } => {
                                data.nickname = ArrayString::from_string(nickname);
                                data.clan = ArrayString::from_string(clan);
                            },
                            ClientMessage::Chat(message) => {
                                for (&id, (_, game_messages)) in self.clients.iter_mut() {
                                    if client_id != id {
                                        game_messages.push(GameMessage::Server(ServerMessage::ClientChat { client_id, message: message.to_string() }));
                                    }
                                }
                            }
                        }
                    }
                }
                GameMessage::Server(_) => {}
            }
        }
    }

    fn finalize_packet(&mut self, client_id: u64) -> Option<Vec<<Self as GameTick>::G>> {
        let (_, game_messages) = self.clients.get_mut(&client_id).unwrap();

        if game_messages.is_empty() {
            return None;
        }

        Some(game_messages.drain(..).collect())
    }
}
