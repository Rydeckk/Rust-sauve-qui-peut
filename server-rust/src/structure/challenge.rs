use commun::structs::ActionError;

use super::{maze::Point, player::Player};

#[derive(Clone, Debug)]
pub struct ChallengePosition {
    position: Point,
    answer: String,
    is_finish: bool
}

impl ChallengePosition {
    pub fn new(position: Point) -> Self {
        Self {
            position: Point {
                x: position.x,
                y: position.y
            },
            answer: String::from("0"),
            is_finish: false
        }
    }

    pub fn update_challenge_statut(&mut self, is_finish: bool) {
        self.is_finish = is_finish;
    }

    pub fn get_challenge_answer(self) -> String {
        self.answer
    }

    pub fn get_challenge_statut(self) -> bool {
        self.is_finish
    }

    pub fn get_challenge_position(self) -> Point {
        self.position
    }

}

fn check_is_challenge_available(player: &Player) -> bool {
    player.clone().get_is_challenge_actif()
}

fn check_answer_is_correct(challenge: &ChallengePosition, answer: String) -> bool {
    challenge.clone().get_challenge_answer() == answer
}

pub fn check_all_condition_challenge(answer: String, challenge: &ChallengePosition, player: &Player) -> Result<(), ActionError> {
    if check_is_challenge_available(&player) == false {return Err(ActionError::NoRunningChallenge)}

    if check_answer_is_correct(&challenge, answer) == false {return Err(ActionError::InvalidChallengeSolution)}

    Ok(())
}

pub fn check_is_challenge_position(position_player: &Point, challenge: &ChallengePosition) -> bool {
    let position_challenge = challenge.clone().get_challenge_position();
    let is_finish_challenge = challenge.clone().get_challenge_statut();

    if is_finish_challenge == false && position_player.x == position_challenge.x && position_player.y == position_challenge.y {
        return true;
    }

    false
}