use std::time::SystemTime;

use josekit::jwt::JwtPayload;

use crate::token_helper::{
    decrypt_jwt, make_jwt, make_payload, validate_header, validate_jwt, validate_payload,
};

// example test keys for testing DO NOT USE IN PRODUCTION
const PUBLIC_TEST_KEY: &str = // example public key
    r#"-----BEGIN PUBLIC KEY-----
    MCowBQYDK2VuAyEAkOThmuwUKlejA/aXOn3Ic+d/zguTq1+Zr340FYAPCGg=
    -----END PUBLIC KEY-----"#;
const PRIVATE_TEST_KEY: &str = // example private key
    r#"-----BEGIN PRIVATE KEY-----
    MC4CAQAwBQYDK2VuBCIEIND8lafFpSpO7YhqB75/HZ2+m7P78ymm36V7j9uA2HR0
    -----END PRIVATE KEY-----"#;
const JWT_TEST_OUTPUT: &str ="eyJ0eXAiOiJKV1QiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwiZXBrIjp7Imt0eSI6Ik9LUCIsImNydiI6IlgyNTUxOSIsIngiOiJZZzJDSzd0b1ZkTG9WQTc1anhJTnFXYXlyR2NUS3VldzRxZWRJZFZHcW5RIn0sImFsZyI6IkVDREgtRVMifQ..eu_biZDuPyNer8tUywCBGw.QWs5dpsREvrhktA3OwECIw.Q1QBPrvZlzuSBJ7XF4fGRw";

#[test]
fn test_make_jwt() {
    let public_key = PUBLIC_TEST_KEY.as_bytes().to_vec();
    let mut payload = JwtPayload::new();
    payload.set_issuer("test");
    let jwt = make_jwt(&public_key, &payload);
    assert!(jwt.is_ok());
}

#[test]
fn test_make_payload() {
    let issuer = "test.localdomain";
    let audiences = vec!["test.app".to_string()];
    let subject = "test";
    let now_time = SystemTime::now();
    let not_before = now_time;
    let exp_time = std::time::Duration::from_secs(3600);
    let payload = make_payload(
        issuer,
        audiences.to_vec(),
        &Some(now_time),
        &Some(not_before),
        exp_time,
        subject,
    );

    let payload_issuer = payload.issuer().unwrap();
    let payload_audiences = payload.audience().unwrap();
    let payload_subject = payload.subject().unwrap();
    let payload_issued_at = payload.issued_at().unwrap();
    let payload_not_before = payload.not_before().unwrap();
    let payload_expiration = payload.expires_at().unwrap();

    assert_eq!(payload_issuer, issuer);
    // the audience should have the same elements as the audiences vector
    for i in 0..payload_audiences.len() {
        assert_eq!(payload_audiences[i], audiences[i]);
    }
    // the payload issued at time should be the same as the now_time
    // however the payload issued at time is in seconds and the now_time is in nanoseconds
    // so we need to convert the now_time to seconds
    let now_time_secs = now_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let now_time_secs: SystemTime =
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(now_time_secs);
    // the same goes for not before and expiration
    let not_before_secs = not_before
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let not_before_secs: SystemTime =
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(not_before_secs);

    assert_eq!(payload_subject, subject);
    println!("payload issued at: {:?}", payload_issued_at);
    println!("now time: {:?}", now_time_secs);
    assert_eq!(payload_issued_at, now_time_secs);
    assert_eq!(payload_not_before, not_before_secs);
    assert_eq!(payload_expiration, now_time_secs + exp_time);
}

#[test]
fn test_decrypt_jwt() {
    let private_key = PRIVATE_TEST_KEY.as_bytes().to_vec();
    let jwt = JWT_TEST_OUTPUT;
    let decrypted = decrypt_jwt(&private_key, jwt);
    assert!(decrypted.is_ok());
}

#[test]
fn test_validate_header() {
    let private_key = PRIVATE_TEST_KEY.as_bytes().to_vec();
    let jwt = JWT_TEST_OUTPUT;
    let decrypted = decrypt_jwt(&private_key, jwt);
    assert!(decrypted.is_ok());
    let (_, header) = decrypted.unwrap();
    assert_eq!(header.token_type().unwrap(), "JWT");
    assert_eq!(header.algorithm().unwrap(), "ECDH-ES");
    assert!(validate_header(&header).is_ok());
}

#[test]
fn test_validate_payload() {
    let private_key = PRIVATE_TEST_KEY.as_bytes().to_vec();
    let public_key = PUBLIC_TEST_KEY.as_bytes().to_vec();
    let issuer = "test.localdomain";
    let audiences = vec!["test.app".to_string()];
    let subject = "test";
    let now_time = SystemTime::now();
    let not_before = now_time;
    let exp_time = std::time::Duration::from_secs(3600);
    let payload = make_payload(
        issuer,
        audiences.to_vec(),
        &Some(now_time),
        &Some(not_before),
        exp_time,
        subject,
    );
    let jwt = make_jwt(&public_key, &payload);
    assert!(jwt.is_ok());
    let jwt = jwt.unwrap();
    let decrypted = decrypt_jwt(&private_key, &jwt);
    assert!(decrypted.is_ok());
    let (payload, _) = decrypted.unwrap();
    assert!(validate_payload(&payload, issuer, &vec!["test.app"]).is_ok());
}

#[test]
fn test_validate_jwt() {
    let private_key = PRIVATE_TEST_KEY.as_bytes().to_vec();
    let public_key = PUBLIC_TEST_KEY.as_bytes().to_vec();
    let issuer = "test.localdomain";
    let audiences = vec!["test.app".to_string()];
    let subject = "test";
    let now_time = SystemTime::now();
    let not_before = now_time;
    let exp_time = std::time::Duration::from_secs(3600);
    let payload = make_payload(
        issuer,
        audiences.to_vec(),
        &Some(now_time),
        &Some(not_before),
        exp_time,
        subject,
    );
    let jwt = make_jwt(&public_key, &payload);
    assert!(jwt.is_ok());
    let jwt = jwt.unwrap();
    let decrypted = decrypt_jwt(&private_key, &jwt);
    assert!(decrypted.is_ok());
    let (payload, _) = decrypted.unwrap();
    assert!(validate_payload(&payload, issuer, &vec!["test.app"]).is_ok());
    assert!(validate_jwt(&private_key, &jwt, issuer, &vec!["test.app"]).is_ok());
}
