use super::*;

//Actions
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

#[derive(Debug, serde::Serialize, PartialEq)]
pub enum ActionError {
    CannotPassThroughWall,
    CannotPassThroughOpponent,
    NoRunningChallenge,
    SolveChallengeFirst,
    InvalidChallengeSolution
}

//Challenge

#[derive(serde::Serialize)]
pub enum Challenge {
    SecretModulo(u64),
    SOS,
}

//Message

#[derive(serde::Serialize)]
pub enum JsonWrapper {
    RegisterTeamResult(RegisterTeamResult),
    SubscribePlayerResult(SubscribePlayerResult),
    RadarView(String),
    Challenge(Challenge),
    ActionError(ActionError)
}

#[derive(serde::Serialize)]
pub enum RegistrationError {
    AlreadyRegistered, 
    InvalidName, 
    InvalidRegistrationToken, 
    TooManyPlayers
}

#[derive(serde::Serialize)]
pub enum RegisterTeamResult {
    Ok {
        expected_players: u8,
        registration_token: String
    },
    Err(RegistrationError)
}

#[derive(serde::Serialize)]
pub enum SubscribePlayerResult {
    Ok,
    Err(RegistrationError)
}

//Command

#[derive(Serialize, Deserialize)]
pub enum Command {
    RegisterTeam {name: String},
    SubscribePlayer {name: String, registration_token: String},
    Action(Action)
}