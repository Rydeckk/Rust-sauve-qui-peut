use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use commun::{utils::generate_acess_key, structs::RegistrationError};
use crate::send_to_client;
use super::message::{register_team_result, subscribe_player_result };
use super::player::Player;
use tracing::{info, warn, error};

const MAX_PLAYER: u8 = 3;

pub enum TeamCommand {
    Create(String),
    SubscribePlayer { name: String, registration_token: String },
}

pub struct TeamManager {
    teams: HashMap<String, Team>,
}

#[derive(Clone)]
struct Team {
    name: String,
    players: Vec<Player>,
}

impl TeamManager {
    pub fn new() -> Self {
        info!("Initializing new TeamManager");
        Self {
            teams: HashMap::new(),
        }
    }

    fn create_team(&mut self, name: &String) -> Result<String, RegistrationError> {
        if self.teams.values().any(|team| team.name == *name) {
            warn!("Team '{}' already exists", name);
            return Err(RegistrationError::AlreadyRegistered);
        }

        let team = Team {
            name: name.to_string(),
            players: vec![],
        };

        let access_key = generate_acess_key();
        self.teams.insert(access_key.clone(), team.clone());

        info!("Team '{}' created with access key: {}", name, access_key);
        Ok(access_key)
    }

    fn register_player(&mut self, access_key: String, name: String) -> Result<Player, RegistrationError> {
        match self.teams.get_mut(&access_key) {
            Some(team) => {
                if team.players.len() >= MAX_PLAYER.into() {
                    warn!("Team '{}' already has the maximum number of players", team.name);
                    return Err(RegistrationError::TooManyPlayers);
                }

                let player = Player::new(name.clone());
                team.players.push(player.clone());

                info!("Player '{}' successfully registered in team '{}'", name, team.name);
                Ok(player)
            }
            None => {
                warn!("Invalid registration token: {}", access_key);
                Err(RegistrationError::InvalidRegistrationToken)
            }
        }
    }
}

impl TeamCommand {
    fn create_process(name_team: String, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) {
        let mut manager = team_manager.lock().unwrap();
        info!("Processing team creation for '{}'", name_team);

        let message = match manager.create_team(&name_team) {
            Ok(access_key) => register_team_result(Ok((MAX_PLAYER, access_key))),
            Err(error) => {
                error!("Failed to create team '{}': {:?}", name_team, error);
                register_team_result(Err(error))
            }
        };

        if let Err(e) = send_to_client(stream, message) {
            error!("Failed to send team creation response: {}", e);
        }
    }

    fn register_player_process(name_player: String, registration_token: String, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) {
        let mut manager = team_manager.lock().unwrap();
        info!(
            "Processing player registration: '{}' with token '{}'",
            name_player, registration_token
        );

        let message = match manager.register_player(registration_token.clone(), name_player.clone()) {
            Ok(player) => {
                info!("Player '{}' successfully subscribed", name_player);
                subscribe_player_result (Ok(player))
            }
            Err(error) => {
                error!("Failed to register player '{}': {:?}", name_player, error);
                subscribe_player_result (Err(error))
            }
        };

        if let Err(e) = send_to_client(stream, message) {
            error!("Failed to send player registration response: {}", e);
        }
    }

    pub fn process(command: TeamCommand, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) {
        match command {
            TeamCommand::Create(name_team) => Self::create_process(name_team, stream, team_manager),
            TeamCommand::SubscribePlayer {
                name,
                registration_token,
            } => Self::register_player_process(name, registration_token, stream, team_manager),
        };
    }
}
