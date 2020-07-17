use ansi_term::{ANSIString, Colour, Style};
mod chess;
mod render;

use chess::{Board, Move, Piece, Position};
use std::io::{stdin, stdout, Write};

fn main() {
    let mut message = ANSIString::from("");
    let mut board = Board::new();

    board.randomize();
    board.test();

    loop {
        render::draw_board(&board);

        println!();

        if message.len() > 0 {
            println!("{}", message.to_string());
            message = ANSIString::from("");
        }

        if board.is_checkmate() {
            println!("{}", Colour::Blue.paint("Checkmate!"));
            break;
        } else if board.is_stalemate() {
            println!("{}", Colour::Blue.paint("Stalemate!"));
            break;
        } else if board.is_check() {
            println!("{}", Colour::Blue.paint("Check!"));
        }

        let color = if board.white_turn() { "White" } else { "Black" };
        print!("{} move: ", color);
        stdout().flush().unwrap();

        if !handle_input(&mut message, &mut board) {
            break;
        }

        if board.get_promoting().is_some() {
            message = ANSIString::from("");
            render::draw_board(&board);
            println!();

            loop {
                let color = Colour::Green;

                print!(
                    "Promote ({}ueen/{}ook/{}night/{}ishop): ",
                    color.paint("q"),
                    color.paint("r"),
                    color.paint("k"),
                    color.paint("b"),
                );

                stdout().flush().unwrap();

                let line = read_line();

                let piece = match line.trim() {
                    "b" | "bishop" => Piece::Bishop,
                    "k" | "knight" => Piece::Knight,
                    "q" | "queen" => Piece::Queen,
                    "r" | "rook" => Piece::Rook,
                    _ => Piece::None,
                };

                if piece != Piece::None {
                    if board.promote(piece) {
                        break;
                    } else {
                        println!("{}", Colour::Red.paint("Could not promote"));
                    }
                } else {
                    println!("{}", Colour::Red.paint("Invalid piece type"));
                }
            }
        }
    }
}

fn handle_input(message: &mut ANSIString, board: &mut Board) -> bool {
    let line = read_line();
    let input = line.trim();

    let args: Vec<&str> = input.split_whitespace().collect();
    let command = *args.get(0).unwrap_or(&"");

    match command {
        "q" | "quit" | "exit" => return false,
        _ => handle_move(message, board, args),
    }

    true
}

fn handle_move(message: &mut ANSIString, board: &mut Board, args: Vec<&str>) {
    if args.len() == 1 {
        let pos_str = args.get(0);

        if board.cur_pos.is_none() {
            if !board.select(pos_str) {
                *message = Colour::Red.paint(format!("Invalid position '{}'", &pos_str.unwrap()));
            }
        } else {
            let to_opt = Position::from(args[0].as_bytes());

            if to_opt.is_none() {
                *message = Colour::Red.paint("Invalid move");
            } else {
                let from = board.cur_pos.unwrap().clone();
                let mv_opt = Move::new(from, to_opt.unwrap());
                do_move(message, board, mv_opt);
                board.select(None);
            }
        }
    } else if args.len() == 2 {
        let move_str = args.join(" ");
        let mv_opt = Move::from(move_str.as_str());
        do_move(message, board, mv_opt);
    } else {
        board.select(None);
    }
}

fn do_move(message: &mut ANSIString, board: &mut Board, mv_opt: Option<Move>) {
    if mv_opt.is_none() {
        *message = Colour::Red.paint("Invalid move");
        return;
    }

    let mv = mv_opt.unwrap();

    if !board.do_move(&mv) {
        *message = Colour::Red.paint(format!("Cannot move '{}'", &mv));
        return;
    }

    *message = Colour::Green.paint("Move ok");
}

fn read_line() -> String {
    let mut input = String::new();
    input.clear();

    match stdin().read_line(&mut input) {
        Err(_) => println!("Error reading line"),
        _ => (),
    }

    input
}
