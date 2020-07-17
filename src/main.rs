mod chess;
mod render;

use chess::{Board, Move, Position};
use std::io::stdin;

fn main() {
    let mut board = Board::new();
    board.randomize();

    loop {
        render::draw_board(&board);

        if !handle_input(&mut board) {
            break;
        }
    }
}

fn handle_input(board: &mut Board) -> bool {
    let line = read_line();
    let input = line.trim();

    let args: Vec<&str> = input.split_whitespace().collect();
    let command = *args.get(0).unwrap_or(&"");

    match command {
        "q" | "quit" | "exit" => return false,
        _ => handle_move(board, args),
    }

    true
}

fn handle_move(board: &mut Board, args: Vec<&str>) {
    if args.len() == 1 {
        let pos_str = args.get(0);

        if board.cur_pos.is_none() {
            if !board.select(pos_str) {
                println!("Invalid position '{}'", &pos_str.unwrap());
            }
        } else {
            let to_opt = Position::from(args[0].as_bytes());

            if to_opt.is_none() {
                println!("Invalid move");
            } else {
                let from = board.cur_pos.unwrap().clone();
                let mv_opt = Move::new(from, to_opt.unwrap());
                do_move(board, mv_opt);
                board.select(None);
            }
        }
    } else if args.len() == 2 {
        let move_str = args.join(" ");
        let mv_opt = Move::from(move_str.as_str());
        do_move(board, mv_opt);
    } else {
        board.select(None);
    }
}

fn do_move(board: &mut Board, mv_opt: Option<Move>) {
    if mv_opt.is_none() {
        println!("Invalid move");
        return;
    }

    let mv = mv_opt.unwrap();

    if !board.do_move(&mv) {
        println!("Cannot move '{}'", &mv);
        return;
    }

    println!("Move ok");
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
