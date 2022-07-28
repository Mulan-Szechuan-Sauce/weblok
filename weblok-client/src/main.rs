#![feature(variant_count)]
#![feature(generic_const_exprs)]
#![feature(let_chains)]

mod comms;

/*
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, input::mouse::MouseWheel, prelude::*, window::PresentMode};

mod game;
use game::*;
mod ui_state;
use ui_state::*;
mod log;
use log::*;

#[derive(Component)]
struct UnplacedPiece(i8, i8);

fn close_on_esc(key_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if key_input.just_released(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn place_piece_system(
    mut cursor_evr: EventReader<CursorMoved>,
    mut query: Query<(&mut Transform, &UnplacedPiece), With<UnplacedPiece>>,
    mut ui_state: ResMut<UiState>,
    windows: Res<Windows>,
    mut board: ResMut<Board>,
) {
    let (_, win_height) = get_window_dims(&windows);

    let piece_offsets = ui_state.piece.offsets(ui_state.rotation);
    let pivot = piece_offsets.pivot;

    for ev in cursor_evr.iter() {
        let (mouse_board_coords, snap_place_pos) =
            snap_piece_if_in_grid(&ui_state, ev.position, win_height, pivot);
        ui_state.mouse_board_coords = mouse_board_coords;

        let place_pos = match mouse_board_coords {
            Some((col, row)) => {
                let coords = coords_for_placement(ui_state.piece, ui_state.rotation, col, row);
                if board.is_placement_valid(ui_state.occupancy, &coords) {
                    snap_place_pos
                } else {
                    ev.position
                }
            }
            _ => ev.position,
        };

        for (mut transform, UnplacedPiece(x, y)) in query.iter_mut() {
            *transform = ui_state.tile_transform(place_pos, *x, *y, transform.translation.z, pivot);
        }
    }
}

/// Returns (board_coords, adjusted/snapped_position)
fn snap_piece_if_in_grid(
    ui_state: &UiState,
    ev_position: Vec2,
    win_height: f32,
    pivot: (i8, i8),
) -> (Option<(i8, i8)>, Vec2) {
    let magic_size = ui_state.tile_size + ui_state.tile_padding;
    let snap_dist = 0.2;

    let possy = Vec2::new(
        (ev_position.x + ui_state.window_center_x / 2.0) / magic_size,
        (win_height - ev_position.y + ui_state.window_center_y / 2.0) / magic_size,
    );

    let round_x = possy.x.round();
    let round_y = possy.y.round();
    if round_x >= 0.0 && round_x < 20.0 && round_y >= 0.0 && round_y < 20.0 {
        if (possy.x - round_x).abs() <= snap_dist && (possy.y - round_y).abs() <= snap_dist {
            let coords = (round_x as i8 - pivot.0, round_y as i8 - pivot.1);

            return (
                Some(coords),
                Vec2::new(
                    (-ui_state.window_center_x / 2.0) + round_x * magic_size,
                    (win_height + ui_state.window_center_y / 2.0) - round_y * magic_size,
                ),
            );
        }
    }
    (None, ev_position)
}

fn get_window_dims(windows: &Windows) -> (f32, f32) {
    let window = windows.get_primary().unwrap();
    (window.width(), window.height())
}

fn replace_selected_piece(
    commands: &mut Commands,
    ui_state: &UiState,
    unplaced_entities: impl Iterator<Item = Entity>,
    windows: &Windows,
) {
    for e in unplaced_entities {
        commands.entity(e).despawn();
    }
    spawn_piece(commands, ui_state, windows);
}

fn spawn_piece(commands: &mut Commands, ui_state: &UiState, windows: &Windows) {
    let window = windows.get_primary().unwrap();
    let mouse = if let Some(position) = window.cursor_position() {
        position
    } else {
        return;
    };

    let PieceOffsets { offsets, pivot } = ui_state.piece.offsets(ui_state.rotation);

    for (x, y) in offsets {
        let [h, s, l, _] = ui_state.occupancy.color().as_hlsa_f32();

        commands
            .spawn_bundle(SpriteBundle {
                transform: ui_state.tile_transform(mouse, x, y, 5., pivot),
                sprite: Sprite {
                    color: Color::hsla(h, s, l * 1.3, 0.7),
                    custom_size: Some(Vec2::new(ui_state.tile_size, ui_state.tile_size)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(UnplacedPiece(x as i8, y as i8));
    }
}

fn pizz_system(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut event_reader: EventReader<MouseWheel>,
    mut ui_state: ResMut<UiState>,
    unplaced_entities: Query<Entity, With<UnplacedPiece>>,
    windows: Res<Windows>,
    mut board: ResMut<Board>,
) {
    if key_input.just_released(KeyCode::Tab) {
        ui_state.next_selected_occupancy();
        replace_selected_piece(&mut commands, &ui_state, unplaced_entities.iter(), &windows);
    }
    if key_input.just_released(KeyCode::R) {
        ui_state.next_selected_rotation();
        replace_selected_piece(&mut commands, &ui_state, unplaced_entities.iter(), &windows);
    }
    if mouse_input.just_released(MouseButton::Left) && let Some((col, row)) = ui_state.mouse_board_coords {
        if board.place(
            ui_state.occupancy,
            ui_state.piece,
            ui_state.rotation,
            col,
            row,
        ) {
            for (x, y) in coords_for_placement(ui_state.piece, ui_state.rotation, col, row) {
                spawn_tile(&mut commands, &mut board, ui_state.tile_size, x, y, 2.);
            }
        }
    }

    for ev in event_reader.iter() {
        let increment = ev.y as i64;
        if increment < 0 {
            ui_state.next_selected_piece();
            replace_selected_piece(&mut commands, &ui_state, unplaced_entities.iter(), &windows);
        } else {
            ui_state.prev_selected_piece();
            replace_selected_piece(&mut commands, &ui_state, unplaced_entities.iter(), &windows);
        }
    }
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);

    let mut board = Board::new();
    board.place(Occupancy::Green, Piece::FiveU, Rotation::OneEighty, 0, 0);
    board.place(Occupancy::Green, Piece::One, Rotation::OneEighty, 3, 2);

    let ui_state = UiState::new();
    for y in 0..DIM {
        for x in 0..DIM {
            spawn_tile(
                &mut commands,
                &mut board,
                ui_state.tile_size,
                x as i8,
                y as i8,
                1.,
            );
        }
    }

    spawn_piece(&mut commands, &ui_state, &windows);

    commands.insert_resource(board);
    commands.insert_resource(ui_state);
}

fn spawn_tile(commands: &mut Commands, board: &mut Board, tile_size: f32, x: i8, y: i8, z: f32) {
    let color = match board.occupancies.get(x as i8, y as i8) {
        Occupancy::Empty => Color::DARK_GRAY,
        other => other.color(),
    };
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(x as f32 * 25. - 250., y as f32 * -25. + 250., z),
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            ..Default::default()
        },
        ..Default::default()
    });
}

*/

