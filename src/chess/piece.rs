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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Piece {
    pub fn from(name: &str) -> Piece {
        match name {
            "None" => Piece::None,
            "Bishop" => Piece::Bishop,
            "King" => Piece::King,
            "Knight" => Piece::Knight,
            "Pawn" => Piece::Pawn,
            "Queen" => Piece::Queen,
            "Rook" => Piece::Rook,
            _ => Piece::None,
        }
    }
}
