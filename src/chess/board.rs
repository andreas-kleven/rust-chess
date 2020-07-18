use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

use crate::chess::{Move, Piece, Player, Position, Square};

#[derive(Debug)]
pub struct Board {
    pub player1: Player,
    pub player2: Player,
    pub turn: i32,
    pub cur_pos: Option<Position>,
    pub cur_moves: Vec<Move>,
    pub prev_move: Option<Move>,
    pub grid: [[Square; 8]; 8],
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            player1: self.player1.clone(),
            player2: self.player2.clone(),
            turn: self.turn.clone(),
            cur_pos: None,
            cur_moves: Vec::new(),
            prev_move: None,
            grid: self.grid.clone(),
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            player1: Player { num: 1 },
            player2: Player { num: 2 },
            turn: 1,
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

    pub fn test(&mut self) {
        self.grid = [[Square::from(Piece::None, 0); 8]; 8];
        self.grid[0][4] = Square::from(Piece::King, 1);
        self.grid[5][2] = Square::from(Piece::Knight, 2);
        self.grid[6][1] = Square::from(Piece::Pawn, 1);
        self.grid[0][0] = Square::from(Piece::Rook, 1);
        self.grid[0][7] = Square::from(Piece::Rook, 1);
    }

    pub fn white_turn(&self) -> bool {
        self.turn == 1
    }

    pub fn black_turn(&self) -> bool {
        self.turn == 2
    }

    pub fn get(&self, x: i32, y: i32) -> &Square {
        &self.grid[y as usize][x as usize]
    }

    pub fn getp(&self, pos: &Position) -> &Square {
        self.get(pos.x as i32, pos.y as i32)
    }

