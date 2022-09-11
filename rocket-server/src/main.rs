#[macro_use]
extern crate rocket;

extern crate auth_server_lib;

use auth_server_lib::api::v1::authentication;

mod catchers;

use rocket::{
    http::{ContentType, Status},
    serde::json::{serde_json::json, Json},
};

use catchers::*;
use rocket_okapi::{
    openapi, openapi_get_routes,
    rapidoc::*,
    settings::UrlObject,
    swagger_ui::*,
};


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
    rocket::build()
        .register("/", catchers![not_found, bad_request, unprocessable_entity])
        // if we are in production only map the /api/v1 route
        //.mount("/api/v1/", routes![login])
        // we should check if is debug mode and show the docs only in debug mode
        .mount("/api/v1", openapi_get_routes![
            login
        ])
        .mount("/api/v1/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }))
        .mount("/api/v1/rapidoc/",
            make_rapidoc(&RapiDocConfig{
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
