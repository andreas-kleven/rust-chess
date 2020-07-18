use crate::chess::{Board, Piece, Square};
use ansi_term::{ANSIString, Colour, Style};

const WHITE_COLOR: Colour = Colour::RGB(255, 255, 255);
const BLACK_COLOR: Colour = Colour::RGB(0, 0, 0);
const MOVE_COLOR: Colour = Colour::RGB(64, 128, 160);
const PREV_COLOR: Colour = Colour::RGB(96, 128, 96);
const BOARD_BACKGROUND_1: Colour = Colour::RGB(96, 96, 96);
const BOARD_BACKGROUND_2: Colour = Colour::RGB(160, 160, 160);
//const BOARD_BACKGROUND_1: Colour = Colour::RGB(119, 148, 85);
//const BOARD_BACKGROUND_2: Colour = Colour::RGB(235, 235, 208);

pub fn draw_board(board: &Board) {
    let info_style = Colour::White;
    let mut rows: Vec<String> = Vec::new();

    for y in (0..8).rev() {
        let mut columns: Vec<ANSIString> = Vec::with_capacity(8);

        for x in 0..8 {
            let square = &board.get(x, y);
            let mut s = square_string(square, x, y);

            if board.cur_pos.is_some() {
                let moves_pos = &board.cur_pos.unwrap();

                if x == moves_pos.x && y == moves_pos.y {
                    s = square_string_style(square, &square_color(square).on(MOVE_COLOR));
                }

                if board.cur_moves.iter().any(|mv| mv.to.x == x && mv.to.y == y) {
                    s = square_string_style(square, &square_color(square).on(MOVE_COLOR));
                }
            } else if board.prev_move.is_some() {
                let prev_move = &board.prev_move.unwrap();

                if (x == prev_move.from.x && y == prev_move.from.y)
                    || (x == prev_move.to.x && y == prev_move.to.y)
                {
                    s = square_string_style(square, &square_color(square).on(PREV_COLOR));
                }
            }

            columns.push(s);
        }

        let cols_str: Vec<_> = columns.iter().map(ToString::to_string).collect();
        let line = cols_str.join("");
        let line_num = info_style.paint(format!("{}", (y + 1).to_string()));

        rows.push(format!("{} {} {}", line_num, line, line_num));
    }

    println!("  {}", info_style.paint("a b c d e f g h"));
    println!("{}", rows.join("\n"));
    println!("  {}", info_style.paint("a b c d e f g h"));
}

fn square_string(square: &Square, x: i32, y: i32) -> ANSIString {
    let style = square_color(&square).on(square_backgroud(x, y));
    square_string_style(square, &style)
}

fn square_string_style<'a>(square: &Square, style: &Style) -> ANSIString<'a> {
    style.paint(format!("{} ", square_letter(square).to_string()))
}

fn square_letter(square: &Square) -> char {
    if square.is_white() && false {
        match square.piece {
            Piece::None => ' ',
            Piece::Bishop => '♗',
            Piece::King => '♔',
            Piece::Knight => '♘',
            Piece::Pawn => '♙',
            Piece::Queen => '♕',
            Piece::Rook => '♖',
        }
    } else {
        match square.piece {
            Piece::None => ' ',
            Piece::Bishop => '♝',
            Piece::King => '♚',
            Piece::Knight => '♞',
            Piece::Pawn => '♟',
            Piece::Queen => '♛',
            Piece::Rook => '♜',
        }
    }
}

fn square_color(square: &Square) -> Style {
    if square.is_white() {
        Style::new().fg(WHITE_COLOR)
    } else {
        Style::new().fg(BLACK_COLOR)
    }
}

fn square_backgroud(x: i32, y: i32) -> Colour {
    let odd = (y % 2) ^ (x % 2) == 0;

    if odd {
        BOARD_BACKGROUND_1
    } else {
        BOARD_BACKGROUND_2
    }
}
