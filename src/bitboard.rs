use crate::{piece::PieceType, position::Color};

use core::fmt;
use std::{char, collections::HashMap, ops::AddAssign, string};

pub type Bitboard = u64;

fn bitboard_rep(bb: Bitboard) -> String {
    unimplemented!()
}

#[derive(Debug)]
pub struct PieceBitboard(pub HashMap<PieceType, Bitboard>);

#[derive(Debug)]
pub struct PiecePlacement(pub HashMap<Color, PieceBitboard>);

impl fmt::Display for PiecePlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut piece_vectors: Vec<Vec<char>> = Vec::new();
        for color in Color::iterator() {
            for piece in PieceType::iterator() {
                piece_vectors.push(replace_piece(
                    format!("{:64b}", self.0.get(color).unwrap().0.get(piece).unwrap())
                        .chars()
                        .collect(),
                    piece.piece_rep(color),
                ));
            }
        }

        let piece_vector = add_char_vecs(piece_vectors);

        let mut board_string_representation: String = "Piece placement: \n".to_string();
        for i in 0..8 {
            board_string_representation += "\t -————-————-————-————-————-————-————-————-\n\t";

            for j in (0..=7).rev() {
                let index = i * 8 + j;
                board_string_representation += " | ";
                board_string_representation += &piece_vector[index].to_string();
                board_string_representation += " ";
            }
            board_string_representation += " | ";
            board_string_representation += (8 - i).to_string().as_str();
            board_string_representation += "\n";
        }
        board_string_representation += "\t -————-————-————-————-————-————-————-————-\n";
        board_string_representation += "\t    a    b    c    d    e    f    g    h  \n";

        f.write_str(board_string_representation.as_str())
    }
}

fn replace_piece(mut char_vector: Vec<char>, piece_representation: char) -> Vec<char> {
    for ch in &mut char_vector {
        match *ch {
            '1' => *ch = piece_representation,
            _ => *ch = ' ',
        }
    }
    return char_vector;
}

fn add_char_vecs(char_vectors: Vec<Vec<char>>) -> Vec<char> {
    let mut aggregated_vector: Vec<char> = vec![' '; 64];
    for char_vector in char_vectors {
        for (i, &c) in char_vector.iter().enumerate() {
            match c {
                ' ' => {}
                _ => aggregated_vector[i] = c,
            }
        }
    }
    return aggregated_vector;
}
