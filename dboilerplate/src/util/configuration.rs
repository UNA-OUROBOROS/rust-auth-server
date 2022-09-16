use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use dotenvy::dotenv;

/// search for the config file in the current directory
/// then in the upper directory, and so on until the root
fn get_default_config_file(environment: &str) -> Figment {
    let mut config = Figment::new().select(environment);
    // get the current directory
    let mut current_dir = std::env::current_dir().unwrap();
    // loop until the root directory is reached
    loop {
        // check if the config file exists
        if current_dir.join("config.toml").exists() {
            // if it does, load it
            config = config.merge(Toml::file(current_dir.join("config.toml")));
            break;
        }
        // if it doesn't, go up one directory (and check if it exists)
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            // if the parent doesn't exist, we've reached the root
            break;
        }
    }
    return config;
}

pub fn get_config<'a>(
    app_name: Option<String>,
    environment: Option<&'a str>,
) -> Result<Figment, &'a str> {
    let environment = match environment {
        Some(env) => env,
        None => "default",
    };
    let mut config: Figment = get_default_config_file(environment);
    // try to check if is in the config or in the arguments
    let app_name: Option<String> = if app_name.is_some() {
        app_name
    } else {
        get_default_app_name(Some(environment.to_string()))
    };
    match app_name {
        Some(app_name) => {
            if is_valid_identifier(&app_name) {
                let toml_name = format!("{}.toml", app_name);
                // merge the environment variables
                config = config.merge(Env::prefixed(app_name.to_uppercase().as_str()));
                // then merge the app config file
                let config_dir = dirs::config_dir().unwrap();
                config = config.merge(Toml::file(config_dir.join(toml_name)));
                return Ok(config);
            }

            Err("Invalid app name")
        }
        None => {
            // just return the default config
            Ok(config)
        }
    }
}

/// Checks if the application name is an valid identifier.
/// true if the application name is a valid identifier, false otherwise.
///
/// non valid characters are: `/\?%*:|"<>.,;= `
///
/// # Examples
///
/// ```
/// use dboilerplate::util::configuration::is_valid_identifier;
///
/// assert_eq!(is_valid_identifier("invalid name"), false);
/// assert_eq!(is_valid_identifier("valid_name"), true);
/// ```
pub fn is_valid_identifier(app_name: &str) -> bool {
    static INVALID_CHARS: [char; 15] = [
        '\\', '/', '?', '%', '*', ':', '|', '"', '<', '>', '.', ',', ';', '=', ' ',
    ];
    if app_name.is_empty() {
        return false;
    }
    for c in app_name.chars() {
        if INVALID_CHARS.contains(&c) {
            return false;
        }
    }
    true
}

/// checks wheter we are in debug mode or not
pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}

/// returns the current application name if exists
fn get_default_app_name(environment: Option<String>) -> Option<String> {
    let environment = match environment {
        Some(env) => env,
        None => get_default_environment(),
    };
    let config = get_default_config_file(&environment);
    let app_name: Option<String> = config.extract().unwrap_or_default();
    if app_name.is_some() {
        return app_name;
    }

    // check in the program arguments
    for arg in std::env::args() {
        if arg.starts_with("--app-name=") {
            return Some(arg.replace("--app-name=", ""));
        }
    }
    // check in dotenv file (dotenvy)
    if dotenv().is_ok() {
        for (key, value) in std::env::vars() {
            if key == "APP_NAME" {
                return Some(value);
            }
        }
    }
    // check in the environment variables
    if let Ok(app_name) = std::env::var("APP_NAME") {
        return Some(app_name);
    }
    // nothing found
    None
}

/// returns the current environment, if not returns default
fn get_default_environment() -> String {
    // check in the program arguments
    for arg in std::env::args() {
        if arg.starts_with("--environment=") {
            return arg.replace("--environment=", "");
        }
    }
    // load dotenv file (dotenvy)
    dotenv().ok();
    // check in the environment variables
    if let Ok(environment) = std::env::var("APP_ENVIRONMENT") {
        return environment;
    }
    // check if we are in debug mode
    if is_debug() {
        return "debug".to_string();
    }
    // nothing found
    "default".to_string()
}
