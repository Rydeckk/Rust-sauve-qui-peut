use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use commun::structs::{Action, RelativeDirection};
use crate::send_to_client;
use super::{
    challenge::{check_all_condition_challenge, check_is_challenge_position, ChallengePosition},
    maze::{check_movement_possible, Point},
    message::action_result,
    player::Player
};
use tracing::{info, warn, error};

pub trait ActionFunction {
    fn process_direction(direction: RelativeDirection, player: Arc<Mutex<Player>>, stream: TcpStream, challenge: Arc<Mutex<ChallengePosition>>);
    fn process_challenge(answer: String, player: Arc<Mutex<Player>>, stream: TcpStream, challenge: Arc<Mutex<ChallengePosition>>);
    fn process(action: Action, stream: TcpStream, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>);
}

impl ActionFunction for Action {
    fn process_direction(direction: RelativeDirection, player: Arc<Mutex<Player>>, stream: TcpStream, challenge: Arc<Mutex<ChallengePosition>>) {
        info!("Processing MoveTo: {:?}", direction);

        let mut player = player.lock().unwrap();
        let challenge = challenge.lock().unwrap();

        let message = match check_movement_possible(direction, &player) {
            Ok(new_position) => {
                player.set_position(Point { x: new_position.x, y: new_position.y });
                let is_challenge_position = check_is_challenge_position(&player.get_position(), &challenge);
                if is_challenge_position {
                    player.set_is_challenge_actif(true);
                    info!("Player has reached a challenge position.");
                }
                info!("Player moved to new position: x={}, y={}", new_position.x, new_position.y);
                action_result(Ok((player.clone(), is_challenge_position)))
            },
            Err(action_err) => {
                warn!("Movement failed: {:?}", action_err);
                action_result(Err(action_err))
            },
        };

        if let Err(e) = send_to_client(stream, message) {
            error!("Failed to send response to client: {}", e);
        }
    }

    fn process_challenge(answer: String, player: Arc<Mutex<Player>>, stream: TcpStream, challenge: Arc<Mutex<ChallengePosition>>) {
        info!("Processing challenge solution: {}", answer);

        let mut challenge = challenge.lock().unwrap();
        let mut player = player.lock().unwrap();

        let message = match check_all_condition_challenge(answer, &challenge, &player) {
            Ok(()) => {
                player.set_is_challenge_actif(false);
                challenge.update_challenge_statut(true);
                let is_challenge_position = check_is_challenge_position(&player.get_position(), &challenge);
                info!("Challenge successfully completed.");
                action_result(Ok((player.clone(), is_challenge_position)))
            },
            Err(action_err) => {
                warn!("Challenge failed: {:?}", action_err);
                action_result(Err(action_err))
            },
        };

        if let Err(e) = send_to_client(stream, message) {
            error!("Failed to send response to client: {}", e);
        }
    }

    fn process(action: Action, stream: TcpStream, player: Arc<Mutex<Player>>, challenge: Arc<Mutex<ChallengePosition>>) {
        match action {
            Action::MoveTo(direction) => Self::process_direction(direction, player, stream, challenge),
            Action::SolveChallenge { answer } => Self::process_challenge(answer, player, stream, challenge),
        };
    }
}
