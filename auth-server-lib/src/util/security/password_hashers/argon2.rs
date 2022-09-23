use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher as Argon2PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};

use crate::util::security::hasher::PasswordHasher;

pub(crate) struct Argon2Hasher;

impl PasswordHasher for Argon2Hasher {
    fn hash_password(password: &[u8]) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password, &salt)
            .map_err(|e| e.to_string())?
            .to_string())
    }

    fn verify_password(password: &[u8], encoded: &[u8]) -> Result<(), String> {
        // convert the encoded hash to a &str
        let encoded_str = std::str::from_utf8(encoded).map_err(|e| e.to_string())?;
        // parse the encoded hash
        let hash = PasswordHash::new(encoded_str).map_err(|e| e.to_string())?;
        // verify the password
        Argon2::default()
            .verify_password(password, &hash)
            .map_err(|e| e.to_string())
    }
}
