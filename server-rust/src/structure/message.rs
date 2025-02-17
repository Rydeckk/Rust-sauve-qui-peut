use commun::{serde_json, structs::{ActionError, Challenge, JsonWrapper, RegisterTeamResult, RegistrationError, SubscribePlayerResult}};

use super::player::Player;

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