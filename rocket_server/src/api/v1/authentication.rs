use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserCredentials<'r> {
    realm: &'r str,
    username: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
pub struct ErrorDetails {
    code: u16,
    code_name: &'static str,
    message: String,
}

pub fn login(credentials: UserCredentials) -> Result<UserCredentials, ErrorDetails> {
    if credentials.username == "admin" && credentials.password == "admin" {
        Ok(credentials)
    } else {
        Err(ErrorDetails {
            code: 401,
            code_name: "Unauthorized",
            message: "Invalid credentials".to_string(),
        })
    }
}
