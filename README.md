# TODOs

- [X] Create simple chat lobby server program
- [ ] Define gameplay API (probably web socket based) supporting:
  - Client: WASM
  - Client: Linux Windows MacOS native
  - Server: AIs
  - Client: Remote players
  - Client: Local game mode (no server / lobbies)

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
