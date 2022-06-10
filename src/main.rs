#![feature(variant_count)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, input::mouse::MouseWheel, prelude::*, window::PresentMode};

mod game;
use game::*;
mod ui_state;
use ui_state::*;

use weblock_codegen::piece;

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
) {
    for ev in cursor_evr.iter() {
        for (mut transform, UnplacedPiece(x, y)) in query.iter_mut() {
            *transform = ui_state.tile_transform(ev.position, *x, *y, 1.);
        }
    }
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

    for (x, y) in ui_state.selected_piece.offsets(ui_state.selected_rotation) {
        commands
            .spawn_bundle(SpriteBundle {
                transform: ui_state.tile_transform(mouse, x as i8, y as i8, 1.),
                sprite: Sprite {
                    color: Color::SEA_GREEN,
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

fn setup(asset_server: Res<AssetServer>, mut commands: Commands, windows: Res<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);

    let mut board = Board::new();
    board.place(Occupancy::Green, Piece::FourL, Rotation::OneEighty, 0, 0);
    // board.place(Occupancy::Green, Piece::FiveU, Rotation::Zero, 0, 0);
    // board.place(Occupancy::Red, Piece::FiveU, Rotation::Zero, 10, 5);
    // board.place(Occupancy::Blue, Piece::FiveW, Rotation::Zero, 13, 13);
    // board.place(Occupancy::Yellow, Piece::FiveU, Rotation::Ninety, 0, 7);

    let mut ui_state = UiState::new();
    for y in 0..DIM {
        for x in 0..DIM {
            let color = match board.get(x as u8, y as u8) {
                Occupancy::Empty => Color::rgb(1., 1., 1.),
                Occupancy::Green => Color::rgb(0., 1., 0.),
                Occupancy::Red => Color::rgb(1., 0., 0.),
                Occupancy::Blue => Color::rgb(0., 0., 1.),
                Occupancy::Yellow => Color::rgb(1., 1., 0.),
            };
            commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(
                    x as f32 * 25. - 250.,
                    y as f32 * -25. + 250.,
                    1.
                ),
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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(pizz_system)
        .add_system(place_piece_system)
        .run();
}
