use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use commun::structs::{Action, Command};
use super::{action::ActionFunction, challenge::ChallengePosition, player::Player, team::*};
use tracing::{info};

pub trait CommandFunction {
    fn process(command: Command, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>);
}

impl CommandFunction for Command {
    fn process(command: Command, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>) {
        match command {
            Command::RegisterTeam { name } => {
                info!("Processing register_team for: {}", name);
                TeamCommand::process(TeamCommand::Create(name), stream, team_manager);
            }
            Command::SubscribePlayer { name, registration_token } => {
                info!("Processing subscribe_player for: {}", name);
                TeamCommand::process(TeamCommand::SubscribePlayer { name, registration_token }, stream, team_manager);
            }
            Command::Action(action) => {
                info!("Processing Action: {:?}", action);
                Action::process(action, stream, player, challenge);
            }
        }
    }
}