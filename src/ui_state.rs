use std::mem::transmute;

use bevy::{prelude::{Entity, Transform}, math::Vec2};

use crate::game::{Piece, Occupancy, Rotation};

const PIECE_COUNT: u8 = std::mem::variant_count::<Piece>() as u8;

pub struct UiState {
    pub selected_piece: Piece,
    pub selected_occupancy: Occupancy,
    pub selected_rotation: Rotation,
    // FIXME: Misnomer
    pub board_offset_x: f32,
    pub board_offset_y: f32,
    pub tile_size: f32,
    pub tile_padding: f32,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            selected_piece: Piece::One,
            selected_occupancy: Occupancy::Green,
            selected_rotation: Rotation::Zero,
            board_offset_x: -500.,
            board_offset_y: -500.,
            tile_size: 20.,
            tile_padding: 5.,
        }
    }

    pub fn set_selected_piece_u8(&mut self, value: u8) {
        self.selected_piece = unsafe { transmute(value) };
    }

    pub fn next_selected_piece(&mut self) {
        self.selected_piece = unsafe {
            transmute((self.selected_piece as u8 + 1) % PIECE_COUNT)
        };
    }

    pub fn prev_selected_piece(&mut self) {
        self.selected_piece = unsafe { transmute(
            if self.selected_piece == Piece::One {
                PIECE_COUNT - 1
            } else {
                ((self.selected_piece as u8 - 1) % PIECE_COUNT) as u8
            }
        )};
    }

    pub fn next_selected_occupancy(&mut self) {
        self.selected_occupancy = match self.selected_occupancy {
            Occupancy::Empty  => unreachable!(),
            Occupancy::Green  => Occupancy::Red,
            Occupancy::Red    => Occupancy::Blue,
            Occupancy::Blue   => Occupancy::Yellow,
            Occupancy::Yellow => Occupancy::Green,
        };
    }

    pub fn tile_transform(&self, mouse: Vec2, x: i8, y: i8, z: f32) -> Transform {
        Transform::from_xyz(
            mouse.x
                + x as f32 * (self.tile_size + self.tile_padding)
                + self.board_offset_x,
            mouse.y
                + y as f32 * (self.tile_size + self.tile_padding)
                + self.board_offset_y,
            z,
        )
    }
}
