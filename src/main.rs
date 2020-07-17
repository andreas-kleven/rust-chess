mod chess;
mod render;

use chess::{Board, Move};
use std::io::stdin;

fn main() {
    let mut board = Board::new();
    board.randomize();

    loop {
        render::draw_board(&board);
        board.visualize_moves(None);

        if !handle_input(&mut board) {
            break;
        }
    }
}

fn handle_input(board: &mut Board) -> bool {
    let line = read_line();
    let input = line.trim();

    if input.len() > 0 {
        let args: Vec<&str> = input.split_whitespace().collect();
        let command = args[0];

        match command {
            "q" | "quit" | "exit" => return false,
            _ => handle_move(board, args),
        }
    }

    true
}

fn handle_move(board: &mut Board, args: Vec<&str>) {
    if args.len() == 1 {
        let pos_str = args.get(0);
        if !board.visualize_moves(pos_str) {
            println!("Invalid position '{}'", &pos_str.unwrap());
        }
    } else if args.len() == 2 {
        let move_str = args.join(" ");
        let mv_opt = Move::from(move_str.as_str());

        if mv_opt.is_none() {
            println!("Invalid move '{}'", move_str);
            return;
        }
        let mv = mv_opt.unwrap();
        if !board.do_move(&mv) {
            println!("Cannot move '{}'", &mv);
            return;
        }
        render::draw_board(board);
        print!("Confirm y/N: ");
        if read_line() == "y" {
            //board.confirm();
        } else {
            //board.cancel();
        }
    } else {
        println!("Invalid command");
    }
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
