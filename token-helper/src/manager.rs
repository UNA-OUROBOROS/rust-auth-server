use std::time::Duration;

use crate::{
    token_helper::{make_jwt, make_payload, validate_jwt},
    user::UserData,
};

pub fn encode_user_data(
    public_key: Vec<u8>,
    user_data: UserData,
    issuer: &str,
    audiences: Vec<String>,
    exp_time: Duration,
) -> Result<String, String> {
    let subject = &user_data.get_subject();
    let payload = make_payload(issuer, audiences, &None, &None, exp_time, subject);
    let jwt = make_jwt(&public_key, &payload).map_err(|e| e.to_string())?;
    Ok(jwt)
}

pub fn decode_user_data(
    private_key: Vec<u8>,
    jwt: &str,
    expected_issuer: &str,
    accepted_audiences: &Vec<&str>,
) -> Result<UserData, String> {
    let subject = validate_jwt(&private_key, jwt, expected_issuer, accepted_audiences)?;
    let user_data = UserData::from_subject(subject.as_str())?;
    Ok(user_data)
}
