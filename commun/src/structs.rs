use super::*;

//Actions
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum RelativeDirection {
    Front, 
    Right, 
    Back, 
    Left
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Action {
    MoveTo(RelativeDirection),
    SolveChallenge{ answer: String}
}

#[derive(Debug, serde::Serialize, Deserialize, PartialEq)]
pub enum ActionError {
    CannotPassThroughWall,
    CannotPassThroughOpponent,
    NoRunningChallenge,
    SolveChallengeFirst,
    InvalidChallengeSolution,
    PlayerMustBeRescued,
}

//Challenge
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Challenge {
    SecretSumModulo(u64),
    SOS,
}

//Message
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum JsonWrapper {
    RegisterTeamResult(RegisterTeamResult),
    SubscribePlayerResult(SubscribePlayerResult),
    RadarView(String),
    Challenge(Challenge),
    ActionError(ActionError),
    RegisterTeam(RegisterTeam),
    SubscribePlayer(SubscribePlayer),
    Action(Action),
    Hint(Hint),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RegistrationError {
    AlreadyRegistered, 
    InvalidName, 
    InvalidRegistrationToken, 
    TooManyPlayers
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RegisterTeamResult {
    Ok {
        expected_players: u8,
        registration_token: String
    },
    Err(RegistrationError)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SubscribePlayerResult {
    Ok,
    Err(RegistrationError),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Hint {
    RelativeCompass { angle: f32 },
    GridSize { columns: u32, rows: u32 },
    Secret(u64),
    SOSHelper,
}

//Team
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RegisterTeam {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterTeamWrapper {
    pub register_team: RegisterTeam,
}

// Subscription
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SubscribePlayer {
    pub name: String,
    pub registration_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribePlayerWrapper {
    pub subscribe_player: SubscribePlayer,
}

//Command
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    RegisterTeam {
        name: String,
    },
    SubscribePlayer {
        name: String,
        registration_token: String,
    },
    Action(Action),
}