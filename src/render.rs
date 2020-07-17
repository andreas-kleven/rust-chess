use crate::chess::{Board, Piece, Square};
use ansi_term::{ANSIString, Colour, Style};

const EMPTY_COLOR: Colour = Colour::RGB(100, 100, 100);
const WHITE_COLOR: Colour = Colour::White;
const BLACK_COLOR: Colour = Colour::Red;
const CURRENT_COLOR: Colour = Colour::Blue;
const CURRENT_BACKGROUND: Colour = Colour::RGB(0, 255, 0);

pub fn draw_board(board: &Board) {
    let info_style = Colour::Black.on(Colour::Yellow);

    for y in (0..8).rev() {
        let mut row: Vec<ANSIString> = Vec::with_capacity(8);

        for x in 0..8 {
            let square = &board.get(x, y);
            let mut s = square_string(square);

            if board.vis_pos.is_some() {
                let moves_pos = &board.vis_pos.unwrap();

                if x == moves_pos.x && y == moves_pos.y {
                    s = Style::new()
                        .on(CURRENT_COLOR)
                        .paint(square_letter(&square).to_string());
                }

                if board.vis_moves.iter().any(|mv| mv.x == x && mv.y == y) {
                    s = Style::new().on(CURRENT_BACKGROUND).paint(s.to_string());
                }
            }

            row.push(s);
        }

        let cols_str: Vec<_> = row.iter().map(ToString::to_string).collect();
        let line = cols_str.join(" ");
        let line_num = info_style.paint(format!("{}", (y + 1).to_string()));
        println!("{}   {}   {}", line_num, line, line_num);
    }

    println!("\n    {}\n", info_style.paint("a b c d e f g h"));
}

fn square_string(square: &Square) -> ANSIString {
    square_color(&square).paint(square_letter(&square).to_string())
}

fn square_letter(square: &Square) -> char {
    match square.piece {
        Piece::None => 'x',
        Piece::Bishop => 'B',
        Piece::King => 'K',
        Piece::Knight => 'N',
        Piece::Pawn => 'p',
        Piece::Queen => 'Q',
        Piece::Rook => 'R',
    }
}

fn square_color(square: &Square) -> Style {
    if square.is_white() {
        WHITE_COLOR.bold()
    } else if square.is_black() {
        BLACK_COLOR.bold()
    } else {
        Style::new().fg(EMPTY_COLOR)
    }
}
