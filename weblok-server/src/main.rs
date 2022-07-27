use std::{env, io::Error};

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Sender},
};
use tokio_tungstenite::tungstenite::Message;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use weblok_common::*;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:6969".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on: {}", addr);

    let (tx, _) = broadcast::channel::<String>(10);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(accept_connection(stream, tx.clone()));
            }
            Err(err) => error!("{}", err),
        };
    }
}

async fn accept_connection(stream: TcpStream, room_broadcaster: Sender<String>) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    let mut broadcast_listener = room_broadcaster.subscribe();

    // Client sends message
    // -> Server intprets message and decides on response
    // -> Server response is pushed to channel
    // -> All subscribers send the ServerMessage to their websocket
    //
    // example:
    // Client sends chat message
    // -> Server turns this into ServerMessage::ChatMessage
    // -> Server writes ChatMessage to room broadcaster
    // -> Subscribers take ServerMessage and send to their websocket

    loop {
        tokio::select! {
            next = read.next() => {
                match next {
                    Some(Ok(Message::Binary(msg))) => {
                        match bincode::deserialize::<ClientMessage>(&msg) {
                            Ok(des) => match des {
                                ClientMessage::SendChatMessage(chat_msg) => {
                                    room_broadcaster.send(chat_msg).expect("Broadcast failed to send");
                                },
                                _ => todo!(),
                            },
                            _ => todo!(),
                        }
                    },
                    Some(Ok(Message::Close(None))) => {
                        println!("OUR client gracefully disconnected");
                        return;
                    },
                    Some(Ok(_)) => {
                        println!("OUR client sent a non-binary message");
                    },
                    Some(Err(e)) => {
                        println!("OUR client errored: {}", e);
                    },
                    None => {
                        panic!("wtf happened")
                    },
                }
            }
            bc_msg = broadcast_listener.recv() => {
                println!("Somebody sent a message");

                let content = bincode::serialize(
                    &ServerMessage::BroadcastChatMessage(
                        0,
                        "peelimirks".to_owned(),
                        bc_msg.expect("Recieving message")))
                    .expect("Poo");

                write.send(Message::Binary(content))
                    .await
                    .expect("Failed to send message");
            }
        };
    }
}
