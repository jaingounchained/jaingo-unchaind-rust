use crate::bitboard::PiecePlacement;
use crate::moves::Square;
use core::fmt;
use std::collections::HashMap;
use std::fmt::write;

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
        write!(
            f,
            "\n{}\nActive {}\n\nCastling rights:\n{}\n\nHalf move clock: {}\n\nFull move number: {}\n",
            self.piece_placement,
            self.active_color,
            self.castling_rights,
            self.half_move_clock,
            self.full_move_number,
        )
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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color: {}", self.string())
    }
}

// king side and queen side castling
#[derive(Debug)]
pub struct CastlingTypes(pub bool, pub bool);

impl fmt::Display for CastlingTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tKing side: {}\n\tQueen side: {}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct CastlingRights(pub HashMap<Color, CastlingTypes>);

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "White: \n{}\nBlack: \n{}",
            self.0.get(&Color::White).unwrap(),
            self.0.get(&Color::Black).unwrap(),
        )
    }
}
