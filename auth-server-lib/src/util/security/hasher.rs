pub(crate) trait PasswordHasher {
    /// Hashes the password and creates a salt for it
    fn hash_password(password: &[u8]) -> Result<String, String>;
    /// Verifies the password against the encoded hash
    fn verify_password(password: &[u8], encoded: &[u8]) -> Result<(), String>;
}
