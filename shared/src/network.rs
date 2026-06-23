use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientPacket {
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerPacket {
    Pong,
}
