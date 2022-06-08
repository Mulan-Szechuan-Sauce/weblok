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

#[proc_macro]
pub fn piece(input: TokenStream) -> TokenStream {
    let Arguments(args_parsed) = parse_macro_input!(input as Arguments);

    let col = &args_parsed[0];
    let row = &args_parsed[1];
    let rot = &args_parsed[2];

    let mask_zero: Vec<Vec<u8>> = args_parsed[3..]
        .iter()
        .map(|id| {
            let s = id.to_string();

            s.chars().map(|c| if c == '_' { 0 } else { 1 }).collect()
        })
        .collect();

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
    let coords_zero = mask_to_coords(&mask_zero);
    let coords_ninety = mask_to_coords(&mask_ninety);
    let coords_one_eighty = mask_to_coords(&mask_one_eighty);
    let coords_two_seventy = mask_to_coords(&mask_two_seventy);

    let expanded = quote! {
        match #rot {
            Rotation::Zero       => vec![#(#coords_zero,)*],
            Rotation::Ninety     => vec![#(#coords_ninety,)*],
            Rotation::OneEighty  => vec![#(#coords_one_eighty,)*],
            Rotation::TwoSeventy => vec![#(#coords_two_seventy,)*],
        }.iter().map(|(x, y)| (#col + x, #row + y)).collect()
    };

    TokenStream::from(expanded)
}

fn mask_to_coords(piece_map: &Vec<Vec<u8>>) -> Vec<Coord> {
    let col_count = piece_map[0].len();
    let row_count = piece_map.len();
    let mut coords = vec![];

    for col in 0..col_count {
        for row in 0..row_count {
            if piece_map[row][col] == 1 {
                coords.push(Coord((col as u8, row as u8)));
            }
        }
    }
    coords
}

/// Requires all the strs to be the same length
fn rot_piece_90(piece_map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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

fn rot_piece_180(piece_map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    piece_map
        .iter()
        .map(|s| s.iter().rev().map(|x| *x).collect())
        .rev()
        .collect()
}

struct Coord((u8, u8));

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
