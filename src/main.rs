#![allow(unused)]

mod bitboard;
mod fen_parser;
mod moves;
mod piece;
mod position;

fn main() {
    let position: position::Position =
        fen_parser::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    println!("{}", position)
}
