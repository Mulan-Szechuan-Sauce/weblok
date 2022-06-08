#[allow(dead_code)]
use weblock_codegen::*;

const DIM: usize = 20;

pub struct GameState {
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Board {
    occupancies: [Occupancy; DIM * DIM],
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

    fn get(&self, x: u8, y: u8) -> Occupancy {
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
    match piece {
        Piece::One => vec!((col, row)),
        Piece::Two => piece!(
            col, row, rot,
            XX
        ),
        Piece::ThreeI => piece!(
            col, row, rot,
            XXX
        ),
        Piece::ThreeL => piece!(
            col, row, rot,
            XX,
            X_
        ),
        Piece::FourI => piece!(
            col, row, rot,
            XXXX,
        ),
        Piece::FourL => piece!(
            col, row, rot,
            XXX,
            X__,
        ),
        Piece::FourStairs => piece!(
            col, row, rot,
            XX_,
            _XX,
        ),
        Piece::FourSquare => piece!(
            col, row, rot,
            XX,
            XX,
        ),
        Piece::FourT => piece!(
            col, row, rot,
            XXX,
            _X_,
        ),
        Piece::FiveF => piece!(
            col, row, rot,
            X__,
            XXX,
            _X_,
        ),
        Piece::FiveI => piece!(
            col, row, rot,
            XXXXX,
        ),
        Piece::FiveL => piece!(
            col, row, rot,
            XXXX,
            X___,
        ),
        Piece::FiveN => piece!(
            col, row, rot,
            XXX_,
            __XX,
        ),
        Piece::FiveP => piece!(
            col, row, rot,
            XXX,
            _XX,
        ),
        Piece::FiveT => piece!(
            col, row, rot,
            XXX,
            _X_,
            _X_,
        ),
        Piece::FiveU => piece!(
            col, row, rot,
            XXX,
            X_X,
        ),
        Piece::FiveV => piece!(
            col, row, rot,
            XXX,
            X__,
            X__,
        ),
        Piece::FiveW => piece!(
            col, row, rot,
            XX_,
            _XX,
            __X,
        ),
        Piece::FiveX => piece!(
            col, row, rot,
            _X_,
            XXX,
            _X_,
        ),
        Piece::FiveY => piece!(
            col, row, rot,
            _X,
            XX,
            _X,
            _X,
        ),
        Piece::FiveZ => piece!(
            col, row, rot,
            XX_,
            _X_,
            _XX
        ),
        _ => todo!()
    }
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
