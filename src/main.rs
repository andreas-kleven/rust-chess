use ansi_term::{ANSIString, Colour, Style};
mod chess;
mod net;
mod render;

use chess::{Board, Move, Piece, Position};
use net::{DummyInterface, Interface, TcpInterface};
use rand::Rng;
use std::env;
use std::io::{stdin, stdout, Write};

struct Context<'a> {
    board: Board,
    interface: Box<dyn Interface>,
    message: ANSIString<'a>,
    player: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut player = 0;

    let interface: Box<dyn Interface> = match args.get(1) {
        Some(s) => {
            if s.contains(':') {
                let mut client = TcpInterface::client(s);
                player = client.get_player().unwrap();
                Box::new(client)
            } else {
                let port = s.to_string().parse::<u16>().unwrap_or(0);

                if port == 0 {
                    println!("Invalid port");
                    return;
                }

                let mut server = TcpInterface::server(port);
                player = select_player();
                server.send_player(player);
                Box::new(server)
            }
        }
        None => {
            player = 1;
            Box::new(DummyInterface {})
        }
    };

    if player != 1 && player != 2 {
        println!("Invalid player '{}'", player);
        return;
    }

    let mut ctx = Context {
        player,
        board: Board::new(),
        interface: interface,
        message: ANSIString::from(""),
    };

    let color = player_color(ctx.player);

    if !ctx.interface.is_local() {
        ctx.message = ANSIString::from(format!("You are playing as {}", color));
    }

    ctx.board.randomize();
    ctx.board.test();

    main_loop(&mut ctx);
}

fn select_player() -> i32 {
    loop {
        print!(
            "Select color ({}hite/{}lack/{}andom): ",
            Colour::Green.paint("w"),
            Colour::Green.paint("b"),
            Colour::Green.paint("r")
        );

        stdout().flush().unwrap();

        let line = read_line();

        match line.trim() {
            "w" | "white" => break 1,
            "b" | "black" => break 2,
            "r" => break rand::thread_rng().gen_range(1, 3),
            _ => (),
        }
    }
}

fn main_loop(ctx: &mut Context) {
    loop {
        render::draw_board(&ctx.board, ctx.player == 2);

        println!();

        if ctx.message.len() > 0 {
            println!("{}", ctx.message.to_string());
            ctx.message = ANSIString::from("");
        }

        if ctx.board.is_checkmate() {
            println!("{}", Colour::Blue.paint("Checkmate!"));
            break;
        } else if ctx.board.is_stalemate() {
            println!("{}", Colour::Blue.paint("Stalemate!"));
            break;
        } else if ctx.board.is_check() {
            println!("{}", Colour::Blue.paint("Check!"));
        }

        let color = player_color(ctx.board.turn);

        if ctx.interface.is_local() || ctx.player == ctx.board.turn {
            print!("{} move: ", color);
            stdout().flush().unwrap();

            if !handle_input(ctx) {
                return;
            }

            handle_promote(ctx);
        } else {
            println!("Waiting for {}...", color);

            if !ctx.interface.wait(&mut ctx.board) {
                println!("Player disconnected");
                return;
            }
        }
    }
}

fn handle_input(ctx: &mut Context) -> bool {
    let line = read_line();
    let input = line.trim();

    let args: Vec<&str> = input.split_whitespace().collect();
    let command = *args.get(0).unwrap_or(&"");

    match command {
        "q" | "quit" | "exit" => {
            //
            ctx.interface.send_surrender();
            return false;
        }
        _ => handle_move(ctx, args),
    }

    true
}

fn handle_move(ctx: &mut Context, args: Vec<&str>) {
    if args.len() == 1 {
        let pos_str = args.get(0);

        if ctx.board.cur_pos.is_none() {
            if !ctx.board.select(pos_str) {
                ctx.message =
                    Colour::Red.paint(format!("Invalid position '{}'", &pos_str.unwrap()));
            }
        } else {
            let to_opt = Position::from(args[0].as_bytes());

            if to_opt.is_none() {
                ctx.message = Colour::Red.paint("Invalid move");
            } else {
                let from = ctx.board.cur_pos.unwrap().clone();
                let mv_opt = Move::new(from, to_opt.unwrap());
                do_move(ctx, mv_opt);
                ctx.board.select(None);
            }
        }
    } else if args.len() == 2 {
        let move_str = args.join(" ");
        let mv_opt = Move::from(move_str.as_str());
        do_move(ctx, mv_opt);
    } else {
        ctx.board.select(None);
    }
}

fn do_move(ctx: &mut Context, mv_opt: Option<Move>) {
    if mv_opt.is_none() {
        ctx.message = Colour::Red.paint("Invalid move");
        return;
    }

    let mv = mv_opt.unwrap();

    if !ctx.board.do_move(&mv) {
        ctx.message = Colour::Red.paint(format!("Cannot move '{}'", &mv));
        return;
    }

    ctx.message = Colour::Green.paint("Move ok");
    ctx.interface.send_move(&mv);
}

fn handle_promote(ctx: &mut Context) {
    if ctx.board.get_promoting().is_none() {
        return;
    }

    ctx.message = ANSIString::from("");
    render::draw_board(&ctx.board, ctx.player == 2);
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
            if ctx.board.promote(piece) {
                if let Some(true) = ctx.interface.send_promote(piece) {
                    break;
                } else {
                    // TODO: ctx.board.undo();
                    println!("Promote error");
                }
            } else {
                println!("{}", Colour::Red.paint("Could not promote"));
            }
        } else {
            println!("{}", Colour::Red.paint("Invalid piece type"));
        }
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

fn player_color(player: i32) -> String {
    if player == 1 {
        String::from("White")
    } else {
        String::from("Black")
    }
}
