use commun::{structs::{Action, Command}, Arc, Mutex, TcpStream};

use super::{action::ActionFunction, challenge::ChallengePosition, player::Player, team::*};

pub trait CommandFunction {
    fn process(command: Command, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>);
}

impl CommandFunction for Command {
    fn process(command: Command, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>) { 
        match command {
            Command::RegisterTeam { name } => TeamCommand::process(TeamCommand::Create(name), stream, team_manager),
            Command::SubscribePlayer { name, registration_token } => TeamCommand::process(TeamCommand::SubscribePlayer { name, registration_token }, stream, team_manager),
            Command::Action(action) => Action::process(action, stream, player, challenge),
        }
    }
}