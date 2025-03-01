use commun::decodage::decode_message;
use commun::encodage::encode_message;
use commun::structs::{Action, ActionError, Challenge, Hint, JsonWrapper, RegisterTeam, RegisterTeamResult, RelativeDirection, SubscribePlayer, SubscribePlayerResult};
use std::io::{Result, Write};
use std::net::TcpStream;
use maze_engine::challenge::ChallengeManager;

struct Client {
    stream: TcpStream,
    challenge_manager: ChallengeManager,
}

impl Client {
    fn new(server: &str) -> Result<Self> {
        let stream = TcpStream::connect(server)?;
        println!("Connected to server at {}", server);
        Ok(Client { stream, challenge_manager: ChallengeManager { secrets: Default::default(), sos_active: None } })
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
            JsonWrapper::RegisterTeamResult(RegisterTeamResult::Ok { registration_token, expected_players }) => {
                println!("Team registered successfully. Expected players: {}", expected_players);
                Ok(registration_token)
            }
            JsonWrapper::RegisterTeamResult(RegisterTeamResult::Err(err)) => {
                println!("Registration error: {:?}", err);
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Registration error"))
            }
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unexpected response")),
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
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unexpected response")),
        }
    }

    fn game_loop(&mut self) -> Result<()> {
        loop {
            let message = match self.receive_message() {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                    continue;
                }
            };

            match message {
                JsonWrapper::RadarView(radar) => {
                    println!("Received radar view: {}", radar);
                    let action = JsonWrapper::Action(Action::MoveTo(RelativeDirection::Right));
                    self.send_message(&action)?;
                }
                JsonWrapper::Challenge(challenge) => {
                    println!("Received challenge: {:?}", challenge);
                    match challenge {
                        Challenge::SecretSumModulo(modulo) => {
                            let answer = self.challenge_manager.solve_secret_sum_modulo(modulo);
                            println!("Solving SecretSumModulo with answer: {}", answer);
                            let action = JsonWrapper::Action(Action::SolveChallenge {
                                answer: answer.to_string(),
                            });
                            self.send_message(&action)?;
                        }
                        Challenge::SOS => {
                            println!("Received SOS challenge, attempting resolution...");
                            match self.challenge_manager.resolve_sos(0) {
                                Ok(_) => {
                                    println!("SOS resolved successfully!");
                                }
                                Err(err) => {
                                    println!("Failed to resolve SOS: {:?}", err);
                                }
                            }
                        }
                    }
                }
                JsonWrapper::Hint(hint) => {
                    println!("Received hint: {:?}", hint);
                    if let Hint::Secret(secret) = hint {
                        // Stocke le secret reçu pour le challenge `SecretSumModulo`
                        self.challenge_manager.set_secret(0, secret); // Remplace 0 par l'ID du joueur
                        println!("Stored secret for SecretSumModulo: {}", secret);
                    }
                }
                JsonWrapper::ActionError(error) => {
                    println!("Received action error: {:?}", error);
                    if matches!(error, ActionError::SolveChallengeFirst) {
                        println!("A challenge must be solved first!");
                    }
                }
                _ => {
                    println!("Received unknown message: {:?}", message);
                }
            }
        }
    }


}

fn main() -> Result<()> {
    const SERVER_PORT: u16 = 8778;
    let server_addr = format!("localhost:{}", SERVER_PORT);

    let mut client = Client::new(&server_addr)?;

    let token = client.register_team("rust_warriors")?;
    println!("Got registration token: {}", token);

    let mut new_client = Client::new(&server_addr)?;
    new_client.subscribe_player("player1", &token)?;
    new_client.game_loop()?;

    Ok(())
}
