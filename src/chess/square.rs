use crate::chess::Piece;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub piece: Piece,
    pub player: i32,
    pub moved: bool,
}

impl Square {
    pub fn from(piece: Piece, player: i32) -> Square {
        Square {
            piece,
            player,
            moved: false,
        }
    }

    #[allow(dead_code)]
    pub fn is_none(&self) -> bool {
        self.piece == Piece::None
    }

    #[allow(dead_code)]
    pub fn is_white(&self) -> bool {
        self.player == 1
    }

    #[allow(dead_code)]
    pub fn is_black(&self) -> bool {
        self.player == 2
    }
}
