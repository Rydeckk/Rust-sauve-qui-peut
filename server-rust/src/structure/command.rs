use crate::shared::libs::*;

use super::team::*;

#[derive(Serialize, Deserialize)]
pub enum Command {
    RegisterTeam {name: String},
    SubscribePlayer {name: String, registration_token: String}
}

impl Command {
    pub fn process(command: Command, stream: TcpStream, team_manager: Arc<Mutex<TeamManager>>) { 
        match command {
            Command::RegisterTeam { name } => TeamCommand::process(TeamCommand::Create(name), stream, team_manager),
            Command::SubscribePlayer { name, registration_token } => TeamCommand::process(TeamCommand::SubscribePlayer { name, registration_token }, stream, team_manager),
        }
    }
}