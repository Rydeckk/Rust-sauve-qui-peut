use crate::shared::libs::*;

use super::{action::Action, challenge::ChallengePosition, player::Player, team::*};

#[derive(Serialize, Deserialize)]
pub enum Command {
    RegisterTeam {name: String},
    SubscribePlayer {name: String, registration_token: String},
    Action(Action)
}

impl Command {
    pub fn process(command: Command, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>) { 
        match command {
            Command::RegisterTeam { name } => TeamCommand::process(TeamCommand::Create(name), stream, team_manager),
            Command::SubscribePlayer { name, registration_token } => TeamCommand::process(TeamCommand::SubscribePlayer { name, registration_token }, stream, team_manager),
            Command::Action(action) => Action::process(action, stream, player, challenge),
        }
    }
}