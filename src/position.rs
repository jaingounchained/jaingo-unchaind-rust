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
    pub castling_rights: Option<CastlingRights>,
    pub en_passant_target: Option<Square>,
    pub half_move_clock: u16,
    pub full_move_number: u16,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut position_representation: String =
            format!("Piece placement:\n {}\n", self.piece_placement);
        position_representation = format!(
            "{}Active color: {}\n",
            position_representation, self.active_color
        );
        match &self.castling_rights {
            Some(castling_rights) => {
                position_representation = format!(
                    "{}Castling rights:\n{}\n",
                    position_representation, castling_rights
                )
            }
            None => {}
        }
        match &self.en_passant_target {
            Some(en_passant_target) => {
                position_representation = format!(
                    "{}En passant target: {}\n",
                    position_representation, en_passant_target
                )
            }
            None => {}
        }
        position_representation = format!(
            "{}Half move clock: {}\n",
            position_representation, self.half_move_clock
        );
        position_representation = format!(
            "{}Full move number: {}\n",
            position_representation, self.full_move_number
        );
        f.write_str(&position_representation)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
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

    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 2] = [Color::White, Color::Black];
        COLORS.iter()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[derive(Debug)]
// king side and queen side castling
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
