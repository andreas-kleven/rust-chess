use crate::chess::{Board, Piece, PieceType};
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
            let piece = &board.get(x, y);
            let mut s = piece_string(piece);

            if board.vis_pos.is_some() {
                let moves_pos = &board.vis_pos.unwrap();

                if x == moves_pos.x && y == moves_pos.y {
                    s = Style::new()
                        .on(CURRENT_COLOR)
                        .paint(piece_letter(&piece).to_string());
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

fn piece_string(piece: &Piece) -> ANSIString {
    piece_color(&piece).paint(piece_letter(&piece).to_string())
}

fn piece_letter(piece: &Piece) -> char {
    match piece.piece_type {
        PieceType::None => 'x',
        PieceType::Bishop => 'B',
        PieceType::King => 'K',
        PieceType::Knight => 'N',
        PieceType::Pawn => 'p',
        PieceType::Queen => 'Q',
        PieceType::Rook => 'R',
    }
}

fn piece_color(piece: &Piece) -> Style {
    if piece.is_white() {
        WHITE_COLOR.bold()
    } else if piece.is_black() {
        BLACK_COLOR.bold()
    } else {
        Style::new().fg(EMPTY_COLOR)
    }
}
