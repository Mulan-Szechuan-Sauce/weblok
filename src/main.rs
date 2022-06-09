#![feature(variant_count)]
use bevy::{app::AppExit, prelude::*, window::PresentMode, input::mouse::MouseWheel};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod game;
use game::*;
mod ui_state;
use ui_state::*;

use weblock_codegen::piece;

#[derive(Component)]
struct UnplacedPiece;

fn close_on_esc(key_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if key_input.just_released(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn place_piece_system(
    mut cursor_evr: EventReader<CursorMoved>,
    mut query: Query<(&mut Transform, With<UnplacedPiece>)>,
) {
    for ev in cursor_evr.iter() {
        for (mut transform, something) in query.iter_mut() {
            transform.translation = Vec3::new(ev.position.x - 500., ev.position.y - 500., 1.);
        }
    }
}

// fn place_piece_system(
//     windows: Res<Windows>,
//     mut commands: Commands,
//     ui_state: Res<UiState>,
//     mut query: Query<(&mut Transform, With<UnplacedPiece>)>,
// ) {
//     let window = windows.get_primary().expect("Mouse is in a window????");

//     for (mut transform, something) in query.iter_mut() {
//         if let Some(pos) = window.cursor_position() {
//             transform.translation = Vec3::new(pos.x - 500., pos.y - 500., 1.);
//         }
//     }
// }

fn pizz_system(
    key_input: Res<Input<KeyCode>>,
    mut event_reader: EventReader<MouseWheel>,
    mut ui_state: ResMut<UiState>
) {
    if key_input.just_released(KeyCode::Tab) {
        ui_state.next_selected_occupancy();
    }

    for ev in event_reader.iter() {
        let increment = ev.y as i64;
        if increment > 0 {
            ui_state.next_selected_piece();
        } else {
            ui_state.prev_selected_piece();
        }
    }
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);

    let mut board = Board::new();
    board.place(Occupancy::Green, Piece::FourL, Rotation::OneEighty, 0, 0);
    // board.place(Occupancy::Green, Piece::FiveU, Rotation::Zero, 0, 0);
    // board.place(Occupancy::Red, Piece::FiveU, Rotation::Zero, 10, 5);
    // board.place(Occupancy::Blue, Piece::FiveW, Rotation::Zero, 13, 13);
    // board.place(Occupancy::Yellow, Piece::FiveU, Rotation::Ninety, 0, 7);

    let mut state = UiState::new();
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
                transform: Transform::from_xyz(x as f32 * 25. - 250., y as f32 * -25. + 250., 1.),
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(20., 20.)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }

    
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        sprite: Sprite {
            color: Color::SEA_GREEN,
            custom_size: Some(Vec2::new(40., 40.)),
            ..Default::default()
        },
        ..Default::default()
    }).insert(UnplacedPiece);
    
    commands.insert_resource(state);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("weblok"),
            width: 1000.,
            height: 1000.,
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
