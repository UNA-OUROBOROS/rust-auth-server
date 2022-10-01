#[macro_use]
extern crate rocket;
extern crate auth_server_lib;
extern crate colored;



use colored::*;

use auth_server_lib::api::{endpoints, model};

mod catchers;

use rocket::{
    http::{ContentType, Status},
    serde::json::{serde_json::json, Json},
};

use catchers::*;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, settings::UrlObject, swagger_ui::*};

#[openapi(tag = "Users")]
#[post("/email/login", data = "<credentials>", format = "application/json")]
fn login(
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
fn register_by_email_password(
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

#[launch]
fn rocket() -> _ {
    let base_url = "/auth";
    let openapi_json_url = format!("{}/openapi.json", base_url);
    let rocket_app =
        rocket::build().register("/", catchers![not_found, bad_request, unprocessable_entity]);
    match cfg!(debug_assertions) {
        false => {
            println!("{}", "*************************************".cyan());
            println!("Running in {} mode", "production".green());
            println!("{}", "*************************************".cyan());
            rocket_app.mount(base_url, routes![login, register_by_email_password])
        }
        true => {
            println!("{}", "*************************************".cyan());
            println!("Running in {} mode", "debug".yellow());
            println!("{}", "*************************************".cyan());
            rocket_app
                .mount(
                    base_url,
                    openapi_get_routes![login, register_by_email_password],
                )
                .mount(
                    format!("{}/api/v1/swagger-ui/", base_url),
                    make_swagger_ui(&SwaggerUIConfig {
                        url: openapi_json_url.to_owned(),
                        ..Default::default()
                    }),
                )
                .mount(
                    format!("{}/api/v1/rapidoc/", base_url),
                    make_rapidoc(&RapiDocConfig {
                        general: GeneralConfig {
                            spec_urls: vec![UrlObject::new("General", &openapi_json_url)],
                            ..Default::default()
                        },
                        hide_show: HideShowConfig {
                            allow_spec_url_load: false,
                            allow_spec_file_load: false,
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                )
        }
    }
}
