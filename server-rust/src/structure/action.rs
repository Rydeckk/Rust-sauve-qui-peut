use crate::{send_to_client, shared::libs::*};

use super::{challenge::{check_all_condition_challenge, check_is_challenge_position, ChallengePosition}, maze::{check_movement_possible, Point}, message::action_result, player::Player};

#[derive(Serialize, Deserialize)]
pub enum RelativeDirection {
    Front, 
    Right, 
    Back, 
    Left
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    MoveTo(RelativeDirection),
    SolveChallenge{ answer: String}
}

#[derive(serde::Serialize)]
pub enum ActionError { 
    CannotPassThroughWall, 
    CannotPassThroughOpponent, 
    NoRunningChallenge, 
    SolveChallengeFirst, 
    InvalidChallengeSolution 
}

impl Action {
    fn process_direction(direction: RelativeDirection, player: Arc<Mutex<Player>>, stream: TcpStream, challenge: Arc<Mutex<ChallengePosition>>) {
        let mut player = player.lock().unwrap();
        let challenge = challenge.lock().unwrap();

        let message = match check_movement_possible(direction, &player) {
            Ok(new_position) => {
                player.set_position(Point { x: new_position.x, y:new_position.y});
                let is_challenge_position = check_is_challenge_position(&player.clone().get_position(), &challenge);
                if is_challenge_position == true {
                    player.set_is_challenge_actif(true);
                } 
                action_result(Ok((player.clone(), is_challenge_position)))
            },
            Err(action_err) => action_result(Err(action_err)),
        };

        send_to_client(stream,message);
    }

    fn process_challenge(answer: String, player: Arc<Mutex<Player>>, stream: TcpStream, challenge: Arc<Mutex<ChallengePosition>>) {
        let mut challenge = challenge.lock().unwrap();
        let mut player = player.lock().unwrap();
        
        let message = match check_all_condition_challenge(answer, &challenge,&player) {
            Ok(()) => {
                player.set_is_challenge_actif(false);
                challenge.update_challenge_statut(true);
                let is_challenge_position = check_is_challenge_position(&player.clone().get_position(), &challenge);
                action_result(Ok((player.clone(),is_challenge_position)))
            },
            Err(action_err) => action_result(Err(action_err)),
        };

        send_to_client(stream,message);
    }

    pub fn process(action: Action, stream: TcpStream, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>) {
        match action {
            Action::MoveTo(direction) => Action::process_direction(direction, player, stream, challenge),
            Action::SolveChallenge { answer } => Action::process_challenge(answer, player, stream, challenge),
        };
    }
}