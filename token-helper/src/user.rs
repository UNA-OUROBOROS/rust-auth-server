#[derive(Debug, PartialEq, Eq)]
pub struct UserData {
    // an user id of at least 36 characters long (alphanumeric)
    // this should be used for the application to identify the user
    pub user_id: String,
    // the user realm where he comes from (so the user can have the same id in different realms)
    // however the user id is unique across all realms
    pub realm: String,
    // used if this user is an alias of another user(ex: the user was mapped from 2fa to an existing user)
    // accepted values '=' for false and ~ for true
    pub is_alias: bool,
}

impl UserData {
    /// Creates a new valid user payload data
    /// the user id must be at least 36 characters long (alphanumeric-case insensitive)
    /// the ream needs to be a valid domain name (not required to be a valid domain)}
    ///
    /// # example
    /// ```
    /// use token_helper::user::UserData;
    ///
    /// let user_data = UserData::new("1234567890abcdef1234567890abcdef12345678".to_string(), "test-realm".to_string(), false);
    /// println!("{:?}", user_data);
    /// assert!(user_data.is_ok());
    /// let invalid_user_data = UserData::new("invalid_user".to_string(), "test-realm".to_string(), true);
    /// assert!(invalid_user_data.is_err());
    /// ```
    pub fn new(user_id: String, realm: String, is_alias: bool) -> Result<Self, String> {
        if user_id.len() < 36 {
            return Err("user id must be at least 36 characters long".to_string());
        }
        // check if the user id is alphanumeric
        if !user_id.chars().all(|c| c.is_alphanumeric()) {
            return Err("user id must be alphanumeric".to_string());
        }
        if realm.len() > 63 || realm.len() == 0 {
            return Err("realm must be shorter than 63 characters and not empty".to_string());
        }
        // check that does not start or end with a hyphen
        if realm.starts_with('-') || realm.ends_with('-') {
            return Err("realm cannot start or end with a hyphen".to_string());
        }
        // check that only contains alphanumeric and hyphens and dots
        if !realm
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '.')
        {
            return Err("realm can only contain alphanumeric, hyphens and dots".to_string());
        }
        // check that the dots are not next to each other
        if realm.contains("..") {
            return Err("realm cannot contain 2 dots next to each other".to_string());
        }

        Ok(Self {
            user_id,
            realm,
            is_alias,
        })
    }

    /// constructs the user data from a subject string
    ///
    /// #example
    /// ```
    /// use token_helper::user::UserData;
    ///
    /// let subject = "=:1234567890abcdef1234567890abcdef12345678:example.com";
    /// let user_data = UserData::from_subject(subject).unwrap();
    /// assert_eq!(user_data.user_id, "1234567890abcdef1234567890abcdef12345678");
    /// assert_eq!(user_data.realm, "example.com");
    /// assert_eq!(user_data.is_alias, false);
    /// ```
    ///
    pub fn from_subject(subject: &str) -> Result<Self, String> {
        let mut parts = subject.split(':');
        // the alias can be = or ~
        let is_alias = match parts.next().ok_or("invalid alias")? {
            "=" => false,
            "~" => true,
            _ => return Err("invalid alias".to_string()),
        };
        let user_id = parts.next().ok_or("invalid subject")?.to_string();
        let realm = parts.next().ok_or("invalid subject")?.to_string();
        Self::new(user_id, realm, is_alias)
    }

    /// returns the subject in the following format:
    /// {is_alias}:{user_id}:{realm} 1234567890abcdef1234567890abcdef12345678:example.com
    ///
    /// #example
    /// ```
    /// use token_helper::user::UserData;
    ///
    /// let user_data = UserData::new("1234567890abcdef1234567890abcdef12345678".to_string(), "example.com".to_string(), false).unwrap();
    /// assert_eq!(user_data.get_subject(), "=:1234567890abcdef1234567890abcdef12345678:example.com");
    /// ```
    pub fn get_subject(&self) -> String {
        format!(
            "{}:{}:{}",
            if self.is_alias { "~" } else { "=" },
            self.user_id,
            self.realm,
        )
    }
}
