use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserCredentials<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
pub struct ErrorDetails {
    code: u16,
    message: String,
}

pub fn login(credentials: UserCredentials) -> Result<UserCredentials, ErrorDetails> {
    if credentials.username == "admin" && credentials.password == "admin" {
        Ok(credentials)
    } else {
        Err(ErrorDetails {
            code: 401,
            message: "Invalid credentials".to_string(),
        })
    }
}
