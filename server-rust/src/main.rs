mod structure;
mod shared;

use crate::shared::libs::*;
use structure::team::*;
use structure::command::*;

fn send_to_client(mut stream: TcpStream, message: String) {
    let message_convert = message.as_bytes();
    let n = message.len() as u32;

    let bytes = n.to_le_bytes();

    let _ = stream.write(&bytes);
    let _ = stream.write(message_convert);
}

fn handle_client(mut stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) {
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

                    Command::process(serde_json::from_str(&s).unwrap(), stream, team_manager);
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

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let team_manager = Arc::clone(&team_manager);
                thread::spawn(move || {
                    handle_client(stream, team_manager);
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