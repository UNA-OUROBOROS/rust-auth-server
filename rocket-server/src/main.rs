#[macro_use]
extern crate rocket;

pub mod api;
mod catchers;

use api::v1::authentication;
use rocket::{
    http::Status,
    response::{content, status},
    serde::json::{serde_json::json, Json},
};

use catchers::*;

#[post("/login", data = "<credentials>")]
fn index(
    credentials: Json<authentication::UserCredentials<'_>>,
) -> status::Custom<content::RawJson<String>> {
    match authentication::login(credentials.into_inner()) {
        Ok(creds) => status::Custom(
            Status::Ok,
            content::RawJson(
                json!({
                    "result": "success",
                    "credentials": creds
                })
                .to_string(),
            ),
        ),
        Err(err) => status::Custom(
            Status::Unauthorized,
            content::RawJson(
                json!({
                    "result": "failed",
                    "details": err
                })
                .to_string(),
            ),
        ),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, bad_request, unprocessable_entity])
        .mount("/api/v1/", routes![index])
}
