use crate::bitboard::{Bitboard, KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS};
use crate::moves::{Move, Square};
use crate::position::{Color, Position};
use core::fmt;
use std::char;
use std::slice::Iter;

use lazy_static::lazy_static;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
        write!(f, "{}", self.string())
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

    pub fn piece_rep(&self, color: &Color) -> char {
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
                PieceType::Pawn => std::char::from_u32(0x265F).unwrap(),
                PieceType::Knight => std::char::from_u32(0x265E).unwrap(),
                PieceType::Bishop => std::char::from_u32(0x265D).unwrap(),
                PieceType::Rook => std::char::from_u32(0x265C).unwrap(),
                PieceType::Queen => std::char::from_u32(0x265B).unwrap(),
                PieceType::King => std::char::from_u32(0x265A).unwrap(),
            },
        }
    }

    pub fn is_slider(&self) -> bool {
        match self {
            Self::Pawn | Self::Knight | Self::King => false,
            Self::Bishop | Self::Rook | Self::Queen => true,
        }
    }

    pub fn iterator() -> Iter<'static, PieceType> {
        static PIECETYPES: [PieceType; 6] = [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ];
        PIECETYPES.iter()
    }

    pub fn generate_moves(position: Position) -> Vec<Move> {
        unimplemented!()
    }

    pub fn attact_bitboard(
        &self,
        color: &Color,
        square: &Square,
        our_squares: &Bitboard,
        opponent_squares: &Bitboard,
    ) -> Bitboard {
        let all_occupied = our_squares | opponent_squares;

        match self {
            Self::King => KING_ATTACKS[square.0 as usize],
            Self::Queen => {
                Slider::Diagonal.slider_attacks(&square, &all_occupied)
                    | Slider::AntiDiagonal.slider_attacks(&square, &all_occupied)
                    | Slider::File.slider_attacks(&square, &all_occupied)
                    | Slider::Rank.slider_attacks(&square, &all_occupied)
            }
            Self::Rook => {
                Slider::File.slider_attacks(&square, &all_occupied)
                    | Slider::Rank.slider_attacks(&square, &all_occupied)
            }
            Self::Bishop => {
                Slider::Diagonal.slider_attacks(&square, &all_occupied)
                    | Slider::AntiDiagonal.slider_attacks(&square, &all_occupied)
            }
            Self::Knight => KNIGHT_ATTACKS[square.0 as usize],
            Self::Pawn => PAWN_ATTACKS.get(&color).unwrap()[square.0 as usize],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Slider {
    File,
    Rank,
    Diagonal,
    AntiDiagonal,
}

struct SquareMask {
    bit_mask: Bitboard,
    slider_mask_ex: HashMap<Slider, Bitboard>,
}

lazy_static! {
    static ref SQUARE_MASKS: [SquareMask; 64] = { unimplemented!() };
}

impl Slider {
    /*
        Calculating
         - file attacks
         - rank attacks
         - diagonal attacks
         - antidiagonal attacks
        using Hyperbola Quintessence
        https://www.chessprogramming.org/Hyperbola_Quintessence
    */
    fn slider_attacks(&self, square: &Square, occupied_squares: &Bitboard) -> Bitboard {
        let (mut forward, mut reverse) = (0, 0);

        let square_mask = &SQUARE_MASKS[square.0 as usize];

        // (o-r): masking the file & subtracting the sqaure
        forward = occupied_squares & square_mask.slider_mask_ex.get(self).unwrap();
        reverse = forward.reverse_bits();
        // (o-2r)
        forward -= square_mask.bit_mask;
        reverse -= square_mask.bit_mask.reverse_bits();
        // (o-2r)^rev(o'-2r')
        forward ^= reverse.reverse_bits();
        forward &= square_mask.slider_mask_ex.get(self).unwrap();

        return forward;
    }
}