    pub fn set(&mut self, x: i32, y: i32, square: &Square) {
        self.grid[y as usize][x as usize] = *square;
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

    pub fn get_castling_move(&self, pos: &Position, side: i32) -> Option<Move> {
        let square = self.getp(pos);

        if square.moved {
            return None;
        }

        let xr = if side == 0 { 0 } else { 7 };
        let sign = if side == 0 { -1 } else { 1 };
        let rook_sq = self.get(xr, pos.y);

        if rook_sq.moved || rook_sq.piece != Piece::Rook {
            return None;
        }

        let range = if side == 0 { 1..pos.x } else { (pos.x + 1)..7 };

        for x in range {
            if !self.get(x, pos.y).is_none() {
                return None;
            }
        }

        let p1 = &Position::new(pos.x + 1 * sign, pos.y);
        let p2 = &Position::new(pos.x + 2 * sign, pos.y);

        if self.square_vulnerable(p1) || self.square_vulnerable(p2) {
            return None;
        }

        let from = pos.clone();
        let to = Position::new(pos.x + 2 * sign, pos.y);
        Move::new(from, to)
    }

    pub fn get_moves(&self, p: &Position, all: bool) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let square = self.getp(p);
        let row = self.get_row(square, p);

        let mut try_add = |add_pos: Position, attack: bool| -> bool {
            if self.can_move_to(square, &add_pos, attack) {
                let is_none = self.getp(&add_pos).is_none();
                let mv = Move::new(p.clone(), add_pos);

                if mv.is_some() {
                    moves.push(mv.unwrap());
                }

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

                if !all && !self.is_check() {
                    let mv1 = self.get_castling_move(p, 0);
                    let mv2 = self.get_castling_move(p, 1);

                    if mv1.is_some() {
                        moves.push(mv1.unwrap());
                    }

                    if mv2.is_some() {
                        moves.push(mv2.unwrap());
                    }
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

        if !all {
            let mut cloned = self.clone();
            let grid = cloned.grid;

            moves.retain(|mv| {
                cloned.grid = grid.clone();
                cloned.perform_move(&mv);
                !cloned.is_check()
            })
        }

        moves
    }

    pub fn square_vulnerable(&self, pos: &Position) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let from_sq = self.get(x, y);

                if from_sq.player == 0 || from_sq.player == self.turn {
                    continue;
                }

                let moves = self.get_moves(&Position::new(x, y), true);

                if moves.iter().any(|mv| &mv.to == pos) {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_check(&self) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let square = self.get(x, y);

                if square.piece == Piece::King && square.player == self.turn {
                    return self.square_vulnerable(&Position::new(x, y));
                }
            }
        }

        false
    }

    pub fn is_stalemate(&self) -> bool {
        !self.is_check() && !self.can_move_any()
    }

    pub fn is_checkmate(&self) -> bool {
        self.is_check() && !self.can_move_any()
    }

    pub fn can_move_any(&self) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let square = self.get(x, y);

                if square.player != self.turn {
                    continue;
                }

                let moves = self.get_moves(&Position::new(x, y), false);

                if moves.len() > 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn can_move(&self, mv: &Move) -> bool {
        let moves = self.get_moves(&mv.from, false);
        moves.iter().any(|m| m == mv)
    }

    pub fn select(&mut self, pos_str: Option<&&str>) -> bool {
        self.cur_pos = None;
        self.cur_moves.clear();

        if pos_str.is_some() {
            let s = &pos_str.unwrap();
            let pos_opt = Position::from(s.as_bytes());

            if pos_opt.is_none() {
                return false;
            } else {
                let pos = pos_opt.unwrap();
                let square = self.getp(&pos);

                if square.player != self.turn {
                    return false;
                }

                self.cur_pos = pos_opt;
                self.cur_moves = self.get_moves(&pos, false);
            }
        }

        true
    }

    pub fn do_move(&mut self, mv: &Move) -> bool {
        if !mv.from.is_valid() || !mv.to.is_valid() {
            return false;
        }

        if !self.can_move(mv) {
            return false;
        }

        let from_sq = self.getp(&mv.from);
        let to_sq = self.getp(&mv.to);

        if from_sq.player != self.turn || from_sq.player == to_sq.player {
            return false;
        }

        if !to_sq.is_none() {
            //
        }

        self.perform_move(mv);
        self.prev_move = Some(*mv);

        if self.get_promoting().is_none() {
            self.next_turn();
        }

        true
    }

    fn perform_move(&mut self, mv: &Move) {
        let mut from_sq = self.getp(&mv.from).clone();

        from_sq.moved = true;
        self.setp(&mv.to, &from_sq);
        self.setp(&mv.from, &Square::from(Piece::None, 0));

        // Castling
        if from_sq.piece == Piece::King && (mv.from.x - mv.to.x).abs() > 1 {
            let xr = if mv.to.x < mv.from.x { 0 } else { 7 };
            let sign = if mv.to.x < mv.from.x { -1 } else { 1 };
            let pr = &Position::new(xr, mv.to.y);

            let mut rook_sq = self.getp(pr).clone();
            rook_sq.moved = true;
            self.setp(&Position::new(mv.to.x - sign, mv.to.y), &rook_sq);
            self.setp(&pr, &Square::from(Piece::None, 0));
        }
    }

    pub fn get_promoting(&self) -> Option<Position> {
        let y = if self.turn == 1 { 7 } else { 0 };

        for x in 0..8 {
            let square = self.grid[y][x];

            if square.player == self.turn && square.piece == Piece::Pawn {
                return Some(Position::new(x as i32, y as i32));
            }
        }

        return None;
    }

    pub fn promote(&mut self, piece: Piece) -> bool {
        let pos = self.get_promoting();

        if pos.is_none() {
            return false;
        }

        if piece != Piece::Bishop
            && piece != Piece::Knight
            && piece != Piece::Queen
            && piece != Piece::Rook
        {
            return false;
        }

        self.setp(&pos.unwrap(), &Square::from(piece, self.turn));
        self.next_turn();
        true
    }

    pub fn next_turn(&mut self) {
        self.turn = if self.turn == 1 { 2 } else { 1 };
    }
}
