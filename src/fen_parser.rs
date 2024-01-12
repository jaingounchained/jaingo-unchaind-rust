use std::collections::HashMap;
use std::io::Error;
use std::{char, io::Empty};

use crate::{
    bitboard::{Bitboard, PieceBitboard, PiecePlacement},
    moves::Square,
    piece::PieceType,
    position::{CastlingRights, CastlingTypes, Color, Position},
};

pub fn parse_fen(fen: &str) -> Result<Position, String> {
    let components: Vec<&str> = fen.split_whitespace().collect();

    if components.len() != 6 {
        return Err(String::from("Number of components in fen!=6"));
    }

    return Ok(Position {
        piece_placement: parse_piece_placement(components[0])?,
        active_color: parse_active_color(components[1])?,
        castling_rights: parse_castling_rights(components[2])?,
        en_passant_target: parse_en_passant_target(components[3])?,
        half_move_clock: parse_u16_int(components[4])?,
        full_move_number: parse_u16_int(components[5])?,
    });
}

fn parse_piece_placement(piece_placement_str: &str) -> Result<PiecePlacement, String> {
    let ranks: Vec<&str> = piece_placement_str.split("/").collect();

    if ranks.len() != 8 {
        return Err(String::from("number of ranks in fen != 8"));
    }

    let mut white_pieces: HashMap<PieceType, Bitboard> = HashMap::new();
    let mut black_pieces: HashMap<PieceType, Bitboard> = HashMap::new();

    for piece in PieceType::iterator() {
        white_pieces.insert(*piece, 0);
        black_pieces.insert(*piece, 0);
    }

    for i in 0..8 {
        let mut k = 0;
        for c in ranks[i].chars() {
            let index = 8 * (7 - i) + k;
            match c {
                'P' => {
                    *white_pieces.entry(PieceType::Pawn).or_default() += 1 << index;
                    k += 1;
                }
                'N' => {
                    *white_pieces.entry(PieceType::Knight).or_default() += 1 << index;
                    k += 1;
                }
                'B' => {
                    *white_pieces.entry(PieceType::Bishop).or_default() += 1 << index;
                    k += 1;
                }
                'R' => {
                    *white_pieces.entry(PieceType::Rook).or_default() += 1 << index;
                    k += 1;
                }
                'Q' => {
                    *white_pieces.entry(PieceType::Queen).or_default() += 1 << index;
                    k += 1;
                }
                'K' => {
                    *white_pieces.entry(PieceType::King).or_default() += 1 << index;
                    k += 1;
                }
                'p' => {
                    *black_pieces.entry(PieceType::Pawn).or_default() += 1 << index;
                    k += 1;
                }
                'n' => {
                    *black_pieces.entry(PieceType::Knight).or_default() += 1 << index;
                    k += 1;
                }
                'b' => {
                    *black_pieces.entry(PieceType::Bishop).or_default() += 1 << index;
                    k += 1;
                }
                'r' => {
                    *black_pieces.entry(PieceType::Rook).or_default() += 1 << index;
                    k += 1;
                }
                'q' => {
                    *black_pieces.entry(PieceType::Queen).or_default() += 1 << index;
                    k += 1;
                }
                'k' => {
                    *black_pieces.entry(PieceType::King).or_default() += 1 << index;
                    k += 1;
                }
                _ => match c.to_string().parse::<usize>() {
                    Ok(empty_positions) if (empty_positions >= 1 && empty_positions <= 8) => {
                        k += empty_positions
                    }
                    _ => return Err(format!("char {}: invalid empty space", c)),
                },
            };
        }
    }

    let mut piece_placement = HashMap::new();
    piece_placement.insert(Color::White, PieceBitboard(white_pieces));
    piece_placement.insert(Color::Black, PieceBitboard(black_pieces));

    return Ok(PiecePlacement(piece_placement));
}

fn parse_active_color(active_color_str: &str) -> Result<Color, String> {
    match active_color_str {
        "w" => return Ok(Color::White),
        "b" => return Ok(Color::Black),
        _ => return Err(String::from("Invalid active color")),
    }
}

fn parse_castling_rights(castling_rights_str: &str) -> Result<Option<CastlingRights>, String> {
    if castling_rights_str == "-" {
        return Ok(None);
    }

    if castling_rights_str.len() > 4 {
        return Err(String::from(
            "castling rights string contains more than 4 characters",
        ));
    }

    let (mut white_king_side_castling_type, mut white_queen_side_castling_type) = (false, false);
    let (mut black_king_side_castling_type, mut black_queen_side_castling_type) = (false, false);

    for c in castling_rights_str.chars() {
        match c {
            'K' => white_king_side_castling_type = true,
            'Q' => white_queen_side_castling_type = true,
            'k' => black_king_side_castling_type = true,
            'q' => black_queen_side_castling_type = true,
            _ => {
                return Err(String::from(
                    "castling rights string contain characters other than K, Q, k, q",
                ))
            }
        }
    }

    let mut castling_rights = HashMap::new();
    if white_king_side_castling_type || white_queen_side_castling_type {
        castling_rights.insert(
            Color::White,
            CastlingTypes(
                white_king_side_castling_type,
                white_queen_side_castling_type,
            ),
        );
    }
    if black_king_side_castling_type || black_queen_side_castling_type {
        castling_rights.insert(
            Color::Black,
            CastlingTypes(
                black_king_side_castling_type,
                black_queen_side_castling_type,
            ),
        );
    }

    if castling_rights.len() == 0 {
        return Ok(None);
    }

    return Ok(Some(CastlingRights(castling_rights)));
}

fn parse_en_passant_target(en_passant_target: &str) -> Result<Option<Square>, String> {
    if en_passant_target == "-" {
        return Ok(None);
    }

    let en_passant_target_chars: Vec<char> = en_passant_target.chars().collect();
    if en_passant_target_chars.len() != 2 {
        return Err(String::from("en passant target in wrong format"));
    }

    let (file, rank) = (
        en_passant_target_chars[0] as u8 - b'a',
        en_passant_target_chars[1] as u8 - b'1',
    );

    if file > 7 || rank > 7 {
        return Err(String::from("en passant target in wrong format"));
    }

    return Ok(Some(Square(rank * 8 + file)));
}

fn parse_u16_int(u16_string: &str) -> Result<u16, String> {
    match u16_string.parse::<u16>() {
        Ok(value) => Ok(value),
        Err(_) => Err(String::from("Invalid string, couldn't convert to number")),
    }
}
