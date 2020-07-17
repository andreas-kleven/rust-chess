use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    None,
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}
