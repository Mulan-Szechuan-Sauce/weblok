use bevy::{prelude::*, window::PresentMode, app::AppExit};

mod game;
use game::*;
use weblock_codegen::piece;

fn close_on_esc(
    key_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if key_input.just_released(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board.to_string());
    println!("{}", piece_map_to_string(rot_piece_180(vec!(
        vec!(0, 1),
        vec!(1, 1),
    ))));

    // App::new()
    //     .insert_resource(WindowDescriptor {
    //         title: String::from("weblok"),
    //         width: 1000.,
    //         height: 1000.,
    //         ..default()
    //     })
    //     .add_plugins(DefaultPlugins)
    //     .add_system(close_on_esc)
    //     .run();
}
