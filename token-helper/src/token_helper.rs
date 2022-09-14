use std::time::SystemTime;

use josekit::{
    jwe::{JweHeader, ECDH_ES},
    jwt::{self, JwtPayload},
    JoseError,
};

/// makes a new encrypted json web token
/// the token is encrypted with the public key of the user
pub(crate) fn make_jwt(public_key: &Vec<u8>, payload: &JwtPayload) -> Result<String, JoseError> {
    let mut header = JweHeader::new();
    header.set_token_type("JWT");
    header.set_content_encryption("A128CBC-HS256");

    let encrypter = ECDH_ES.encrypter_from_pem(public_key)?;
    let jwe = jwt::encode_with_encrypter(payload, &header, &encrypter)?;
    Ok(jwe)
}

pub(crate) fn make_payload(
    issuer: &str,
    audiences: Vec<String>,
    now_time: &Option<SystemTime>,
    not_before: &Option<SystemTime>,
    exp_time: std::time::Duration,
    subject: &str,
) -> JwtPayload {
    let now_time = now_time.unwrap_or_else(SystemTime::now);
    let expiration_time: SystemTime = now_time + exp_time;

    let mut payload = JwtPayload::new();
    payload.set_issuer(issuer);
    payload.set_subject(subject);
    payload.set_audience(audiences);
    payload.set_issued_at(&now_time);
    payload.set_expires_at(&expiration_time);
    payload.set_not_before(&not_before.unwrap_or(now_time));

    payload
}

pub(crate) fn decrypt_jwt(
    private_key: &Vec<u8>,
    jwt: &str,
) -> Result<(JwtPayload, JweHeader), String> {
    let decrypter = ECDH_ES
        .decrypter_from_pem(private_key)
        .map_err(|e| e.to_string())?;
    let (payload, header) =
        jwt::decode_with_decrypter(jwt, &decrypter).map_err(|e| e.to_string())?;
    Ok((payload, header))
}

// validates the header of a jwt
// checks if the type and the encryption are correct
pub(crate) fn validate_header(header: &JweHeader) -> Result<(), String> {
    let typ = header.claim("typ").ok_or("no JWT token type")?;
    let enc = header.claim("enc").ok_or("no JWT token encryption")?;
    if typ != "JWT" {
        return Err("invalid JWT token type".to_string());
    }
    if enc != "A128CBC-HS256" {
        return Err("invalid JWT token encryption".to_string());
    }
    Ok(())
}
/// validates the payload of a jwt
/// and returns the subject of the token
/// if not returns an error
pub(crate) fn validate_payload(
    payload: &JwtPayload,
    expected_issuer: &str,
    accepted_audiences: &Vec<&str>,
) -> Result<String, String> {
    let issuer = payload.issuer().ok_or("no issuer")?;
    let subject = payload.subject().ok_or("no subject")?;
    let audiences = payload.audience().ok_or("no audience")?;
    let issued_at = payload.issued_at().ok_or("no issued at")?;
    let expires_at = payload.expires_at().ok_or("no expires at")?;
    let not_before = payload.not_before().ok_or("no not before")?;
    let time_now = SystemTime::now();

    if issuer != expected_issuer {
        return Err("invalid issuer".to_string());
    }
    if !audiences.iter().any(|a| accepted_audiences.contains(&a)) {
        return Err("there is not an valid audience for this token".to_string());
    }
    if issued_at > time_now {
        return Err("token issued in the future".to_string());
    }
    if expires_at < time_now {
        return Err("token expired".to_string());
    }
    if not_before > time_now {
        return Err("token not valid yet".to_string());
    }
    // the application itself can validate the user data such
    // as realm, is alias, and the user id
    Ok(subject.to_string())
}

/// validates a jwt and returns the subject of the token
pub(crate) fn validate_jwt(
    private_key: &Vec<u8>,
    jwt: &str,
    expected_issuer: &str,
    accepted_audiences: &Vec<&str>,
) -> Result<String, String> {
    let (payload, header) = decrypt_jwt(private_key, jwt)?;
    validate_header(&header)?;
    validate_payload(&payload, expected_issuer, accepted_audiences)
}
