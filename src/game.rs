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
        row: usize,
        col: usize,
    ) -> bool {
        // Step 1: Create vec of new placements

        /*
        .....
        .^G..
        .GGG.
        ..G..
        ....G

        .....
        ..G..
        ..G..
        .....
        .....
        */

        true
    }
}

// poop!(
//     "XX",
// )

// poop!(
//     "XXX",
//     " X "
// )

// poop!(
//     XXX,
//     _X_,
// )


macro_rules! gen_piece_placements {
    ($col:expr, $row:expr, $rot:expr, $piece_map:expr) => {
        match $rot {
            Rotation::Zero => {
                coords.push((col, row));
                coords.push((col + 1, row));
            },
        }
    }
}

// Generates a symbol tokenizer match statemnt for ambiguous multi-char tokens
// macro_rules! gen_piece_placements {
//     ($col:expr, $row:expr, $rot:expr, $($extra:expr, $token:expr),*) => {
//         if $context.offset >= $context.content.len() {
//             Ok(($pos, $default))
//         } else {
//             let ch = unsafe {
//                 *$context.content.get_unchecked($context.offset)
//             };

//             match ch as char {
//                 $(
//                     $extra => {
//                         $context.offset += 1;
//                         Ok(($pos, $token))
//                     },
//                 )*
//                 _ => Ok(($pos, $default)),
//             }
//         }
//     };
// }

fn rot_piece_map(
    piece_map: Vec<Vec<u8>>,
    rot: Rotation,
) -> Vec<Vec<u8>> {
    match rot {
        Rotation::Zero       => piece_map,
        Rotation::Ninety     => rot_piece_90(piece_map),
        Rotation::OneEighty  => rot_piece_180(piece_map),
        Rotation::TwoSeventy => rot_piece_90(rot_piece_180(piece_map)),
    }
}

/// Requires all the strs to be the same length
pub fn rot_piece_90(
    piece_map: Vec<Vec<u8>>,
) -> Vec<Vec<u8>> {
    let col_count = piece_map[0].len();
    let row_count = piece_map.len();
    let mut output = vec![Vec::with_capacity(row_count); col_count];

    for col in (0..col_count).rev() {
        for row in piece_map.iter() {
            output[col_count - 1 - col].push(row[col]);
        }
    }
    output
}

type PieceMap<const LEN: usize> = [[u8; LEN]; LEN];

pub const fn piece_map_width<const LEN: usize>(
    piece_map: PieceMap<LEN>,
) -> usize {
    let mut max = 1;
    let mut row = 0;
    let mut col = 0;

    while row < LEN {
        while col < LEN {
            if col + 1 > max && piece_map[row][col] == 1 {
                max = col + 1;
            }
            col += 1;
        }
        row += 1;
    }
    max
}

pub fn rot_piece_180(
    piece_map: Vec<Vec<u8>>,
) -> Vec<Vec<u8>> {
    piece_map.into_iter()
        .map(|s| s.into_iter().rev().collect())
        .rev()
        .collect()
}

pub fn piece_map_to_string(
    piece_map: Vec<Vec<u8>>,
) -> String {
    piece_map.iter()
        .map(|row| row.iter().map(|t| t.to_string()).collect())
        .collect::<Vec<String>>()
        .join("\n")
}

/// row and col refer to the upper left corner of the piece bounding box
/// So the row and col of a piece will relatively changed based on rotation
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
        _ => todo!()
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(DIM * (DIM + 1));
        for i in 0..self.occupancies.len() {
            if i > 0 && i % DIM == 0 {
                s.push('\n');
            }

            s.push(match self.occupancies[i] {
                Occupancy::Empty  => '.',
                Occupancy::Green  => 'G',
                Occupancy::Red    => 'R',
                Occupancy::Blue   => 'B',
                Occupancy::Yellow => 'Y',
            });
        }
        s
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
