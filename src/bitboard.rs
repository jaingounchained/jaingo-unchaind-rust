use lazy_static::lazy_static;

use crate::{moves::Square, piece::PieceType, position::Color, utils::log2n};

use core::fmt;
use std::{char, collections::HashMap, fmt::format, ops::BitOrAssign, usize};

pub type Bitboard = u64;

fn left_most_significant_square(bitboard: &Bitboard) -> Square {
    return Square(log2n(&(bitboard & bitboard.wrapping_neg())).unwrap() as u8);
}

fn remove_pieces(bitboard: &Bitboard, our_squares: &Bitboard) -> Bitboard {
    return bitboard & !our_squares;
}

fn shift_direction(bitboard: &Bitboard, directions: &[Direction]) -> Bitboard {
    let mut transformed_bitboard: Bitboard = *bitboard;
    for direction in directions {
        match direction {
            Direction::West | Direction::NorthWest | Direction::SouthWest => {
                transformed_bitboard =
                    bit_shift(&transformed_bitboard, *direction as i64) & NOT_H_FILE
            }
            Direction::East | Direction::NorthEast | Direction::SouthEast => {
                transformed_bitboard =
                    bit_shift(&transformed_bitboard, *direction as i64) & NOT_A_FILE
            }
            Direction::North | Direction::South => {
                transformed_bitboard = bit_shift(&transformed_bitboard, *direction as i64)
            }
        }
    }
    return transformed_bitboard;
}

fn bit_shift(bitboard: &Bitboard, n: i64) -> Bitboard {
    if n >= 0 {
        return bitboard << n;
    } else {
        return bitboard >> -n;
    }
}

#[derive(Debug)]
pub struct PieceBitboard(pub HashMap<PieceType, Bitboard>);

#[derive(Debug)]
pub struct PiecePlacement(pub HashMap<Color, PieceBitboard>);

static NOT_A_FILE: Bitboard = 0xfefefefefefefefe;
static NOT_H_FILE: Bitboard = 0x7f7f7f7f7f7f7f7f;
static NOT_AB_FILE: Bitboard = 0xFCFCFCFCFCFCFCFC;
static NOT_GH_FILE: Bitboard = 0x3F3F3F3F3F3F3F3F;

lazy_static! {
    pub static ref KING_ATTACKS: [Bitboard; 64] = {
        let mut king_attacks: [Bitboard; 64] = [0; 64];

        for i in 0..64 {
            let square_bb: Bitboard = 1 << i;

            king_attacks[i] = shift_direction(&square_bb, &[Direction::South])
                | shift_direction(&square_bb, &[Direction::North])
                | shift_direction(&square_bb, &[Direction::East])
                | shift_direction(&square_bb, &[Direction::West])
                | shift_direction(&square_bb, &[Direction::NorthWest])
                | shift_direction(&square_bb, &[Direction::NorthEast])
                | shift_direction(&square_bb, &[Direction::SouthWest])
                | shift_direction(&square_bb, &[Direction::SouthEast]);
        }

        king_attacks
    };
    pub static ref KNIGHT_ATTACKS: [Bitboard; 64] = {
        let mut knight_attacks: [Bitboard; 64] = [0; 64];

        for i in 0..64 {
            let square_bb: Bitboard = 1 << i;

            knight_attacks[i] =
                shift_direction(&square_bb, &[Direction::South, Direction::SouthWest])
                    | shift_direction(&square_bb, &[Direction::South, Direction::SouthEast])
                    | shift_direction(&square_bb, &[Direction::East, Direction::SouthEast])
                    | shift_direction(&square_bb, &[Direction::East, Direction::NorthEast])
                    | shift_direction(&square_bb, &[Direction::North, Direction::NorthWest])
                    | shift_direction(&square_bb, &[Direction::North, Direction::NorthEast])
                    | shift_direction(&square_bb, &[Direction::West, Direction::SouthWest])
                    | shift_direction(&square_bb, &[Direction::West, Direction::NorthWest])
        }

        knight_attacks
    };
    pub static ref PAWN_ATTACKS: HashMap<Color, [Bitboard; 64]> = {
        let mut pawn_attacks: HashMap<Color, [Bitboard; 64]> = HashMap::new();
        let (mut white_pawn_attacks, mut black_pawn_attacks) = ([0; 64], [0; 64]);

        for i in 0..64 {
            let square_bb: Bitboard = 1 << i;

            white_pawn_attacks[i] = shift_direction(&square_bb, &[Direction::NorthWest])
                | shift_direction(&square_bb, &[Direction::NorthEast]);

            black_pawn_attacks[i] = shift_direction(&square_bb, &[Direction::SouthWest])
                | shift_direction(&square_bb, &[Direction::SouthEast]);
        }

        pawn_attacks.insert(Color::White, white_pawn_attacks);
        pawn_attacks.insert(Color::Black, black_pawn_attacks);

        pawn_attacks
    };
}

#[derive(Clone, Copy)]
enum Direction {
    North = 8,
    East = 1,
    South = -8,
    West = -1,

    NorthEast = Direction::North as isize + Direction::East as isize,
    SouthEast = Direction::South as isize + Direction::East as isize,
    SouthWest = Direction::South as isize + Direction::West as isize,
    NorthWest = Direction::North as isize + Direction::West as isize,
}

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

        let mut board_string_representation: String = String::from("");
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

pub fn bitboard_representation(bb: Bitboard) -> String {
    let mut bitboard_aux_string = format!("{:0>64b}", bb);
    bitboard_aux_string = bitboard_aux_string.replace("0", ".");

    let mut bitboard_string = String::from("");
    for i in 0..8 {
        for j in (0..8).rev() {
            let index = i * 8 + j;
            bitboard_string = format!(
                "{}{} ",
                bitboard_string,
                bitboard_aux_string.chars().nth(index).unwrap(),
            )
        }
        bitboard_string = format!("{}\n", bitboard_string)
    }
    return bitboard_string;
}
