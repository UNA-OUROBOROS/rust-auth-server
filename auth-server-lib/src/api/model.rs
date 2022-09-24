use rocket_okapi::okapi::{schemars, schemars::JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserCredentials<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

