use std::collections::HashMap;
use std::mem::transmute;

use bevy::{prelude::Transform, math::Vec2};

use crate::game::{Piece, Occupancy, Rotation};

const PIECE_COUNT: u8 = std::mem::variant_count::<Piece>() as u8;

pub struct UiState {
    pub piece: Piece,
    pub occupancy: Occupancy,
    pub rotation: Rotation,
    pub window_center_x: f32,
    pub window_center_y: f32,
    pub tile_size: f32,
    pub tile_padding: f32,
    pub mouse_board_coords: Option<(i8, i8)>,
    inventories: HashMap<Occupancy, Vec<Piece>>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            piece: Piece::One,
            occupancy: Occupancy::Green,
            rotation: Rotation::Zero,
            window_center_x: -500.,
            window_center_y: -500.,
            tile_size: 20.,
            tile_padding: 5.,
            mouse_board_coords: None,
            inventories: HashMap::from_iter([
                (Occupancy::Blue, Piece::all()),
                (Occupancy::Green, Piece::all()),
                (Occupancy::Red, Piece::all()),
                (Occupancy::Yellow, Piece::all()),
            ]),
        }
    }

    pub fn next_selected_piece(&mut self) {
        self.piece = unsafe {
            transmute((self.piece as u8 + 1) % PIECE_COUNT)
        };
    }

    pub fn prev_selected_piece(&mut self) {
        self.piece = unsafe { transmute(
            if self.piece == Piece::One {
                PIECE_COUNT - 1
            } else {
                ((self.piece as u8 - 1) % PIECE_COUNT) as u8
            }
        )};
    }

    pub fn next_selected_occupancy(&mut self) {
        self.occupancy = match self.occupancy {
            Occupancy::Empty  => unreachable!(),
            Occupancy::Green  => Occupancy::Red,
            Occupancy::Red    => Occupancy::Blue,
            Occupancy::Blue   => Occupancy::Yellow,
            Occupancy::Yellow => Occupancy::Green,
        };
    }

    pub fn next_selected_rotation(&mut self) {
        self.rotation = self.rotation.next_clockwise();
    }

    pub fn tile_transform(
        &self,
        mouse: Vec2,
        x: i8,
        y: i8,
        z: f32,
        (pivot_x, pivot_y): (i8, i8)
    ) -> Transform {
        Transform::from_xyz(
            mouse.x
                + (x - pivot_x) as f32 * (self.tile_size + self.tile_padding)
                + self.window_center_x,
            mouse.y
                + (pivot_y - y) as f32 * (self.tile_size + self.tile_padding)
                + self.window_center_y as f32,
            z,
        )
    }
}