#[cfg(target_arch = "wasm32")]
use {
    futures::{AsyncReadExt, AsyncWriteExt, SinkExt, StreamExt},
    pharos::*,
    wasm_bindgen::UnwrapThrowExt,
    wasm_bindgen_futures::futures_0_3::spawn_local,
    ws_stream_wasm::*,
};
#[cfg(not(target_arch = "wasm32"))]
use {
    futures_util::{SinkExt, StreamExt},
    tokio::sync::broadcast::{self, Sender},
    tokio_tungstenite::connect_async,
    tokio_tungstenite::tungstenite::Message,
};

static SERVER_URL: &'static str = "ws://127.0.0.1:6969";
type Wss = WebSocketStream<MaybeTlsStream<TcpStream>>;

mod log;
use log::bevy_log;
use tokio::{io::AsyncBufReadExt, net::TcpStream};
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use weblok_common::*;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let (mut ws_stream, _) = connect_async(SERVER_URL).await.expect("Failed to connect");

    let stdin = tokio::io::stdin();
    let reader = tokio::io::BufReader::new(stdin);
    // Take a stream of lines from this
    let mut lines = reader.lines();

    set_username(&mut ws_stream, utils::generate_username()).await;

    loop {
        tokio::select! {
            msg = read_server_message(&mut ws_stream) => {
                dbg!("Server message");
            },
            input = lines.next_line() => {
                match input {
                    Ok(None)        => break,
                    Ok(Some(other)) => match other {
                        msg if msg == "/quit" => break,
                        msg => send_chat_message(&mut ws_stream, msg).await,
                    },
                    Err(err) => panic!("{}", err),
                };
            },
            // input = read_console() => {
            //     dbg!(input);
            // },
        }
    }

    ws_stream.close(None).await.expect("Failed to close stream");
}

async fn send_chat_message(ws_stream: &mut Wss, message: String) {
    let content = bincode::serialize(&ClientMessage::SendChatMessage(message))
        .expect("Failed to serialize chat message");

    ws_stream
        .send(Message::Binary(content))
        .await
        .expect("Failed to write message");
}

async fn set_username(ws_stream: &mut Wss, username: String) {
    let content = bincode::serialize(&ClientMessage::SetUsername(username))
        .expect("Failed to serialize set username request");

    ws_stream
        .send(Message::Binary(content))
        .await
        .expect("Failed to write message");
}

async fn read_server_message(ws_stream: &mut Wss) {
    match ws_stream.next().await {
        Some(Ok(Message::Text(t))) => bevy_log(&t),
        Some(Ok(Message::Binary(t))) => match bincode::deserialize::<ServerMessage>(&t) {
            Ok(server_msg) => {
                dbg!(server_msg);
            },
            Err(e) => {
                dbg!(e);
                panic!("Get owned");
            },
        },
        Some(Err(e)) => {
            dbg!(e);
            return;
        }
        _ => panic!("Oh no. Why am I here"),
        //Message::Binary(_) => panic!("Binary recieved but I didn't expect it"),
    };
}

#[cfg(target_arch = "wasm32")]
fn main() {
    spawn_local(async {
        let (mut ws, mut ws_stream) = WsMeta::connect(SERVER_URL, None)
            .await
            .expect_throw("assume the connection succeeds");
        bevy_log("Connected to web socket!");

        let mut events = ws
            .observe(ObserveConfig::default())
            .await
            .expect_throw("observe");
        bevy_log("observe! ... but not of the error variety");

        ws_stream
            .send(WsMessage::Text("yeet street".to_owned()))
            .await
            .expect("Failed to write message");

        match ws_stream.next().await.expect("Read message") {
            WsMessage::Text(t) => bevy_log(&t),
            WsMessage::Binary(_) => panic!("Binary recieved but I didn't expect it"),
        }

        ws.close().await;

        // Note that since WsMeta::connect resolves to an opened connection, we don't see
        // any Open events here.
        assert!(events.next().await.unwrap_throw().is_closing());
        assert!(events.next().await.unwrap_throw().is_closed());
    });
    /*
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("weblok"),
            width: 1000.,
            height: 1000.,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(pizz_system)
        .add_system(place_piece_system)
        .run();
        */
}
