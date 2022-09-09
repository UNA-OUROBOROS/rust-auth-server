use figment::{
    providers::{Env, Toml},
    Figment,
};

pub fn getConfig(app_name: &str) {
    if (is_valid_identifier(app_name)) {
        let toml_name = format!("{}.toml", app_name);

        // search for the config file in the current directory
        // then in the upper directory, and so on until the root
        let config = Figment::new();
        // get the current directory
        let mut current_dir = std::env::current_dir();
        // loop until the root directory is reached
        loop {
            // check if the config file exists
            if current_dir.join("config.toml").exists() {
                // if it does, load it
                config.merge(Toml::file(current_dir.join("config.toml")));
                break;
            }
            // if it doesn't, go up one directory (and check if it exists)
            if let Some(parent) = current_dir.parent() {
                current_dir = parent;
            } else {
                // if the parent doesn't exist, we've reached the root
                break;
            }
        }
        // then merge the config file 
        let config_dir = dirs::config_dir().unwrap();
    }
    Err<String> {
        "Invalid app name"
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
///
/// fn main() -> io::Result<()> {
///    let invalid_name = "invalid name";
///    let valid_name = "valid_name";
///   assert_eq!(is_valid_identifier(invalid_name), false);
///  assert_eq!(is_valid_identifier(valid_name), true);
/// }
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
