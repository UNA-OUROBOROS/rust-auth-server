#[macro_use]
extern crate rocket;

pub mod api;

use api::v1::authentication;
use rocket::serde::json::{serde_json::json, Json, Value};

#[post("/login", data = "<credentials>")]
fn index(credentials: Json<authentication::UserCredentials<'_>>) -> Value {
    match authentication::login(credentials.into_inner()) {
        Ok(creds) => json!({
            "result": "success",
            "credentials": creds
        }),
        Err(err) => json!({
            "result": "failed",
            "details": err
        }),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![index])
}
