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
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Move {
    from: Position,
    to: Position,
}

impl Move {
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
    pub fn is_white(self) -> bool {
        self.num == 1
    }
    pub fn is_black(self) -> bool {
        self.num == 2
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
    pub player: Player,
}

impl Square {
    pub fn from(piece: Piece, player: Player) -> Square {
        Square { piece, player }
    }

    pub fn is_none(&self) -> bool {
        self.piece == Piece::None
    }

    pub fn is_white(&self) -> bool {
        self.player.is_white()
    }

    pub fn is_black(&self) -> bool {
        self.player.is_black()
    }
}

#[derive(Debug)]
pub struct Board {
    pub player1: Player,
    pub player2: Player,
    pub mv_pos: Option<Position>,
    pub vis_pos: Option<Position>,
    pub vis_moves: Vec<Position>,
    pub grid: [[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let player0 = Player { num: 0 };
        let player1 = Player { num: 1 };
        let player2 = Player { num: 2 };

        Board {
            player1,
            player2,
            mv_pos: None,
            vis_pos: None,
            vis_moves: Vec::new(),
            grid: [
                [
                    Square::from(Piece::Rook, player1),
                    Square::from(Piece::Knight, player1),
                    Square::from(Piece::Bishop, player1),
                    Square::from(Piece::King, player1),
                    Square::from(Piece::Queen, player1),
                    Square::from(Piece::Bishop, player1),
                    Square::from(Piece::Knight, player1),
                    Square::from(Piece::Rook, player1),
                ],
                [Square::from(Piece::Pawn, player1); 8],
                [Square::from(Piece::None, player0); 8],
                [Square::from(Piece::None, player0); 8],
                [Square::from(Piece::None, player0); 8],
                [Square::from(Piece::None, player0); 8],
                [Square::from(Piece::Pawn, player2); 8],
                [
                    Square::from(Piece::Rook, player2),
                    Square::from(Piece::Knight, player2),
                    Square::from(Piece::Bishop, player2),
                    Square::from(Piece::King, player2),
                    Square::from(Piece::Queen, player2),
                    Square::from(Piece::Bishop, player2),
                    Square::from(Piece::Knight, player2),
                    Square::from(Piece::Rook, player2),
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

    pub fn visualize_moves(&mut self, pos_str: Option<&&str>) -> bool {
        self.vis_pos = None;
        self.vis_moves.clear();

        if pos_str.is_some() {
            let s = &pos_str.unwrap();
            self.vis_pos = Position::from(s.as_bytes());

            if self.vis_pos.is_none() {
                return false;
            } else {
                self.vis_moves = self.get_moves(&self.vis_pos.unwrap());
            }
        }

        true
    }

    pub fn do_move(&self, mv: &Move) -> bool {
        if !self.can_move(mv) {
            return false;
        }

        true
    }
}
