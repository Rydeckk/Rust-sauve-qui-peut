mod client_test;

// use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const MAX_PLAYER: i32 = 3;

enum PlayCommand {
    Play
}

enum TeamCommand {
    Create(String),
}

enum Command {
    Team,
    Play
}

struct Player {
    name: String
}

struct Team {
    name: String,
    nb_player: i32,
    access_key: String
}

impl TeamCommand {
    fn input_process_command(command: Vec<String>, stream: TcpStream) {
        match command.get(0).map(String::as_str) {
            Some("create") => TeamCommand::process(TeamCommand::Create(command[1].clone()), stream),
            _ => send_to_client(stream, "Commande introuvable".to_string()),
        }
    }

    fn create_team(name_team: String, stream: TcpStream) {
        let team = Team {name: name_team, nb_player: 0, access_key: "00000".to_string()};
        send_to_client(stream, format!("Team {} created.\nAccess Key : {}", team.name, team.access_key));
    }

    fn process(command: TeamCommand, stream: TcpStream) {
        match command {
            TeamCommand::Create(name_team) => TeamCommand::create_team(name_team, stream),
        };
    }
}

impl PlayCommand {
    fn input_process_command(command: Vec<String>, stream: TcpStream) {
        match command.get(0).map(String::as_str) {
            _ => send_to_client(stream, "Commande introuvable".to_string()),
        }
    }

    fn process(play_command: PlayCommand) {
        match play_command {
            PlayCommand::Play => todo!()
        }
    }
}

impl Command {
    fn input_process(command: String, stream: TcpStream) {
        let command_vec: Vec<String> = command
        .split_whitespace()
        .map(String::from)
        .collect();

        let args = command_vec[1..].to_vec();

        match command_vec.get(0).map(String::as_str) {
            Some("team") => Command::process(Command::Team, args, stream),
            Some("play") => Command::process(Command::Play, args, stream),
            _ => send_to_client(stream, "Commande introuvable".to_string()),
        }
    }

    fn process(command: Command, args: Vec<String>, stream: TcpStream) { 
        match command {
            Command::Team => TeamCommand::input_process_command(args, stream),
            Command::Play => PlayCommand::input_process_command(args, stream)
        }
    }
}

fn send_to_client(mut stream: TcpStream, message: String) {
    let message_convert = message.as_bytes();
    let n = message.len() as u32;

    let bytes = n.to_le_bytes();

    stream.write(&bytes);
    stream.write(message_convert);
}

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());

    let mut size_buffer = [0_u8; 4];
    match stream.read_exact(&mut size_buffer) {
        Ok(_) => {
            let n = u32::from_le_bytes(size_buffer);

            // Lecture des données
            let mut buffer = vec![0; n as usize];
            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    let s = String::from_utf8(buffer.to_vec()).unwrap();
                    println!("Request: {}", s);

                    Command::input_process(s, stream);
                }
                Err(e) => {
                    eprintln!("Erreur de lecture des données : {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Erreur de lecture de la taille : {}", e);
        }
    }

}

fn main() {

    match inner_main() {
        Ok(()) => {
            println!("Success")
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

fn inner_main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Erreur de connexion : {}", e);
            }
        }
    }
    Ok(())
}