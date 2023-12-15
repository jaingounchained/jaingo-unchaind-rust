use std::{collections::HashMap, os::unix::fs::MetadataExt};

use crate::{
    bitboard::{Bitboard, PieceBitboard, PiecePlacement},
    moves::Square,
    piece::PieceType,
    position::{CastlingRights, CastlingTypes, Color, Position},
};

pub fn parse_fen(fen: &str) -> Position {
    let components: Vec<&str> = fen.split_whitespace().collect();

    return Position {
        piece_placement: parse_piece_placement(components[0]),
        active_color: parse_active_color(components[1]),
        castling_rights: parse_castling_rights(components[2]),
        en_passant_target: parse_en_passant_target(components[3]),
        half_move_clock: components[4].parse::<u16>().unwrap(),
        full_move_number: components[5].parse::<u16>().unwrap(),
    };
}

fn parse_piece_placement(piece_placement_str: &str) -> PiecePlacement {
    let ranks: Vec<&str> = piece_placement_str.split("/").collect();

    let mut white_pieces: HashMap<PieceType, Bitboard> = HashMap::new();
    let mut black_pieces: HashMap<PieceType, Bitboard> = HashMap::new();

    for i in 0..8 {
        let mut k = 0;
        for c in ranks[i].chars() {
            let index = 8 * (7 - i) + k;
            match c {
                'P' => {
                    white_pieces
                        .entry(PieceType::Pawn)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'N' => {
                    white_pieces
                        .entry(PieceType::Knight)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'B' => {
                    white_pieces
                        .entry(PieceType::Bishop)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'R' => {
                    white_pieces
                        .entry(PieceType::Rook)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'Q' => {
                    white_pieces
                        .entry(PieceType::Queen)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'K' => {
                    white_pieces
                        .entry(PieceType::King)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'p' => {
                    black_pieces
                        .entry(PieceType::Pawn)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'n' => {
                    black_pieces
                        .entry(PieceType::Knight)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'b' => {
                    black_pieces
                        .entry(PieceType::Bishop)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'r' => {
                    black_pieces
                        .entry(PieceType::Rook)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'q' => {
                    black_pieces
                        .entry(PieceType::Queen)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                'k' => {
                    black_pieces
                        .entry(PieceType::King)
                        .and_modify(|v| *v += 1 << index);
                    k += 1;
                }
                _ => {
                    let empty_positions = c.to_string().parse::<usize>().unwrap();
                    k += empty_positions;
                }
            };
        }
    }

    let mut piece_placement = HashMap::new();
    piece_placement.insert(Color::White, PieceBitboard(white_pieces));
    piece_placement.insert(Color::Black, PieceBitboard(black_pieces));

    return PiecePlacement(piece_placement);
}

fn parse_active_color(active_color_str: &str) -> Color {
    if active_color_str == "w" {
        return Color::White;
    }
    return Color::Black;
}

fn parse_castling_rights(castling_rights_str: &str) -> CastlingRights {
    let mut white_king_side_castling_type = false;
    let mut white_queen_side_castling_type = false;
    let mut black_king_side_castling_type = false;
    let mut black_queen_side_castling_type = false;

    for c in castling_rights_str.chars() {
        match c {
            'K' => white_king_side_castling_type = true,
            'Q' => white_queen_side_castling_type = true,
            'k' => black_king_side_castling_type = true,
            'q' => black_queen_side_castling_type = true,
            _ => {}
        }
    }

    let mut castling_rights = HashMap::new();
    castling_rights.insert(
        Color::White,
        CastlingTypes(
            white_king_side_castling_type,
            white_queen_side_castling_type,
        ),
    );
    castling_rights.insert(
        Color::Black,
        CastlingTypes(
            black_king_side_castling_type,
            black_queen_side_castling_type,
        ),
    );

    return CastlingRights(castling_rights);
}

fn parse_en_passant_target(en_passant_target: &str) -> u8 {
    if en_passant_target == "-" {
        return 64;
    }

    let file = en_passant_target.chars().nth(0).unwrap() as u8 - 97;
    let rank = en_passant_target.chars().nth(1).unwrap() as u8 - 49;
    return rank * 8 + file;
}
