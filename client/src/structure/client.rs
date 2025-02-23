// COMMENT trying to refacto with common struct but not work for the moment

use commun::{
    serde_json,
    structs::{
        Action, RegisterTeam, RegisterTeamResult, RegisterTeamWrapper, RelativeDirection,
        SubscribePlayer, SubscribePlayerResult, SubscribePlayerWrapper,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(server: &str) -> Result<Self> {
        let stream = TcpStream::connect(server)?;
        Ok(Client { stream })
    }

    pub fn send_message<T: Serialize>(&mut self, message: &T) -> Result<()> {
        let json = serde_json::to_string(message)?;
        let size = (json.len() as u32).to_le_bytes();
        self.stream.write_all(&size)?;
        self.stream.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn receive_message<T: for<'de> Deserialize<'de>>(&mut self) -> Result<T> {
        let mut size_buffer = [0; 4];
        self.stream.read_exact(&mut size_buffer)?;
        let size = u32::from_le_bytes(size_buffer);

        let mut buffer = vec![0; size as usize];
        self.stream.read_exact(&mut buffer)?;

        let response = String::from_utf8_lossy(&buffer);
        let result: T = serde_json::from_str(&response)?;
        Ok(result)
    }

    pub fn register_team(&mut self, team_name: &str) -> Result<String> {
        let registration = RegisterTeamWrapper {
            RegisterTeam: RegisterTeam {
                name: team_name.to_string(),
            },
        };

        self.send_message(&registration)?;

        let result: serde_json::Value = self.receive_message()?;
        let team_result: RegisterTeamResult =
            serde_json::from_value(result["RegisterTeamResult"].clone())?;

        match team_result {
            RegisterTeamResult::Ok {
                registration_token,
                expected_players,
            } => {
                println!(
                    "Team registered successfully. Expected players: {}",
                    expected_players
                );
                Ok(registration_token)
            }
            RegisterTeamResult::Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Registration error: {:?}", err),
            )),
        }
    }

    pub fn subscribe_player(&mut self, player_name: &str, token: &str) -> Result<()> {
        let subscription = SubscribePlayerWrapper {
            SubscribePlayer: SubscribePlayer {
                name: player_name.to_string(),
                registration_token: token.to_string(),
            },
        };

        self.send_message(&subscription)?;

        let result: serde_json::Value = self.receive_message()?;
        let subscribe_result: SubscribePlayerResult =
            serde_json::from_value(result["SubscribePlayerResult"].clone())?;
        match subscribe_result {
            SubscribePlayerResult::Ok => {
                println!("Player subscribed successfully");
                Ok(())
            }
            SubscribePlayerResult::Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Subscription error: {:?}", err),
            )),
        }
    }

    pub fn game_loop(&mut self) -> Result<()> {
        loop {
            let message: serde_json::Value = self.receive_message()?;

            if let Some(radar) = message.get("RadarView") {
                println!("Received radar view: {}", radar);

                let action = Action::MoveTo(RelativeDirection::Right);
                self.send_message(&action)?;
            } else if let Some(challenge) = message.get("Challenge") {
                println!("Received challenge: {}", challenge);
                let action = Action::SolveChallenge {
                    answer: "solution".to_string(),
                };
                self.send_message(&action)?;
            }

            if let Ok(error_message) = self.receive_message::<serde_json::Value>() {
                if let Some(error) = error_message.get("ActionError") {
                    println!("Received action error: {}", error);
                }
            }
        }
    }
}
