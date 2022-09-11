use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

pub fn get_config<'a>(app_name: &'a str, environment: &'a str) -> Result<Figment, &'a str> {
    if is_valid_identifier(app_name) {
        let toml_name = format!("{}.toml", app_name);

        // search for the config file in the current directory
        // then in the upper directory, and so on until the root
        let mut config: Figment = Figment::new().select(environment);
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
        // merge the environment variables
        config = config.merge(Env::prefixed(app_name.to_uppercase().as_str()));
        // then merge the config file
        let config_dir = dirs::config_dir().unwrap();
        config = config.merge(Toml::file(config_dir.join(toml_name)));

        return Ok(config);
    }

    return Err("Invalid app name");
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
///
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
