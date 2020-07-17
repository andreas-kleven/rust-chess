use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn from(text: &[u8]) -> Option<Position> {
        if text.len() != 2 {
            return None;
        }

        let x = (text[0] as i32) - ('a' as i32);
        let y = (text[1] as i32) - ('1' as i32);

        if x < 0 || x >= 8 || y < 0 || y >= 8 {
            return None;
        }

        Some(Position { x, y })
    }

    pub fn side(&self, idx: i32, dist: i32) -> Position {
        if dist <= 0 {
            panic!("Invalid distance '{}'", dist);
        }

        match idx {
            0 => Position::new(self.x, self.y + dist),
            1 => Position::new(self.x + dist, self.y),
            2 => Position::new(self.x, self.y - dist),
            3 => Position::new(self.x - dist, self.y),
            _ => panic!("Invalid index '{}'", idx),
        }
    }

    pub fn corner(&self, idx: i32, dist: i32) -> Position {
        if dist <= 0 {
            panic!("Invalid distance '{}'", dist);
        }

        match idx {
            0 => Position::new(self.x + dist, self.y + dist),
            1 => Position::new(self.x + dist, self.y - dist),
            2 => Position::new(self.x - dist, self.y + dist),
            3 => Position::new(self.x - dist, self.y - dist),
            _ => panic!("Invalid index '{}'", idx),
        }
    }

    pub fn is_valid(&self) -> bool {
        !(self.x < 0 || self.x >= 8 || self.y < 0 || self.y >= 8)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    from: Position,
    to: Position,
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

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub num: i32,
}

impl Player {
    pub fn is_white(&self) -> bool {
        self.num == 1
    }
    pub fn is_black(&self) -> bool {
        self.num == 2
    }
    pub fn is_none(&self) -> bool {
        !self.is_white() && !self.is_black()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num;
    }
}

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

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub piece: Piece,
    pub player: i32,
}

impl Square {
    pub fn from(piece: Piece, player: i32) -> Square {
        Square { piece, player }
    }

    pub fn is_none(&self) -> bool {
        self.piece == Piece::None
    }

    pub fn is_white(&self) -> bool {
        self.player == 1
    }

    pub fn is_black(&self) -> bool {
        self.player == 2
    }
}

