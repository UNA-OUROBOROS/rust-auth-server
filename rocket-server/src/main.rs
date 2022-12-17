#[macro_use]
extern crate rocket;
extern crate auth_server_lib;
extern crate colored;

use colored::*;

mod catchers;
mod endpoints;

use endpoints::*;

use catchers::*;
use rocket_okapi::{openapi_get_routes, rapidoc::*, settings::UrlObject, swagger_ui::*};

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
