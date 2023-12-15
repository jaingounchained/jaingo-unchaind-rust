use crate::{piece::PieceType, position::Color};

use core::fmt;
use std::{collections::HashMap, ops::AddAssign, string};

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
        write!(
            f,
            "Piece placement: \n\t -————-————-————-————-————-————-————-————-\n\t"
        )
    }
}
