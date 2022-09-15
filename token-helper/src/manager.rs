use std::time::{Duration, SystemTime};

use crate::{
    token_helper::{make_jwt, make_payload, validate_jwt},
    user::UserData,
};

/// encodes an user data into a jwt token
///
/// # example
/// ```
/// use token_helper::{manager::encode_user_data, user::UserData};
///
/// let user_data = UserData::new(
///     "1234567890abcdef1234567890abcdef12345678".to_string(),
///     "test-realm".to_string(),
///     false).unwrap();
/// let public_key = r#"-----BEGIN PUBLIC KEY-----
/// MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwZ8Z7Z0Z0Z0Z0Z0Z0Z0Z
/// -----END PUBLIC KEY-----"#.as_bytes().to_vec();
///
/// let issuer = "test-issuer";
/// let audiences = vec!["test-audience".to_string()];
/// let token = encode_user_data(public_key, &user_data, issuer, audiences, None, None, None);
///
/// match token {
///     Ok(token) => println!("token: {}", token),
///     Err(err) => println!("error: {}", err),
/// }
/// ```
pub fn encode_user_data(
    public_key: Vec<u8>,
    user_data: &UserData,
    issuer: &str,
    audiences: Vec<String>,
    now_time: Option<SystemTime>,
    not_before: Option<SystemTime>,
    exp_time: Option<Duration>,
) -> Result<String, String> {
    // none equals to 1 hour
    let exp_time = exp_time.unwrap_or_else(|| Duration::from_secs(3600));
    let subject = &user_data.get_subject();
    let payload = make_payload(issuer, audiences, &now_time, &not_before, exp_time, subject);
    let jwt = make_jwt(&public_key, &payload).map_err(|e| e.to_string())?;
    Ok(jwt)
}

/// decodes a jwt token into an user data
///
/// # example
///
/// ```
/// use token_helper::{manager::decode_user_data, manager::encode_user_data, user::UserData};
///
/// let public_key = r#"-----BEGIN PUBLIC KEY-----
/// MCowBQYDK2VuAyEAkOThmuwUKlejA/aXOn3Ic+d/zguTq1+Zr340FYAPCGg=
/// -----END PUBLIC KEY-----"#.as_bytes().to_vec();
///
/// let private_key = r#"-----BEGIN PRIVATE KEY-----
/// MC4CAQAwBQYDK2VuBCIEIND8lafFpSpO7YhqB75/HZ2+m7P78ymm36V7j9uA2HR0
/// -----END PRIVATE KEY-----"#.as_bytes().to_vec();
/// let issuer = "test-issuer";
/// let audiences = vec!["test-audience".to_string()];
/// let user_data = UserData::new(
///     "1234567890abcdef1234567890abcdef12345678".to_string(),
///     "test-realm".to_string(),
///     false).unwrap();
/// let accepted_audiences = vec!["test-audience", "test-audience-2"];
/// let token = encode_user_data(public_key, &user_data, issuer, audiences, None, None, None).unwrap();
/// let user_data = decode_user_data(private_key, token.as_str(), issuer, &accepted_audiences);
///
/// match user_data {
///     Ok(user_data) => println!("user_data: {:?}", user_data),
///     Err(err) => println!("error: {}", err),
/// }
/// ```
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
