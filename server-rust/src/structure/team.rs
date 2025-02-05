use crate::shared::{libs::*, utils::*};
use crate::send_to_client;
use super::message::{register_team_result, subcribe_player_result, RegistrationError};
use super::player::Player;

const MAX_PLAYER: u8 = 3;

pub enum TeamCommand {
    Create(String),
    SubscribePlayer {name: String, registration_token: String}
}

pub struct TeamManager {
    teams: HashMap<String, Team>
}

#[derive(Clone)]
struct Team {
    name: String,
    players: Vec<Player>
}

impl TeamManager {
    pub fn new() -> Self {
        Self {
            teams: HashMap::new(),
        }
    }

    fn create_team(&mut self, name: &String) -> Result<String,RegistrationError> {
        if self.teams.values().find(|team| team.name == *name).is_some() {
            Err(RegistrationError::AlreadyRegistered) 
        } else {
            let team = Team {
                name: name.to_string(),
                players: vec![]
            };

            let access_key = generate_acess_key();

            self.teams.insert(access_key.clone(), team.clone());
            Ok(access_key)
        }
    }

    fn register_player(&mut self, access_key: String, name: String) -> Result<(Player),RegistrationError> {
        if self.teams.contains_key(&access_key) {
            let team_players = &self.teams.get(&access_key).unwrap().players;
            if team_players.len() >= MAX_PLAYER.into() {
                return Err(RegistrationError::TooManyPlayers);
            }

            let player = Player::new(name);
            self.teams.get_mut(&access_key).unwrap().players.push(player.clone());
            Ok(player)
        } else {
            Err(RegistrationError::InvalidRegistrationToken)
        }
    }
}

impl TeamCommand {
    fn create_process(name_team: String, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) {
        let mut manager = team_manager.lock().unwrap(); 

        let message = match manager.create_team(&name_team) {
            Ok(access_key) => register_team_result(Ok((MAX_PLAYER, access_key))),
            Err(error) => register_team_result(Err(error))
        };

        send_to_client(stream,message);
    }

    fn register_player_process(name_player: String, registration_token: String, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) {
        let mut manager = team_manager.lock().unwrap(); 

        let message = match manager.register_player(registration_token, name_player) {
            Ok(player) => subcribe_player_result(Ok(player)),
            Err(error) => subcribe_player_result(Err(error))
        };

        send_to_client(stream,message);
    }

    pub fn process(command: TeamCommand, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) {
        match command {
            TeamCommand::Create(name_team) => TeamCommand::create_process(name_team, stream, team_manager),
            TeamCommand::SubscribePlayer { name, registration_token } => TeamCommand::register_player_process(name, registration_token, stream, team_manager),
        };
    }
}