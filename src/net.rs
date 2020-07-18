use crate::chess::{Board, Move, Piece};
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

pub trait Interface {
    fn is_local(&self) -> bool;
    fn send_command(&mut self, prefix: &str, data: String) -> Option<bool>;
    fn get_player(&mut self) -> Option<i32>;
    fn wait(&mut self, board: &mut Board) -> bool;

    fn send_player(&mut self, player: i32) {
        self.send_command("player", player.to_string());
    }

    fn send_surrender(&mut self) -> Option<bool> {
        self.send_command("surrender", String::from(""))
    }

    fn send_move(&mut self, mv: &Move) -> Option<bool> {
        self.send_command("move", mv.to_string())
    }

    fn send_promote(&mut self, piece: Piece) -> Option<bool> {
        let letter = piece.to_string();
        self.send_command("promote", letter.to_string())
    }
}

pub struct DummyInterface {}

impl Interface for DummyInterface {
    fn is_local(&self) -> bool {
        true
    }

    fn send_command(&mut self, _: &str, _: String) -> Option<bool> {
        Some(true)
    }

    fn get_player(&mut self) -> Option<i32> {
        None
    }

    fn wait(&mut self, _: &mut Board) -> bool {
        true
    }
}

pub struct TcpInterface {
    stream: TcpStream,
}

impl Interface for TcpInterface {
    fn is_local(&self) -> bool {
        false
    }

    fn send_command(&mut self, prefix: &str, data: String) -> Option<bool> {
        let cmd = format!("{} {}\n", prefix, data);
        let bytes = cmd.as_bytes();

        match self.stream.write(bytes) {
            Err(_) => None,
            Ok(_) => {
                if prefix == "ok" || prefix == "err" {
                    Some(true)
                } else {
                    match self.read_line() {
                        None => None,
                        Some(line) => Some(line.trim() == "ok"),
                    }
                }
            }
        }
    }

    fn get_player(&mut self) -> Option<i32> {
        let player = match self.read_line() {
            None => None,
            Some(line) => {
                let args: Vec<&str> = line.split_whitespace().collect();

                if args.get(0) == Some(&"player") {
                    match args.get(1)?.parse::<i32>() {
                        Ok(player) => {
                            if player == 1 {
                                Some(2)
                            } else if player == 2 {
                                Some(1)
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }
        };

        self.send_response(player.is_some());
        player
    }

    fn wait(&mut self, board: &mut Board) -> bool {
        //let start_turn = board.turn;

        loop {
            let line_opt = self.read_line();

            if line_opt.is_none() {
                break false;
            }

            let line = line_opt.unwrap();
            let command = line.trim();

            println!("{}", command);
            let result = self.handle_command(command, board);
            println!("{}", result);
            self.send_response(result);

            break true;

            /*if board.turn != start_turn {
                break true;
            }*/
        }
    }
}

impl TcpInterface {
    pub fn client(host: &str) -> TcpInterface {
        TcpInterface {
            stream: TcpStream::connect(host).unwrap(),
        }
    }

    pub fn server(port: u16) -> TcpInterface {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
        let listener = TcpListener::bind(addr).unwrap();
        let (socket, addr) = listener.accept().unwrap();
        println!("new client: {:?}", addr);

        TcpInterface { stream: socket }
    }

    fn send_response(&mut self, result: bool) {
        if result {
            self.send_command("ok", String::from(""));
        } else {
            self.send_command("err", String::from(""));
        }
    }

    fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        let mut reader = BufReader::new(self.stream.try_clone().unwrap());

        match reader.read_line(&mut line) {
            Ok(_) => Some(line),
            Err(_) => None,
        }
    }

    fn handle_command(&self, command: &str, board: &mut Board) -> bool {
        let args: Vec<&str> = command.split_whitespace().collect();

        let data = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            String::from("")
        };

        if let Some(prefix) = args.get(0) {
            match prefix {
                &"move" => {
                    let mv_opt = Move::from(data.as_str());

                    if mv_opt.is_some() {
                        return board.do_move(&mv_opt.unwrap());
                    }
                }
                &"promote" => {
                    let piece = Piece::from(data.as_str());
                    return board.promote(piece);
                }
                _ => (),
            }
        }

        false
    }
}
