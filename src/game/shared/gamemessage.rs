use alkahest::{Formula, Serialize, Deserialize};

#[derive(Formula, Serialize, Deserialize)]
pub enum GameMessage {
    Client(ClientMessage),
    Server(ServerMessage)
}

#[derive(Formula, Serialize, Deserialize)]
pub enum ClientMessage {
    Empty
}

#[derive(Formula, Serialize, Deserialize)]
pub enum ServerMessage {
    Empty
}
