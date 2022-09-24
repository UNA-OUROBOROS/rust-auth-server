use crate::{
    api::{
        errors::*,
        model::{get_database_connection, get_user_credentials},
    },
    util::security::password_hasher::{argon2::Argon2Hasher, PasswordHasher},
};

use rocket_okapi::okapi::{schemars, schemars::JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserCredentials<'r> {
    email: &'r str,
    password: &'r str,
}

pub fn login(credentials: UserCredentials) -> Result<String, ErrorDetails> {
    let connection = &mut get_database_connection()?;
    let user = get_user_credentials(connection, credentials.email)?;

    match <Argon2Hasher as PasswordHasher>::verify_password(
        credentials.password.as_bytes(),
        user.password_hash.as_bytes(),
    ) {
        Ok(_) => {
            //let user_data = UserData::new(user.username.clone(), user.realm.clone());
            return Ok("".to_string());
        }
        Err(e) => {
            return Err(ERR_AUTHENTICATION_FAILED.with_internal_error(e.to_string()));
        }
    }
}


