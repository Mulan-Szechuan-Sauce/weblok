use std::mem::transmute;

use crate::game::{Piece, Occupancy};

const PIECE_COUNT: u8 = std::mem::variant_count::<Piece>() as u8;

pub struct UiState {
    pub selected_piece: Piece,
    pub selected_occupancy: Occupancy,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            selected_piece: Piece::One,
            selected_occupancy: Occupancy::Green,
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
}
