#[macro_use]
extern crate rocket;
extern crate auth_server_lib;
extern crate dboilerplate;
extern crate colored;

use colored::*;

use auth_server_lib::api::authentication;

mod catchers;

use rocket::{
    http::{ContentType, Status},
    serde::json::{serde_json::json, Json},
};

use catchers::*;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, settings::UrlObject, swagger_ui::*};

#[openapi(tag = "Users")]
#[post("/login", data = "<credentials>", format = "application/json")]
fn login(
    credentials: Json<authentication::UserCredentials<'_>>,
) -> (Status, (ContentType, serde_json::Value)) {
    match authentication::login(credentials.into_inner()) {
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
            Status::Unauthorized,
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

#[launch]
fn rocket() -> _ {
    let rocket_app =
        rocket::build().register("/", catchers![not_found, bad_request, unprocessable_entity]);
    match dboilerplate::util::configuration::is_debug() {
        false => {
            println!("{}", "*************************************".cyan());
            println!("Running in {} mode", "production".green());
            println!("{}", "*************************************".cyan());
            rocket_app.mount("/api/v1/", routes![login])
        }
        true => {
            println!("{}", "*************************************".cyan());
            println!("Running in {} mode", "debug".yellow());
            println!("{}", "*************************************".cyan());
            rocket_app
                .mount("/api/v1", openapi_get_routes![login])
                .mount(
                    "/api/v1/swagger-ui/",
                    make_swagger_ui(&SwaggerUIConfig {
                        url: "../openapi.json".to_owned(),
                        ..Default::default()
                    }),
                )
                .mount(
                    "/api/v1/rapidoc/",
                    make_rapidoc(&RapiDocConfig {
                        general: GeneralConfig {
                            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
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
