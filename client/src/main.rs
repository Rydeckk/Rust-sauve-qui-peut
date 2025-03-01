use commun::decodage::decode_message;
use commun::encodage::encode_message;
use commun::structs::{Action, JsonWrapper, RegisterTeam, RegisterTeamResult, RelativeDirection, SubscribePlayer, SubscribePlayerResult};
use std::io::{Result, Write};
use std::net::TcpStream;

struct Client {
    stream: TcpStream,
}

impl Client {
    fn new(server: &str) -> Result<Self> {
        let stream = TcpStream::connect(server)?;
        println!("Connected to server at {}", server);
        Ok(Client { stream })
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
            match self.receive_message()? {
                JsonWrapper::RadarView(radar) => {
                    println!("Received radar view: {}", radar);
                    let action = JsonWrapper::Action(Action::MoveTo(RelativeDirection::Right));
                    self.send_message(&action)?;
                }
                JsonWrapper::Challenge(challenge) => {
                    println!("Received challenge: {:?}", challenge);
                    let action = JsonWrapper::Action(Action::SolveChallenge {
                        answer: "solution".to_string(),
                    });
                    self.send_message(&action)?;
                }
                JsonWrapper::ActionError(error) => {
                    println!("Received action error: {:?}", error);
                }
                _ => {}
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
