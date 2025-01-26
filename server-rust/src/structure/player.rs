use crate::shared::libs::*;
use crate::send_to_client;

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String
}

pub enum PlayCommand {
    Play
}

impl PlayCommand {
    fn process(play_command: PlayCommand) {
        match play_command {
            PlayCommand::Play => todo!()
        }
    }
}