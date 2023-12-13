use crate::bitboard::PiecePlacement;
use crate::moves::Square;
use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Position {
    pub piece_placement: PiecePlacement,
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_target: u8,
    pub half_move_clock: u16,
    pub full_move_number: u16,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("11")
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn string(&self) -> &str {
        match self {
            Color::White => "White",
            Color::Black => "Black",
        }
    }
}

pub type CastlingRights = HashMap<Color, CastlingTypes>;

// king side and queen side castling
pub type CastlingTypes = (bool, bool);
