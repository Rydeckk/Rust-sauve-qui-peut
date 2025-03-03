extern crate core;

use commun::decodage::{decode_b64, decode_message, decode_radar_view_binary};
use commun::encodage::encode_message;
use commun::structs::{
    Action, ActionError, Challenge, Hint, JsonWrapper, RegisterTeam, RegisterTeamResult,
    SubscribePlayer, SubscribePlayerResult,
};
use maze_engine::challenge::ChallengeManager;
use maze_engine::navigation::Navigator;
use std::io::{Result, Write};
use std::net::TcpStream;

use maze_engine::scoring::ScoreManager;

struct Client {
    stream: TcpStream,
    challenge_manager: ChallengeManager,
    navigator: Navigator,
    last_challenge: Option<Challenge>,
    score_manager: ScoreManager,
    player_id: u32,
    compass_hint: Option<f32>,
}

impl Client {
    fn new(server: &str, team_size: u32) -> Result<Self> {
        let stream = TcpStream::connect(server)?;
        println!("Connected to server at {}", server);
        Ok(Client {
            stream,
            challenge_manager: ChallengeManager {
                secrets: Default::default(),
                sos_active: None,
            },
            navigator: Navigator::new(),
            last_challenge: None,
            score_manager: ScoreManager::new(team_size),
            player_id: 0,
            compass_hint: None,
        })
    }

    fn send_message(&mut self, message: &JsonWrapper) -> Result<()> {
        let encoded_message = encode_message(message)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
        self.stream.write_all(&encoded_message)?;
        println!("Sent message: {:?}", message);
        Ok(())
    }

    fn receive_message(&mut self) -> Result<JsonWrapper> {
        let message = decode_message(&mut self.stream)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
        println!("Received message: {:?}", message);
        Ok(message)
    }

