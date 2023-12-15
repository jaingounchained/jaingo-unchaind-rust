#![allow(unused)]

mod bitboard;
mod fen_parser;
mod moves;
mod piece;
mod position;

fn main() {
    let position: position::Position =
        fen_parser::parse_fen("4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1");

    println!("{}", position)
}
