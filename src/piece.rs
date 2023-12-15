use crate::position::Color;
use core::fmt;
use std::char;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pawn => f.write_str(""),
            _ => f.write_str(""),
        }
    }
}

impl PieceType {
    fn string(&self) -> &str {
        match self {
            PieceType::Pawn => "Pawn",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Rook => "Rook",
            PieceType::Queen => "Queen",
            PieceType::King => "King",
        }
    }

    fn piece_rep(&self, color: Color) -> char {
        match color {
            Color::White => match self {
                PieceType::Pawn => std::char::from_u32(0x2659).unwrap(),
                PieceType::Knight => std::char::from_u32(0x2658).unwrap(),
                PieceType::Bishop => std::char::from_u32(0x2657).unwrap(),
                PieceType::Rook => std::char::from_u32(0x2656).unwrap(),
                PieceType::Queen => std::char::from_u32(0x2655).unwrap(),
                PieceType::King => std::char::from_u32(0x2654).unwrap(),
            },
            Color::Black => match self {
                PieceType::Pawn => std::char::from_u32(0x265A).unwrap(),
                PieceType::Knight => std::char::from_u32(0x265B).unwrap(),
                PieceType::Bishop => std::char::from_u32(0x265C).unwrap(),
                PieceType::Rook => std::char::from_u32(0x265D).unwrap(),
                PieceType::Queen => std::char::from_u32(0x265E).unwrap(),
                PieceType::King => std::char::from_u32(0x265F).unwrap(),
            },
        }
    }

    fn is_slider(&self) -> bool {
        match self {
            Self::Pawn | Self::Knight | Self::King => false,
            Self::Bishop | Self::Rook | Self::Queen => true,
        }
    }
}
