use super::{action::ActionError, challenge::{check_is_challenge_position, Challenge, ChallengePosition}, player::Player};

#[derive(serde::Serialize)]
pub enum JsonWrapper {
    #[serde(rename = "RegisterTeamResult")]
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

pub fn register_team_result(result: Result<(u8, String),RegistrationError>) -> String {
    let message_wrapped = match result {
        Ok((expected_players, access_key)) => JsonWrapper::RegisterTeamResult(RegisterTeamResult::Ok {expected_players, registration_token: access_key,}),
        Err(error) => JsonWrapper::RegisterTeamResult(RegisterTeamResult::Err(error))
    };

    serde_json::to_string(&message_wrapped).unwrap()
}

pub fn subcribe_player_result(result: Result<Player,RegistrationError>) -> String {
    let message_wrapped = match result {
        Ok(player) => {
            player.get_radar_view();
            JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Ok)
        },
        Err(error) => JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Err(error))
    };

    serde_json::to_string(&message_wrapped).unwrap()
}

pub fn action_result(result: Result<(Player, bool), ActionError>) -> String {
    let message_wrapped = match result {
        Ok((player, is_challenge_position)) => {
            if is_challenge_position == true {
                JsonWrapper::Challenge(Challenge::SecretModulo(10))
            } else {
                JsonWrapper::RadarView(player.get_radar_view())
            }
            
        },
        Err(action_err) => JsonWrapper::ActionError(action_err)
    };

    serde_json::to_string(&message_wrapped).unwrap()
}