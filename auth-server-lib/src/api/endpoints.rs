use crate::api::{errors::*, model::get_database_connection, model::UserPasswords};
use crate::schema::user_passwords::dsl::*;
use diesel::prelude::*;
use token_helper::user::UserData;

use rocket_okapi::okapi::{schemars, schemars::JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserCredentials<'r> {
    realm: &'r str,
    username: &'r str,
    password: &'r str,
}

pub fn login(credentials: UserCredentials) -> Result<String, ErrorDetails> {
    let connection = &mut get_database_connection()?;
    let result = user_passwords
        .filter(user_id.eq(credentials.username))
        .filter(realm.eq(credentials.realm))
        .load::<UserPasswords>(connection)
        .map_err(|e| ERR_BACKEND_QUERY_FAILED.with_internal_error(e.to_string()))?;
    match result.get(0) {
        Some(user) => {
            // TODO: hash password
            if user.password == credentials.password {
                //let user_data = UserData::new(user.username.clone(), user.realm.clone());
                return Ok("".to_string());
            } else {
                Err(ERR_AUTHENTICATION_FAILED)
            }
        }
        None => Err(ERR_AUTHENTICATION_FAILED.with_internal_error("user not found".to_string())),
    }
}
