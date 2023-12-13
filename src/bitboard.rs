use crate::{pieces::PieceType, position::Color};

use std::collections::HashMap;

pub type Bitboard = u64;

pub type PieceBitboard = HashMap<PieceType, Bitboard>;

pub type PiecePlacement = HashMap<Color, PieceBitboard>;
