use commun::structs::ActionError;
use super::{maze::Point, player::Player};
use tracing::{info, warn};

#[derive(Clone, Debug)]
pub struct ChallengePosition {
    position: Point,
    answer: String,
    is_finish: bool,
}

impl ChallengePosition {
    pub fn new(position: Point) -> Self {
        info!("Creating new challenge at position: x={}, y={}", position.x, position.y);
        Self {
            position: Point { x: position.x, y: position.y },
            answer: String::from("0"),
            is_finish: false,
        }
    }

    pub fn update_challenge_statut(&mut self, is_finish: bool) {
        self.is_finish = is_finish;
        info!("Challenge status updated: is_finish={}", is_finish);
    }

    pub fn get_challenge_answer(&self) -> &str {
        &self.answer
    }

    pub fn get_challenge_statut(&self) -> bool {
        self.is_finish
    }

    pub fn get_challenge_position(&self) -> &Point {
        &self.position
    }
}

fn check_is_challenge_available(player: &Player) -> bool {
    let available = player.clone().get_is_challenge_actif();
    info!("Checking if challenge is available for player: {}", available);
    available
}

fn check_answer_is_correct(challenge: &ChallengePosition, answer: &str) -> bool {
    let correct = challenge.get_challenge_answer() == answer;
    info!("Checking challenge answer: expected={}, received={}, correct={}", challenge.get_challenge_answer(), answer, correct);
    correct
}

pub fn check_all_condition_challenge(answer: String, challenge: &ChallengePosition, player: &Player) -> Result<(), ActionError> {
    if !check_is_challenge_available(player) {
        warn!("Player attempted to solve a challenge without an active challenge.");
        return Err(ActionError::NoRunningChallenge);
    }

    if !check_answer_is_correct(challenge, &answer) {
        warn!("Player provided an incorrect challenge solution.");
        return Err(ActionError::InvalidChallengeSolution);
    }

    info!("Challenge successfully solved!");
    Ok(())
}

pub fn check_is_challenge_position(position_player: &Point, challenge: &ChallengePosition) -> bool {
    let position_challenge = challenge.get_challenge_position();
    let is_finish_challenge = challenge.get_challenge_statut();

    let is_correct_position = !is_finish_challenge && position_player.x == position_challenge.x && position_player.y == position_challenge.y;
    info!("Checking if player is on a challenge position: result={}", is_correct_position);
    is_correct_position
}
