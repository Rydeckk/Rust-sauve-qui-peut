use commun::{serde_json, structs::{ActionError, Challenge, JsonWrapper, RegisterTeamResult, RegistrationError, SubscribePlayerResult}};
use super::player::Player;
use tracing::{info, warn};

pub fn register_team_result(result: Result<(u8, String), RegistrationError>) -> String {
    let message_wrapped = match result {
        Ok((expected_players, access_key)) => {
            info!("Register team successful: expected_players={}, access_key={}", expected_players, access_key);
            JsonWrapper::RegisterTeamResult(RegisterTeamResult::Ok { expected_players, registration_token: access_key })
        },
        Err(error) => {
            warn!("Register team failed: {:?}", error);
            JsonWrapper::RegisterTeamResult(RegisterTeamResult::Err(error))
        }
    };

    serde_json::to_string(&message_wrapped).unwrap()
}

pub fn subscribe_player_result(result: Result<Player, RegistrationError>) -> String {
    let message_wrapped = match result {
        Ok(player) => {
            info!("Player subscribed successfully");
            player.get_radar_view();
            JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Ok)
        },
        Err(error) => {
            warn!("Player subscription failed: {:?}", error);
            JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Err(error))
        }
    };

    serde_json::to_string(&message_wrapped).unwrap()
}

pub fn action_result(result: Result<(Player, bool), ActionError>) -> String {
    let message_wrapped = match result {
        Ok((player, is_challenge_position)) => {
            if is_challenge_position {
                info!("Player reached a challenge position");
                JsonWrapper::Challenge(Challenge::SecretModulo(10))
            } else {
                info!("Sending updated radar view");
                JsonWrapper::RadarView(player.get_radar_view())
            }
        },
        Err(action_err) => {
            warn!("Action resulted in error: {:?}", action_err);
            JsonWrapper::ActionError(action_err)
        }
    };

    serde_json::to_string(&message_wrapped).unwrap()
}