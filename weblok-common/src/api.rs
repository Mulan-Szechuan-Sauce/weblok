use serde::{Serialize, Deserialize};

pub use bincode;

/// Sending from the client to the server
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    GetRooms,
    /// room_id
    JoinRoom(String),
    SendChatMessage(String),
}

/// Sending from the server to the client
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    /// timestamp, username, message
    BroadcastChatMessage(u32, String, String),
    GetRoomsResponse(Vec<String>),
    JoinRoomResponse(Result<(), String>),
}
