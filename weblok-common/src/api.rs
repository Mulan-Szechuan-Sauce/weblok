use serde::{Serialize, Deserialize};

/// Sending from the client to the server
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    GetRooms,
    /// room_id
    JoinRoom(String),
    SendChatMessage(String),
}

/// Sending from the server to the client
#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    /// timestamp, username, message
    BroadcastChatMessage(u32, String, String),
    GetRoomsResponse(Vec<String>),
    JoinRoomResponse(Result<(), String>),
}
