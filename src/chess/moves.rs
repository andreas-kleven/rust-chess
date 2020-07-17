use crate::chess::Position;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl Move {
    pub fn new(from: Position, to: Position) -> Option<Move> {
        if from.x < 0 || from.x >= 8 || from.y < 0 || from.y >= 8 {
            None
        } else {
            Some(Move { from, to })
        }
    }

    pub fn from(move_str: &str) -> Option<Move> {
        let bytes = move_str.as_bytes();

        if bytes.len() != 5 {
            None
        } else {
            let from = Position::from(&bytes[0..2]);
            let to = Position::from(&bytes[3..5]);

            if from.is_none() || to.is_none() {
                None
            } else {
                Some(Move {
                    from: from.unwrap(),
                    to: to.unwrap(),
                })
            }
        }
    }
}
