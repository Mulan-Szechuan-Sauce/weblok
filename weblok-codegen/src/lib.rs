use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Ident};

struct Arguments(Vec<Ident>);

impl Parse for Arguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let thing: Vec<Ident> = Punctuated::<Ident, Comma>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(Arguments(thing))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceChar {
    Solid,
    Pivot,
    Empty,
}

#[proc_macro]
pub fn piece(input: TokenStream) -> TokenStream {
    let Arguments(args_parsed) = parse_macro_input!(input as Arguments);

    let rot = &args_parsed[0];

    let mask_zero_res: Result<Vec<Vec<_>>, TokenStream> = args_parsed[1..]
        .iter()
        .map(|id| {
            let s = id.to_string();

            s.chars()
                .map(|c| match c {
                    'X' => Ok(PieceChar::Solid),
                    'O' => Ok(PieceChar::Pivot),
                    '_' => Ok(PieceChar::Empty),
                    c => Err(make_error(&format!("Invalid char {c}"))),
                })
                .collect()
        })
        .collect();

    if let Err(e) = mask_zero_res {
        return e;
    }
    let mask_zero = mask_zero_res.unwrap();

    let expected_len = mask_zero
        .iter()
        .max_by_key(|row| row.len())
        .unwrap_or(&vec![])
        .len();
    let is_valid = mask_zero
        .iter()
        .map(|row| row.len())
        .rfold(true, |acc, it| acc && expected_len == it);

    if !is_valid {
        return make_error(&format!("Expected every row to be {} long", expected_len));
    }

    let mask_ninety = rot_piece_90(&mask_zero);
    let mask_one_eighty = rot_piece_180(&mask_zero);
    let mask_two_seventy = rot_piece_180(&rot_piece_90(&mask_zero));

    // NOW begins the fun
    let (coords_zero, pivot_zero) = mask_to_coords(&mask_zero);
    let (coords_ninety, pivot_ninety) = mask_to_coords(&mask_ninety);
    let (coords_one_eighty, pivot_one_eighty) = mask_to_coords(&mask_one_eighty);
    let (coords_two_seventy, pivot_two_seventy) = mask_to_coords(&mask_two_seventy);

    let expanded = quote! {
        match #rot {
            Rotation::Zero => crate::game::PieceOffsets {
                offsets: vec![#(#coords_zero,)*],
                pivot: #pivot_zero,
            },
            Rotation::Ninety => crate::game::PieceOffsets {
                offsets: vec![#(#coords_ninety,)*],
                pivot: #pivot_ninety,
            },
            Rotation::OneEighty => crate::game::PieceOffsets {
                offsets: vec![#(#coords_one_eighty,)*],
                pivot: #pivot_one_eighty,
            },
            Rotation::TwoSeventy => crate::game::PieceOffsets {
                offsets: vec![#(#coords_two_seventy,)*],
                pivot: #pivot_two_seventy,
            }
        }
    };

    TokenStream::from(expanded)
}

fn mask_to_coords(piece_map: &Vec<Vec<PieceChar>>) -> (Vec<Coord>, Coord) {
    let col_count = piece_map[0].len();
    let row_count = piece_map.len();
    let mut coords = vec![];
    let mut pivot = Coord((0, 0));

    for col in 0..col_count {
        for row in 0..row_count {
            if piece_map[row][col] != PieceChar::Empty {
                coords.push(Coord((col as i8, row as i8)));
            }
            if piece_map[row][col] == PieceChar::Pivot {
                pivot = Coord((col as i8, row as i8));
            }
        }
    }
    (coords, pivot)
}

/// Requires all the strs to be the same length
fn rot_piece_90<T: Copy>(piece_map: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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

fn rot_piece_180<T: Copy>(piece_map: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    piece_map
        .iter()
        .map(|s| s.iter().rev().map(|x| *x).collect())
        .rev()
        .collect()
}

struct Coord((i8, i8));

impl ToTokens for Coord {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Coord((zero, one)) = self;
        tokens.extend(quote! {
            (#zero, #one)
        });
    }
}

fn make_error(message: &str) -> TokenStream {
    TokenStream::from(quote!(compile_error!(#message)))
}
