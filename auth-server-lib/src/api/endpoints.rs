use crate::{
    api::{errors::*, model::UserCredentials},
    util::{
        database::{
            connection::get_database_connection,
            user_email::{
                get_user_credentials,
                register_new_user_email_password as reg_new_user_email_password,
            },
        },
        security::password_hasher::{argon2::Argon2Hasher, PasswordHasher},
    },
};

pub fn login(credentials: UserCredentials) -> Result<String, ErrorDetails> {
    let connection = &mut get_database_connection()?;
    let user = get_user_credentials(connection, credentials.email)?;

    match <Argon2Hasher as PasswordHasher>::verify_password(
        credentials.password.as_bytes(),
        user.password_hash.as_bytes(),
    ) {
        Ok(_) => {
            //create a new jwt token

            return Ok("".to_string());
        }
        Err(e) => {
            return Err(ERR_AUTHENTICATION_FAILED.with_internal_error(e.to_string()));
        }
    }
}

pub fn register_new_user_email_password(
    credentials: UserCredentials,
) -> Result<String, ErrorDetails> {
    let connection = &mut get_database_connection()?;
    // validate the email
    validate_email(credentials.email)?;
    // hash the password
    let password_hash =
        <Argon2Hasher as PasswordHasher>::hash_password(credentials.password.as_bytes())
            .map_err(|e| ERR_UNKNOWN_INTERNAL_ERROR.with_internal_error(e.to_string()))?;
    reg_new_user_email_password(connection, credentials.email, &password_hash)
}

pub fn validate_email(email: &str) -> Result<(), ErrorDetails> {
    // at the moment just check that there is only one @
    // also it can't be the first or last character
    let mut at_count = 0;
    for (i, c) in email.chars().enumerate() {
        if c == '@' {
            at_count += 1;
            if i == 0 || i == email.len() - 1 {
                return Err(ERR_INVALID_DATA
                    .with_internal_error("email can't start or end with @".to_string()));
            }
        }
    }
    if at_count != 1 {
        return Err(
            ERR_INVALID_DATA.with_internal_error("email must contain exactly one @".to_string())
        );
    }
    Ok(())
}
