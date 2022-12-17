use auth_server_lib::api::{endpoints, model};
use rocket_okapi::{openapi};


use rocket::{
    http::{ContentType, Status},
    serde::json::{serde_json::json, Json},
};

#[openapi(tag = "Users")]
#[post("/email/login", data = "<credentials>", format = "application/json")]
pub(crate) fn login(
    credentials: Json<model::UserCredentials<'_>>,
) -> (Status, (ContentType, serde_json::Value)) {
    // set the cors header
    match endpoints::login(credentials.into_inner()) {
        Ok(creds) => (
            Status::Ok,
            (
                ContentType::JSON,
                json!({
                    "result": "success",
                    "credentials": creds
                }),
            ),
        ),
        Err(err) => (
            Status::new(err.http_code),
            (
                ContentType::JSON,
                json!({
                    "result": "failed",
                    "error": err
                }),
            ),
        ),
    }
}

#[openapi(tag = "Users")]
#[post("/email/register", data = "<credentials>", format = "application/json")]
pub(crate) fn register_by_email_password(
    credentials: Json<model::UserCredentials<'_>>,
) -> (Status, (ContentType, serde_json::Value)) {
    match endpoints::register_new_user_email_password(credentials.into_inner()) {
        Ok(user_id) => (
            Status::Ok,
            (
                ContentType::JSON,
                json!({
                    "result": "success",
                    "user_id": user_id
                }),
            ),
        ),
        Err(err) => {
            //
            if err.code_name == "ERR-RECORD-ALREADY-EXISTS" {
                return (
                    Status::new(err.http_code),
                    (
                        ContentType::JSON,
                        json!({
                            "result": "failed",
                            "error": "MAIL-ALREADY-IN-USE"
                        }),
                    ),
                );
            }
            (
                Status::new(err.http_code),
                (
                    ContentType::JSON,
                    json!({
                        "result": "failed",
                        "error": err
                    }),
                ),
            )
        }
    }
}
