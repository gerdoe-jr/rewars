use alkahest::{Formula, Serialize, Deserialize};

#[derive(Debug, Clone, Formula, Serialize, Deserialize)]
pub enum GameMessage {
    Client(ClientMessage),
    Server(ServerMessage)
}

#[derive(Debug, Clone, Formula, Serialize, Deserialize)]
pub enum ClientMessage {
    ClientData {
        nickname: String,
        clan: String
    },
    Chat(String)
}

#[derive(Debug, Clone, Formula, Serialize, Deserialize)]
pub enum ServerMessage {
    ServerData,
    ClientChat {
        client_id: u64,
        message: String
    }
}

pub const MAX_GAMEMESSAGES_PER_PACKET: usize = 128;
