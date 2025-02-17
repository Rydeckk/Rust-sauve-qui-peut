mod structure;

use commun::structs::Command;
use commun::*;
use structure::{team::*, player::Player, challenge::ChallengePosition, command::CommandFunction};

fn send_to_client(mut stream: TcpStream, message: String) {
    let message_convert = message.as_bytes();
    let n = message.len() as u32;

    let bytes = n.to_le_bytes();

    let _ = stream.write(&bytes);
    let _ = stream.write(message_convert);
}

fn handle_client(mut stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>) {
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

                    Command::process(serde_json::from_str(&s).unwrap(), stream, team_manager, player, challenge);
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

fn inner_main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    let team_manager = Arc::new(Mutex::new(TeamManager::new()));
    let player = Arc::new(Mutex::new(Player::new("PLayer1".to_string())));
    let challenge = Arc::new(Mutex::new(ChallengePosition::new(structure::maze::Point { x: 3, y: 5 })));

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let team_manager = Arc::clone(&team_manager);
                let player = Arc::clone(&player);
                let challenge = Arc::clone(&challenge);
                thread::spawn(move || {
                    handle_client(stream, team_manager, player, challenge);
                });
            }
            Err(e) => {
                eprintln!("Erreur de connexion : {}", e);
            }
        }
    }
    Ok(())
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