#![feature(variant_count)]
#![feature(generic_const_exprs)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, input::mouse::MouseWheel, prelude::*, window::PresentMode};

mod game;
use game::*;
mod ui_state;
use ui_state::*;

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
    ui_state: Res<UiState>,
    windows: Res<Windows>,
    mut board: ResMut<Board>,
) {
    let (_, win_height) = get_window_dims(&windows);

    let piece_offsets = ui_state.selected_piece.offsets(ui_state.selected_rotation);
    let pivot = piece_offsets.pivot;

    for ev in cursor_evr.iter() {
        let (mouse_board_coords, snap_place_pos) =
            snap_piece_if_in_grid(&ui_state, ev.position, win_height);

        let place_pos = match mouse_board_coords {
            Some((col, row)) => {
                let coords = coords_for_placement(
                    ui_state.selected_piece,
                    ui_state.selected_rotation,
                    col - pivot.0,
                    row - pivot.1,
                );
                if board.is_placement_valid(ui_state.selected_occupancy, &coords) {
                    snap_place_pos
                } else {
                    ev.position
                }
            }
            _ => ev.position
        };

        for (mut transform, UnplacedPiece(x, y)) in query.iter_mut() {
            *transform = ui_state.tile_transform(place_pos, *x, *y, 1., pivot);
        }
    }
}

/// Returns (board_coords, adjusted/snapped_position)
fn snap_piece_if_in_grid(
    ui_state: &UiState,
    ev_position: Vec2,
    win_height: f32,
) -> (Option<(i8, i8)>, Vec2) {
    let magic_size = ui_state.tile_size + ui_state.tile_padding;
    let snap_dist = 0.2;

    let possy = Vec2::new(
        (ev_position.x + ui_state.board_offset_x / 2.0) / magic_size,
        (win_height - ev_position.y + ui_state.board_offset_y / 2.0) / magic_size,
    );

    let round_x = possy.x.round();
    let round_y = possy.y.round();
    if round_x >= 0.0 && round_x < 20.0 && round_y >= 0.0 && round_y < 20.0 {
        if (possy.x - round_x).abs() <= snap_dist && (possy.y - round_y).abs() <= snap_dist {
            let coords = (round_x as i8, round_y as i8);

            return (
                Some(coords),
                Vec2::new(
                    (-ui_state.board_offset_x / 2.0) + round_x * magic_size,
                    (win_height + ui_state.board_offset_y / 2.0) - round_y * magic_size,
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

    let PieceOffsets { offsets, pivot } =
        ui_state.selected_piece.offsets(ui_state.selected_rotation);

    for (x, y) in offsets {
        let [h, s, l, _] = ui_state.selected_occupancy.color().as_hlsa_f32();

        commands
            .spawn_bundle(SpriteBundle {
                transform: ui_state.tile_transform(mouse, x, y, 1., pivot),
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
    mut event_reader: EventReader<MouseWheel>,
    mut ui_state: ResMut<UiState>,
    unplaced_entities: Query<Entity, With<UnplacedPiece>>,
    windows: Res<Windows>,
) {
    if key_input.just_released(KeyCode::Tab) {
        ui_state.next_selected_occupancy();
        replace_selected_piece(&mut commands, &ui_state, unplaced_entities.iter(), &windows);
    }
    if key_input.just_released(KeyCode::R) {
        ui_state.next_selected_rotation();
        replace_selected_piece(&mut commands, &ui_state, unplaced_entities.iter(), &windows);
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
            let color = match board.occupancies.get(x as i8, y as i8) {
                Occupancy::Empty => Color::DARK_GRAY,
                other => other.color(),
            };
            commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(x as f32 * 25. - 250., y as f32 * -25. + 250., 1.),
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(ui_state.tile_size, ui_state.tile_size)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }

    spawn_piece(&mut commands, &ui_state, &windows);

    commands.insert_resource(board);
    commands.insert_resource(ui_state);
}

fn main() {
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
}
