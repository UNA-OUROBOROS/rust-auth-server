use crate::{
    manager::{decode_user_data, encode_user_data},
    user::UserData,
};

const PUBLIC_TEST_KEY: &[u8] = // example public key
    r#"-----BEGIN PUBLIC KEY-----
MCowBQYDK2VuAyEAkOThmuwUKlejA/aXOn3Ic+d/zguTq1+Zr340FYAPCGg=
-----END PUBLIC KEY-----"#
        .as_bytes();
const PRIVATE_TEST_KEY: &[u8] = // example private key
    r#"-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VuBCIEIND8lafFpSpO7YhqB75/HZ2+m7P78ymm36V7j9uA2HR0
-----END PRIVATE KEY-----"#
        .as_bytes();

#[test]
fn test_encode_user_data() {
    let user_data = UserData::new(
        "1234567890abcdef1234567890abcdef12345678".to_string(),
        "test-realm".to_string(),
    )
    .unwrap();
    let public_key = PUBLIC_TEST_KEY.to_vec();
    let issuer = "test-issuer";
    let audiences = vec!["test-audience".to_string()];
    // set an arbitrary value of the current time
    let now_time = std::time::SystemTime::UNIX_EPOCH;
    let exp_time = std::time::Duration::from_secs(3600);
    let token = encode_user_data(
        public_key,
        &user_data,
        issuer,
        audiences,
        Some(now_time),
        Some(now_time),
        Some(exp_time),
    );
    assert!(token.is_ok());
}

#[test]
fn test_decode_user_data() {
    let public_key = PUBLIC_TEST_KEY.to_vec();
    let private_key = PRIVATE_TEST_KEY.to_vec();
    let user_data = UserData::new(
        "1234567890abcdef1234567890abcdef12345678".to_string(),
        "test-realm".to_string(),
    )
    .unwrap();

    let issuer = "test-issuer";
    let audiences = vec!["test-audience".to_string()];
    let accepted_audiences = vec!["test-audience", "test-audience2"];
    let token =
        encode_user_data(public_key, &user_data, issuer, audiences, None, None, None).unwrap();
    let result_user_data =
        decode_user_data(private_key, token.as_str(), issuer, &accepted_audiences).unwrap();
    assert_eq!(&result_user_data, &user_data);
}
