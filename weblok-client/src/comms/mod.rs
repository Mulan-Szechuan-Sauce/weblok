use async_trait::async_trait;

use weblok_common::*;

#[async_trait]
pub (crate)trait MsgSource {
    async fn send_message(msg: ClientMessage);
    async fn recv_message() -> Option<ServerMessage>;
}
