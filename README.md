# TODOs

- [X] Create simple chat lobby server program
- [X] Get websockets working
  - [X] Client
  - [X] Server
- [ ] Define gameplay API (web socket based) supporting:
  - Client: WASM & native
  - Client: Linux Windows MacOS native
  - Server: AIs
  - Client: Remote players
  - Client: Local game mode (no server / lobbies)
- [X] Simple chat server
- [ ] Generate RNG room tokens specific to each client
- [ ] Lobby system, join rooms to chat

Client modes:
 - Connect to server for game
 - Entirely local



```rust
trait Game {
    fn play_move();
}

/* CPU go brrrrr */
impl Game for LocalGame {
}

/* Websocket */
impl Game for RemoteGame {
}
```
