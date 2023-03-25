use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use rena::prelude::client::*;
use tracing::info; // netcore

use shared::*;

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

pub struct RewarsClient {
    client_id: u64,
    game_messages: Vec<<Self as GameTick>::G>,
    stdin_receiver: Receiver<String>
}

impl Default for RewarsClient {
    fn default() -> Self {
        Self {
            client_id: 0,
            game_messages: Vec::new(),
            stdin_receiver: spawn_stdin_channel()
        }
    }
}

impl GameTick for RewarsClient {
    type G = GameMessage;

    fn on_pre_tick(&mut self) {
        
    }

    fn on_tick(&mut self) {
        if let Ok(line) = self.stdin_receiver.try_recv() {
            self.game_messages.push(GameMessage::Client(ClientMessage::Chat(line)));
        }
    }

    fn on_post_tick(&mut self) {
    }
}

impl GameClient for RewarsClient {
    fn on_player_connect(&mut self, client_id: u64) {
        self.client_id = client_id;

        info!("connected as [{}]", client_id)
    }

    fn on_player_disconnect(&mut self) {
        info!("disconnected")
    }

    fn on_player_packet(&mut self, game_messages: Vec<<Self as GameTick>::G>) {
        for message in game_messages {
            match message {
                GameMessage::Server(message) => {
                    match message {
                        ServerMessage::ClientChat { client_id, message } => {
                            info!("[{}] said: {}", client_id, message);
                        },
                        ServerMessage::ServerData => {}
                    }
                },
                _ => {}
            }
        }
    }

    fn finalize_state(&mut self) -> Option<Vec<<Self as GameTick>::G>> {
        if self.game_messages.is_empty() {
            return None;
        }

        Some(self.game_messages.drain(..).collect())
    }
}