#[derive(Debug)]
pub struct Board {
    pub player_none: Player,
    pub player1: Player,
    pub player2: Player,
    pub cur_pos: Option<Position>,
    pub cur_moves: Vec<Position>,
    pub prev_move: Option<Move>,
    pub grid: [[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        Board {
            player_none: Player { num: 0 },
            player1: Player { num: 1 },
            player2: Player { num: 2 },
            cur_pos: None,
            cur_moves: Vec::new(),
            prev_move: None,
            grid: [
                [
                    Square::from(Piece::Rook, 1),
                    Square::from(Piece::Knight, 1),
                    Square::from(Piece::Bishop, 1),
                    Square::from(Piece::King, 1),
                    Square::from(Piece::Queen, 1),
                    Square::from(Piece::Bishop, 1),
                    Square::from(Piece::Knight, 1),
                    Square::from(Piece::Rook, 1),
                ],
                [Square::from(Piece::Pawn, 1); 8],
                [Square::from(Piece::None, 0); 8],
                [Square::from(Piece::None, 0); 8],
                [Square::from(Piece::None, 0); 8],
                [Square::from(Piece::None, 0); 8],
                [Square::from(Piece::Pawn, 2); 8],
                [
                    Square::from(Piece::Rook, 2),
                    Square::from(Piece::Knight, 2),
                    Square::from(Piece::Bishop, 2),
                    Square::from(Piece::King, 2),
                    Square::from(Piece::Queen, 2),
                    Square::from(Piece::Bishop, 2),
                    Square::from(Piece::Knight, 2),
                    Square::from(Piece::Rook, 2),
                ],
            ],
        }
    }

    pub fn randomize(&mut self) {
        let mut squares = self.grid.concat();
        let mut rng = thread_rng();
        squares.shuffle(&mut rng);

        for y in 0..8 {
            for x in 0..8 {
                self.grid[y][x] = squares[y * 8 + x];
            }
        }
    }

    pub fn get(&self, x: i32, y: i32) -> &Square {
        &self.grid[y as usize][x as usize]
    }

    pub fn getp(&self, pos: &Position) -> &Square {
        self.get(pos.x as i32, pos.y as i32)
    }

    pub fn set(&mut self, x: i32, y: i32, square: &Square) {
        self.grid[y as usize][x as usize].piece = square.piece;
        self.grid[y as usize][x as usize].player = square.player;
    }

    pub fn setp(&mut self, pos: &Position, square: &Square) {
        self.set(pos.x as i32, pos.y as i32, square);
    }

    pub fn can_move_to(&self, square: &Square, pos: &Position, attack: bool) -> bool {
        if pos.x < 0 || pos.x >= 8 || pos.y < 0 || pos.y >= 8 {
            false
        } else {
            let other_square = self.getp(pos);
            other_square.is_none() || (attack && other_square.player != square.player)
        }
    }

    pub fn get_row(&self, square: &Square, pos: &Position) -> i32 {
        if square.is_white() {
            pos.y
        } else {
            7 - pos.y
        }
    }

    pub fn get_moves(&self, p: &Position) -> Vec<Position> {
        let mut moves = vec![];
        let square = self.getp(p);
        let row = self.get_row(square, p);

        let mut try_add = |add_pos: Position, attack: bool| -> bool {
            if self.can_move_to(square, &add_pos, attack) {
                let is_none = self.getp(&add_pos).is_none();
                moves.push(add_pos);
                is_none
            } else {
                false
            }
        };

        match square.piece {
            Piece::Pawn => {
                let sign = if square.is_white() { 1 } else { -1 };

                if try_add(Position::new(p.x, p.y + sign), false) {
                    if row == 1 {
                        try_add(Position::new(p.x, p.y + 2 * sign), false);
                    }
                }

                let a1 = Position::new(p.x + 1, p.y + sign);
                let a2 = Position::new(p.x - 1, p.y + sign);

                for pa in vec![a1, a2] {
                    if self.can_move_to(square, &pa, true) {
                        let other1 = self.getp(&pa);

                        if !other1.is_none() && other1.player != square.player {
                            try_add(pa, true);
                        }
                    }
                }
            }
            Piece::Bishop => {
                for idx in 0..4 {
                    for dist in 1..8 {
                        if !try_add(p.corner(idx, dist), true) {
                            break;
                        }
                    }
                }
            }
            Piece::Rook => {
                for idx in 0..4 {
                    for dist in 1..8 {
                        if !try_add(p.side(idx, dist), true) {
                            break;
                        }
                    }
                }
            }
            Piece::Queen => {
                for idx in 0..4 {
                    for dist in 1..8 {
                        if !try_add(p.side(idx, dist), true) {
                            break;
                        }
                    }
                }
                for idx in 0..4 {
                    for dist in 1..8 {
                        if !try_add(p.corner(idx, dist), true) {
                            break;
                        }
                    }
                }
            }
            Piece::King => {
                for idx in 0..4 {
                    try_add(p.side(idx, 1), true);
                    try_add(p.corner(idx, 1), true);
                }
            }
            Piece::Knight => {
                for idx in 0..4 {
                    try_add(p.side(idx, 2).side((idx + 1) % 4, 1), true);
                    try_add(p.side(idx, 2).side((idx + 3) % 4, 1), true);
                }
            }
            _ => (),
        }

        moves
    }

    pub fn can_move(&self, mv: &Move) -> bool {
        let moves = self.get_moves(&mv.from);
        moves.iter().any(|p| p.x == mv.to.x && p.y == mv.to.y)
    }

    pub fn select(&mut self, pos_str: Option<&&str>) -> bool {
        self.cur_pos = None;
        self.cur_moves.clear();

        if pos_str.is_some() {
            let s = &pos_str.unwrap();
            self.cur_pos = Position::from(s.as_bytes());

            if self.cur_pos.is_none() {
                return false;
            } else {
                self.cur_moves = self.get_moves(&self.cur_pos.unwrap());
            }
        }

        true
    }

    pub fn do_move(&mut self, mv: &Move) -> bool {
        if !self.can_move(mv) {
            return false;
        }

        if !mv.from.is_valid() || !mv.to.is_valid() {
            return false;
        }

        let from_sq = self.getp(&mv.from).clone();
        let to_sq = self.getp(&mv.to).clone();

        if from_sq.is_none() || from_sq.player == to_sq.player {
            return false;
        }

        if !to_sq.is_none() {
            //
        }

        self.setp(&mv.to, &from_sq);
        self.setp(&mv.from, &Square::from(Piece::None, 0));
        self.prev_move = Some(*mv);

        true
    }
}
