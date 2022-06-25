use bevy::prelude::Color;
#[allow(dead_code)]
use weblock_codegen::*;

mod grid;
use self::grid::*;

pub const DIM: usize = 20;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub occupancies: Grid<Occupancy, DIM>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            occupancies: Grid::new(),
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

        if self.is_placement_valid(occupancy, &coords) {
            for (x, y) in coords {
                self.occupancies.set(x, y, occupancy);
            }
            true
        } else {
            false
        }
    }

    // Returns false if the placement is invalid
    pub fn is_placement_valid(&mut self, occupancy: Occupancy, coords: &Vec<(i8, i8)>) -> bool {
        let mut has_anchor = false;

        for (x, y) in coords.iter() {
            if *x as usize >= DIM || *y as usize >= DIM {
                return false;
            }
            if self.occupancies.get(*x, *y) != Occupancy::Empty {
                return false;
            }
            if self.touching_sides(occupancy, *x, *y) {
                return false;
            }
            if self.touching_tips(occupancy, *x, *y) {
                has_anchor = true;
            }
        }
        has_anchor
    }

    /// Print the valid, invalid, and docking placements for the given occupancy
    /// Assumes occupancy is not [`Occupancy::Empty`]
    pub fn print_placements(&self, occupancy: Occupancy) {
        for y in 0..DIM as i8 {
            for x in 0..DIM as i8 {
                if self.occupancies.get(x, y) != Occupancy::Empty
                    || self.touching_sides(occupancy, x, y)
                {
                    print!("X");
                } else if self.touching_tips(occupancy, x, y) {
                    print!("O");
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }

    pub fn vomit_placements(&self, occupancy: Occupancy, placements: &mut Grid<Validity, DIM>) {
        for y in 0..DIM as i8 {
            for x in 0..DIM as i8 {
                if self.occupancies.get(x, y) != Occupancy::Empty
                    || self.touching_sides(occupancy, x, y)
                {
                    placements.set(x, y, Validity::Invalid);
                } else if self.touching_tips(occupancy, x, y) {
                    // TODO: All corners should also be anchors
                    placements.set(x, y, Validity::Anchor);
                } else {
                    placements.set(x, y, Validity::Valid);
                }
            }
        }
    }

    /// Checks if the sides are touching the same occupancy
    fn touching_sides(&self, occupancy: Occupancy, x: i8, y: i8) -> bool {
        self.occupancies.get_opt(x - 1, y) == Some(occupancy)
            || self.occupancies.get_opt(x + 1, y) == Some(occupancy)
            || self.occupancies.get_opt(x, y - 1) == Some(occupancy)
            || self.occupancies.get_opt(x, y + 1) == Some(occupancy)
    }

    /// Checks if the corners are touching the same occupancy
    fn touching_tips(&self, occupancy: Occupancy, x: i8, y: i8) -> bool {
        if (x == 0 || x == DIM as i8 - 1) && (y == 0 || y == DIM as i8 - 1) {
            return true;
        }

        self.occupancies.get_opt(x - 1, y - 1) == Some(occupancy)
            || self.occupancies.get_opt(x - 1, y + 1) == Some(occupancy)
            || self.occupancies.get_opt(x + 1, y - 1) == Some(occupancy)
            || self.occupancies.get_opt(x + 1, y + 1) == Some(occupancy)
    }
}

/// row and col refer to the upper left corner of the piece bounding box
/// So the row and col of a piece will relatively changed based on rotation
/// @return (x,y)
pub fn coords_for_placement(piece: Piece, rot: Rotation, col: i8, row: i8) -> Vec<(i8, i8)> {
    piece
        .offsets(rot)
        .offsets
        .iter()
        .map(|(x, y)| (col + x, row + y))
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Validity {
    Valid,
    Invalid,
    Anchor,
}

impl Default for Validity {
    fn default() -> Self {
        Self::Valid
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

impl Occupancy {
    pub fn color(self) -> Color {
        match self {
            Occupancy::Empty => Color::rgba(0., 0., 0., 0.),
            Occupancy::Green => Color::hex("0cca4a").unwrap(),
            Occupancy::Red => Color::hex("fb3640").unwrap(),
            Occupancy::Blue => Color::hex("2892d7").unwrap(),
            Occupancy::Yellow => Color::hex("ffba49").unwrap(),
        }
    }
}

impl Default for Occupancy {
    fn default() -> Self {
        Self::Empty
    }
}

impl ToString for Occupancy {
    fn to_string(&self) -> String {
        match self {
            Occupancy::Empty => "·",
            Occupancy::Green => "G",
            Occupancy::Red => "R",
            Occupancy::Blue => "B",
            Occupancy::Yellow => "Y",
        }
        .to_owned()
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
            Rotation::Zero => Rotation::TwoSeventy,
            Rotation::Ninety => Rotation::Zero,
            Rotation::OneEighty => Rotation::Ninety,
            Rotation::TwoSeventy => Rotation::OneEighty,
        }
    }
}

#[derive(Debug)]
pub struct PieceOffsets {
    pub offsets: Vec<(i8, i8)>,
    pub pivot: (i8, i8),
}

impl PieceOffsets {
    pub fn print_repr(&self) {
        let mut grid = Grid::<char, 5>::new();
        for x in 0..5 {
            for y in 0..5 {
                grid.set(x, y, '-');
            }
        }
        for (x, y) in &self.offsets {
            grid.set(*x, *y, 'X');
        }
        println!("{}", grid.to_string());
    }
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
            Piece::One => piece!(rot, O),
            Piece::Two => piece!(rot, OX),
            Piece::ThreeI => piece!(rot, XOX),
            Piece::ThreeL => piece!(rot, OX, X_),
            Piece::FourI => piece!(rot, XOXX,),
            Piece::FourL => piece!(rot, OXX, X__,),
            Piece::FourStairs => piece!(rot, XO_, _XX,),
            Piece::FourSquare => piece!(rot, OX, XX,),
            Piece::FourT => piece!(rot, XOX, _X_,),
            Piece::FiveF => piece!(rot,
                                   X__,
                                   XOX,
                                   _X_,),
            Piece::FiveI => piece!(rot, XXOXX,),
            Piece::FiveL => piece!(rot, OXXX, X___,),
            Piece::FiveN => piece!(rot, XXO_, __XX,),
            Piece::FiveP => piece!(rot, XOX, _XX,),
            Piece::FiveT => piece!(rot, XXX, _O_, _X_,),
            Piece::FiveU => piece!(rot, XOX, X_X,),
            Piece::FiveV => piece!(rot, OXX, X__, X__,),
            Piece::FiveW => piece!(rot, XX_, _OX, __X,),
            Piece::FiveX => piece!(rot, _X_, XOX, _X_,),
            Piece::FiveY => piece!(rot, _X, XO, _X, _X,),
            Piece::FiveZ => piece!(rot, XX_, _O_, _XX),
        }
    }
}
