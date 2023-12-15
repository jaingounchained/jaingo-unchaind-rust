use crate::bitboard::PiecePlacement;
use crate::moves::Square;
use core::fmt;
use std::collections::HashMap;
use std::fmt::write;
use std::slice::Iter;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
            "\n{}\nActive color: {}\n\nCastling rights:\n{}\n\nHalf move clock: {}\n\nFull move number: {}\n",
            self.piece_placement,
            self.active_color,
            self.castling_rights,
            self.half_move_clock,
            self.full_move_number,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Color {
    WHITE,
    BLACK,
}

impl Color {
    fn string(&self) -> &str {
        match self {
            Color::WHITE => "White",
            Color::BLACK => "Black",
        }
    }

    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 2] = [Color::WHITE, Color::BLACK];
        COLORS.iter()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
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
            self.0.get(&Color::WHITE).unwrap(),
            self.0.get(&Color::BLACK).unwrap(),
        )
    }
}
