#![allow(unused)]

mod bitboard;
mod fen_parser;
mod move_generator;
mod moves;
mod piece;
mod position;
mod utils;

fn main() {
    match fen_parser::parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e4 0 1") {
        Ok(position) => println!("{}", position),
        Err(err) => println!("{}", err),
    }

    println!("{}", bitboard::bitboard_representation(0x8040201008040200));
}
