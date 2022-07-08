use std::{collections::HashMap, env, io::Error, sync::Arc};

mod api;
use api::*;

use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use log::{error, info};
use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::{
        broadcast::{self, Receiver, Sender},
        RwLock,
    },
};
use tokio_tungstenite::tungstenite::Message;

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
                tokio::spawn(accept_connection(stream, tx.clone(), tx.subscribe()));
            }
            Err(err) => error!("{}", err),
        };
    }
}

async fn accept_connection(stream: TcpStream, tx: Sender<String>, mut rx: Receiver<String>) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            next = read.next() => {
                match next {
                    Some(Ok(Message::Binary(msg))) => {
                        match bincode::deserialize::<ClientMessage>(&msg) {
                            Ok(des) => match des {
                                ClientMessage::SendChatMessage(chat_msg) => {
                                    tx.send(chat_msg).expect("Broadcast failed to send");
                                },
                                _ => todo!(),
                            },
                            _ => todo!(),
                        }
                    },
                    Some(Ok(_)) => {
                        println!("OUR client sent a non-binary message");
                    },
                    Some(Err(e)) => {
                        println!("OUR client errored: {}", e);
                    },
                    _ => {
                        println!("OUR client gracefully disconnected");
                    },
                }
            }
            bc_msg = rx.recv() => {
                println!("Somebody sent a message");

                // TODO: ServerMessage, not client
                let content = bincode::serialize(
                    &ClientMessage::SendChatMessage(bc_msg.expect("Recieving message")))
                    .expect("Poo");

                write.send(Message::Binary(content))
                    .await
                    .expect("Failed to send message");
            }
        };
    }
}
