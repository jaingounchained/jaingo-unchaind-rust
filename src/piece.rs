use crate::position::Color;
use core::fmt;
use std::char;
use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

impl PieceType {
    fn string(&self) -> &str {
        match self {
            PieceType::PAWN => "Pawn",
            PieceType::KNIGHT => "Knight",
            PieceType::BISHOP => "Bishop",
            PieceType::ROOK => "Rook",
            PieceType::QUEEN => "Queen",
            PieceType::KING => "King",
        }
    }

    pub fn piece_rep(&self, color: &Color) -> char {
        match color {
            Color::WHITE => match self {
                PieceType::PAWN => std::char::from_u32(0x2659).unwrap(),
                PieceType::KNIGHT => std::char::from_u32(0x2658).unwrap(),
                PieceType::BISHOP => std::char::from_u32(0x2657).unwrap(),
                PieceType::ROOK => std::char::from_u32(0x2656).unwrap(),
                PieceType::QUEEN => std::char::from_u32(0x2655).unwrap(),
                PieceType::KING => std::char::from_u32(0x2654).unwrap(),
            },
            Color::BLACK => match self {
                PieceType::PAWN => std::char::from_u32(0x265F).unwrap(),
                PieceType::KNIGHT => std::char::from_u32(0x265E).unwrap(),
                PieceType::BISHOP => std::char::from_u32(0x265D).unwrap(),
                PieceType::ROOK => std::char::from_u32(0x265C).unwrap(),
                PieceType::QUEEN => std::char::from_u32(0x265B).unwrap(),
                PieceType::KING => std::char::from_u32(0x265A).unwrap(),
            },
        }
    }

    pub fn is_slider(&self) -> bool {
        match self {
            Self::PAWN | Self::KNIGHT | Self::KING => false,
            Self::BISHOP | Self::ROOK | Self::QUEEN => true,
        }
    }

    pub fn iterator() -> Iter<'static, PieceType> {
        static PIECETYPES: [PieceType; 6] = [
            PieceType::PAWN,
            PieceType::KNIGHT,
            PieceType::BISHOP,
            PieceType::ROOK,
            PieceType::QUEEN,
            PieceType::KING,
        ];
        PIECETYPES.iter()
    }
}
