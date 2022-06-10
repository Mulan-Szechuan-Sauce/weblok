#[allow(dead_code)]
use weblock_codegen::*;

pub const DIM: usize = 20;

pub struct GameState {
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Board {
    pub occupancies: [Occupancy; DIM * DIM],
}

impl Board {
    pub fn new() -> Board {
        Board {
            occupancies: [Occupancy::Empty; DIM * DIM],
        }
    }

    // Returns false if the placement is invalid
    pub fn place(
        &mut self,
        occupancy: Occupancy,
        piece: Piece,
        rot: Rotation,
        col: u8,
        row: u8,
    ) -> bool {
        let coords = coords_for_placement(piece, rot, col, row);
        for (x, y) in coords.iter() {
            if *x as usize >= DIM || *y as usize >= DIM {
                return false;
            }
            if self.get(*x, *y) != Occupancy::Empty {
                return false;
            }
        }
        for (x, y) in coords {
            self.set(x, y, occupancy);
        }
        true
    }

    pub fn get(&self, x: u8, y: u8) -> Occupancy {
        self.occupancies[x as usize + y as usize * DIM]
    }

    fn set(&mut self, x: u8, y: u8, value: Occupancy) {
        self.occupancies[x as usize + y as usize * DIM] = value;
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(DIM * (DIM + 1));
        for i in 0..self.occupancies.len() {
            if i > 0 && i % DIM == 0 {
                s.push('\n');
            }
            s.push_str(&self.occupancies[i].to_string());
        }
        s
    }
}

/// row and col refer to the upper left corner of the piece bounding box
/// So the row and col of a piece will relatively changed based on rotation
/// @return (x,y)
fn coords_for_placement(
    piece: Piece,
    rot: Rotation,
    col: u8,
    row: u8,
) -> Vec<(u8, u8)> {
    piece.offsets(rot).iter().map(|(x, y)| (col + x, row + y)).collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Occupancy {
    Empty,
    Green,
    Red,
    Blue,
    Yellow,
}

impl ToString for Occupancy {
    fn to_string(&self) -> String {
        match self {
            Occupancy::Empty  => "·",
            Occupancy::Green  => "G",
            Occupancy::Red    => "R",
            Occupancy::Blue   => "B",
            Occupancy::Yellow => "Y",
        }.to_owned()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

// https://en.wikipedia.org/wiki/Blokus#/media/File:Blokus_tiles.svg
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    One,
    Two,
    ThreeL,
    ThreeI,
    FourI,
    FourL,
    FourStairs,
    FourSquare,
    FourT,
    FiveF,
    FiveI,
    FiveL,
    FiveN,
    FiveP,
    FiveT,
    FiveU,
    FiveV,
    FiveW,
    FiveX,
    FiveY,
    FiveZ,
}

impl Piece {
    /// @return (x,y)
    pub fn offsets(&self, rot: Rotation) -> Vec<(u8, u8)> {
        match self {
            Piece::One => vec!((0, 0)),
            Piece::Two => piece!(
                rot,
                XX
            ),
            Piece::ThreeI => piece!(
                rot,
                XXX
            ),
            Piece::ThreeL => piece!(
                rot,
                XX,
                X_
            ),
            Piece::FourI => piece!(
                rot,
                XXXX,
            ),
            Piece::FourL => piece!(
                rot,
                XXX,
                X__,
            ),
            Piece::FourStairs => piece!(
                rot,
                XX_,
                _XX,
            ),
            Piece::FourSquare => piece!(
                rot,
                XX,
                XX,
            ),
            Piece::FourT => piece!(
                rot,
                XXX,
                _X_,
            ),
            Piece::FiveF => piece!(
                rot,
                X__,
                XXX,
                _X_,
            ),
            Piece::FiveI => piece!(
                rot,
                XXXXX,
            ),
            Piece::FiveL => piece!(
                rot,
                XXXX,
                X___,
            ),
            Piece::FiveN => piece!(
                rot,
                XXX_,
                __XX,
            ),
            Piece::FiveP => piece!(
                rot,
                XXX,
                _XX,
            ),
            Piece::FiveT => piece!(
                rot,
                XXX,
                _X_,
                _X_,
            ),
            Piece::FiveU => piece!(
                rot,
                XXX,
                X_X,
            ),
            Piece::FiveV => piece!(
                rot,
                XXX,
                X__,
                X__,
            ),
            Piece::FiveW => piece!(
                rot,
                XX_,
                _XX,
                __X,
            ),
            Piece::FiveX => piece!(
                rot,
                _X_,
                XXX,
                _X_,
            ),
            Piece::FiveY => piece!(
                rot,
                _X,
                XX,
                _X,
                _X,
            ),
            Piece::FiveZ => piece!(
                rot,
                XX_,
                _X_,
                _XX
            ),
            _ => todo!()
        }
    }
}


/*  GG
GGG.......
G.G.......
.G.GBBBBB.
.G.G......
..........
..........
..........
........R.
........R.
........RR
*/

use std::{iter::Map, collections::HashMap};