    fn register_team(&mut self, team_name: &str) -> Result<String> {
        let registration = JsonWrapper::RegisterTeam(RegisterTeam {
            name: team_name.to_string(),
        });

        self.send_message(&registration)?;

        match self.receive_message()? {
            JsonWrapper::RegisterTeamResult(RegisterTeamResult::Ok {
                                                registration_token,
                                                expected_players,
                                            }) => {
                println!(
                    "Team registered successfully. Expected players: {}",
                    expected_players
                );
                Ok(registration_token)
            }
            JsonWrapper::RegisterTeamResult(RegisterTeamResult::Err(err)) => {
                println!("Registration error: {:?}", err);
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Registration error"))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unexpected response",
            )),
        }
    }

    fn subscribe_player(&mut self, player_name: &str, token: &str) -> Result<()> {
        let subscription = JsonWrapper::SubscribePlayer(SubscribePlayer {
            name: player_name.to_string(),
            registration_token: token.to_string(),
        });

        self.send_message(&subscription)?;

        match self.receive_message()? {
            JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Ok) => {
                println!("Player subscribed successfully");
                Ok(())
            }
            JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Err(err)) => {
                println!("Subscription error: {:?}", err);
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Subscription error"))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unexpected response",
            )),
        }
    }

    fn game_loop(&mut self) -> Result<()> {
        loop {
            // Rajouter un thread:sleep pour ralentir l'exécution en cas de besoin
            // let sleeping_time = time::Duration::from_millis(100);
            // thread::sleep(sleeping_time);

            // Récupération du message
            let message = match self.receive_message() {
                Ok(msg) => msg,
                Err(e) => {
                    // Si la connexion est fermée (labyrinthe fini), afficher le score final
                    if e.kind() == std::io::ErrorKind::ConnectionAborted || e.kind() == std::io::ErrorKind::UnexpectedEof {
                        let final_score = self.score_manager.compute_score();
                        println!("Labyrinthe fini! Score final: {}", final_score);
                        continue;
                    } else {
                        println!("Error receiving message: {:?}", e);
                        continue;
                    }
                }
            };

            match message {
                JsonWrapper::RadarView(encoded_radar) => {
                    // Décodage Base64 via la fonction interne
                    let radar_bytes = match decode_b64(&encoded_radar) {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            println!("Erreur de décodage Base64 du RadarView: {}", e);
                            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
                        }
                    };

                    // Décodage en structure RadarView
                    let radar_view_array = decode_radar_view_binary(radar_bytes);

                    // Sélection du prochain déplacement
                    let best_move = self.navigator.choose_next_move(&radar_view_array);
                    println!("[Client] Moving in direction: {:?}", best_move);
                    self.navigator.display_memory_map();

                    // Envoi de l'action au serveur
                    if let Err(e) = self.send_message(&JsonWrapper::Action(Action::MoveTo(best_move))) {
                        println!("Erreur d'envoi de message: {}", e);
                        return Err(e);
                    }
                }


                JsonWrapper::Hint(hint) => {
                    println!("Received hint: {:?}", hint);
                    match hint {
                        Hint::RelativeCompass { angle } => {
                            println!("Stored compass hint: {}°", angle);
                            // Traitement de l'indice boussole
                            // self.compass_hint = Some(angle);
                        },
                        Hint::Secret(secret) => {
                            println!("Received secret: {}", secret);
                            self.challenge_manager.set_secret(0, secret);
                        }
                        _ => {}
                    }
                }
                JsonWrapper::Challenge(challenge) => {
                    println!("Received challenge: {:?}", challenge);
                    self.last_challenge = Some(challenge.clone());

                    match challenge {
                        Challenge::SecretSumModulo(modulo) => {
                            let answer =
                                self.challenge_manager.solve_secret_sum_modulo(modulo, &[0]);
                            println!("[Client] Solving SecretModulo with answer: {}", answer);
                            self.send_message(&JsonWrapper::Action(Action::SolveChallenge {
                                answer: answer.to_string(),
                            }))?;
                        }
                        Challenge::SOS => {
                            println!("Received SOS challenge, attempting resolution...");
                            match self.challenge_manager.resolve_sos(0) {
                                Ok(_) => println!("SOS resolved successfully!"),
                                Err(err) => println!("Failed to resolve SOS: {:?}", err),
                            }
                        }
                    }
                }
                JsonWrapper::ActionError(error) => {
                    println!("Received action error: {:?}", error);
                    match error {
                        ActionError::CannotPassThroughWall => {
                            if let Some(last_dir) = self.navigator.movement_history.back().cloned() {
                                self.navigator.handle_move_failure(last_dir);
                            } else {
                                println!("No recorded move to revert.");
                            }
                        },
                        ActionError::SolveChallengeFirst => {
                            println!("A challenge must be solved first!");
                        },
                        ActionError::InvalidChallengeSolution => {
                            println!("Invalid solution, retrying challenge...");
                            if let Some(last_challenge) = &self.last_challenge {
                                match last_challenge {
                                    Challenge::SecretSumModulo(modulo) => {
                                        let answer = self.challenge_manager.solve_secret_sum_modulo(*modulo, &[0]);
                                        println!("Retrying SecretModulo with new answer: {}", answer);
                                        self.send_message(&JsonWrapper::Action(Action::SolveChallenge {
                                            answer: answer.to_string(),
                                        }))?;
                                    }
                                    _ => {
                                        println!("No retry strategy for this challenge.");
                                    }
                                }
                            } else {
                                println!("No challenge stored, cannot retry.");
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<()> {
    const SERVER_PORT: u16 = 8778;
    let server_addr = format!("localhost:{}", SERVER_PORT);

    let mut client = Client::new(&server_addr, 3)?;

    let token = client.register_team("rust_warriors")?;
    println!("Got registration token: {}", token);

    let mut new_client = Client::new(&server_addr, 3)?;
    new_client.subscribe_player("player1", &token)?;
    new_client.game_loop()?;

    Ok(())
}
