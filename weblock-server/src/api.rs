use std::{sync::Arc, collections::HashMap};

use serde::{Serialize, Deserialize};
use tokio::sync::{RwLock, broadcast::Sender};

pub(crate) struct WaitingRoom {
    rooms: Arc<RwLock<HashMap<String, Room>>>,
}

#[derive(Clone)]
pub(crate) struct Room {
    id: String,
    players: Vec<String>,
    tx: Sender<ChatMsg>,
}

pub(crate) struct ChatMsg {
    username: String,
    message: String,
}






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
