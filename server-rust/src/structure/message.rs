#[derive(serde::Serialize)]
pub enum JsonWrapper {
    #[serde(rename = "RegisterTeamResult")]
    RegisterTeamResult(RegisterTeamResult),
    SubscribePlayerResult(SubscribePlayerResult)
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

pub fn subcribe_player_result(result: Result<(),RegistrationError>) -> String {
    let message_wrapped = match result {
        Ok(()) => JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Ok),
        Err(error) => JsonWrapper::SubscribePlayerResult(SubscribePlayerResult::Err(error))
    };

    serde_json::to_string(&message_wrapped).unwrap()
}