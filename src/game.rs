use bevy::prelude::Color;
#[allow(dead_code)]
use weblock_codegen::*;

pub const DIM: usize = 20;

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
        col: i8,
        row: i8,
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

    pub fn get(&self, x: i8, y: i8) -> Occupancy {
        self.occupancies[x as usize + y as usize * DIM]
    }

    fn set(&mut self, x: i8, y: i8, value: Occupancy) {
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
    col: i8,
    row: i8,
) -> Vec<(i8, i8)> {
    piece.offsets(rot).offsets.iter().map(|(x, y)| (col + x, row + y)).collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Occupancy {
    Empty,
    Green,
    Red,
    Blue,
    Yellow,
}

impl Occupancy {
    pub fn color(self) -> Color {
        match self {
            Occupancy::Empty  => Color::rgba(0., 0., 0., 0.),
            Occupancy::Green  => Color::hex("0cca4a").unwrap(),
            Occupancy::Red    => Color::hex("fb3640").unwrap(),
            Occupancy::Blue   => Color::hex("2892d7").unwrap(),
            Occupancy::Yellow => Color::hex("ffba49").unwrap(),
        }
    }
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

impl Rotation {
    pub fn next_clockwise(self) -> Rotation {
        match self {
            Rotation::Zero       => Rotation::TwoSeventy,
            Rotation::Ninety     => Rotation::Zero,
            Rotation::OneEighty  => Rotation::Ninety,
            Rotation::TwoSeventy => Rotation::OneEighty,
        }
    }
}

pub struct PieceOffsets {
    pub offsets: Vec<(i8, i8)>,
    pub pivot: (i8, i8),
}

// https://en.wikipedia.org/wiki/Blokus#/media/File:Blokus_tiles.svg
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
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
    pub fn offsets(&self, rot: Rotation) -> PieceOffsets {
        match self {
            Piece::One => piece!(
                rot,
                O
            ),
            Piece::Two => piece!(
                rot,
                OX
            ),
            Piece::ThreeI => piece!(
                rot,
                XOX
            ),
            Piece::ThreeL => piece!(
                rot,
                OX,
                X_
            ),
            Piece::FourI => piece!(
                rot,
                XOXX,
            ),
            Piece::FourL => piece!(
                rot,
                OXX,
                X__,
            ),
            Piece::FourStairs => piece!(
                rot,
                XO_,
                _XX,
            ),
            Piece::FourSquare => piece!(
                rot,
                OX,
                XX,
            ),
            Piece::FourT => piece!(
                rot,
                XOX,
                _X_,
            ),
            Piece::FiveF => piece!(
                rot,
                X__,
                XOX,
                _X_,
            ),
            Piece::FiveI => piece!(
                rot,
                XXOXX,
            ),
            Piece::FiveL => piece!(
                rot,
                OXXX,
                X___,
            ),
            Piece::FiveN => piece!(
                rot,
                XXO_,
                __XX,
            ),
            Piece::FiveP => piece!(
                rot,
                XOX,
                _XX,
            ),
            Piece::FiveT => piece!(
                rot,
                XXX,
                _O_,
                _X_,
            ),
            Piece::FiveU => piece!(
                rot,
                XOX,
                X_X,
            ),
            Piece::FiveV => piece!(
                rot,
                OXX,
                X__,
                X__,
            ),
            Piece::FiveW => piece!(
                rot,
                XX_,
                _OX,
                __X,
            ),
            Piece::FiveX => piece!(
                rot,
                _X_,
                XOX,
                _X_,
            ),
            Piece::FiveY => piece!(
                rot,
                _X,
                XO,
                _X,
                _X,
            ),
            Piece::FiveZ => piece!(
                rot,
                XX_,
                _O_,
                _XX
            ),
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
