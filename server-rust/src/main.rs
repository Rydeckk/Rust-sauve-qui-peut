mod structure;

use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use commun::structs::{Command};
use structure::{team::TeamManager, player::Player, challenge::ChallengePosition, command::CommandFunction};
use tracing::{info, error};
use commun::serde_json;

fn send_to_client(mut stream: &TcpStream, message: String) -> std::io::Result<()> {
    let message_bytes = message.as_bytes();
    let n = message.len() as u32;
    let bytes = n.to_le_bytes();

    stream.write_all(&bytes)?;
    stream.write_all(message_bytes)?;

    info!("Sent response to client: {}", message);
    Ok(())
}


fn handle_client(mut stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>) {
    info!("New connection from {}", stream.peer_addr().unwrap());

    loop {
        let mut size_buffer = [0_u8; 4];
        if let Err(e) = stream.read_exact(&mut size_buffer) {
            error!("Error reading size: {} - Closing connection", e);
            break;
        }

        let n = u32::from_le_bytes(size_buffer);
        let mut buffer = vec![0; n as usize];

        if let Err(e) = stream.read_exact(&mut buffer) {
            error!("Error reading data: {} - Closing connection", e);
            break;
        }

        let received_data = String::from_utf8_lossy(&buffer);
        info!("Received full message ({} bytes): {}", n, received_data);

        match serde_json::from_str::<Command>(&received_data) {
            Ok(command) => {
                info!("Parsed command: {:?}", command);
                Command::process(command, stream.try_clone().unwrap(), team_manager.clone(), player.clone(), challenge.clone());
            }
            Err(e) => {
                error!("Error parsing JSON: {} - Closing connection", e);
                break;
            }
        }
    }
}

fn inner_main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8778")?;
    info!("Server listening on 127.0.0.1:8778");

    let team_manager = Arc::new(Mutex::new(TeamManager::new()));
    let player = Arc::new(Mutex::new(Player::new("Player1".to_string())));
    let challenge = Arc::new(Mutex::new(ChallengePosition::new(structure::maze::Point { x: 3, y: 5 })));

    let (pos_x, pos_y) = {
        let player_lock = player.lock().unwrap();
        let pos = player_lock.get_position();
        (pos.x, pos.y)
    };

    info!("Player initialized at position: x={}, y={}", pos_x, pos_y);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New client connected: {}", stream.peer_addr()?);
                let team_manager = Arc::clone(&team_manager);
                let player = Arc::clone(&player);
                let challenge = Arc::clone(&challenge);
                thread::spawn(move || {
                    handle_client(stream, team_manager, player, challenge);
                });
            }
            Err(e) => {
                error!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    if let Err(err) = inner_main() {
        error!("Server encountered an error: {}", err);
    }
}