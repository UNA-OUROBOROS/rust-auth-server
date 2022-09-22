use crate::api::{errors::*, model::get_database_connection, model::UserPasswords};

use diesel::prelude::*;

use rocket_okapi::okapi::{schemars, schemars::JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserCredentials<'r> {
    realm: Option<&'r str>,
    username: &'r str,
    password: &'r str,
}

pub fn login(credentials: UserCredentials) -> Result<String, ErrorDetails> {
    use crate::schema::user_passwords::dsl::*;
    let connection = &mut get_database_connection()?;
    let mut query = user_passwords.into_boxed();
    query = query.filter(user_id.eq(credentials.username));
    if credentials.realm.is_some() {
        query = query.filter(realm.eq(credentials.realm.unwrap()));
    } else {
        query = query.filter(realm.is_null());
    }
    // if realm is not specified, then we will use the null value in the filter

    let result = query
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
