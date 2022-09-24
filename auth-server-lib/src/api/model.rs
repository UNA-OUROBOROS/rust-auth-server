use crate::api::errors::*;
use dboilerplate::util::configuration;
use diesel::{prelude::*, Queryable};

pub(crate) fn get_database_connection() -> Result<PgConnection, ErrorDetails> {
    let config = configuration::get_config(None, None);
    let database_url: String = config
        .extract_inner("connection_string")
        .map_err(|e| ERR_BACKEND_CONNECTION_STRING_NOT_FOUND.with_internal_error(e.to_string()))?;
    let connection = PgConnection::establish(&database_url)
        .map_err(|e| ERR_BACKEND_CONNECTION_FAILED.with_internal_error(e.to_string()))?;
    Ok(connection)
}

#[derive(Queryable)]
pub(crate) struct UserCredentials {
    pub user_id: String,
    pub user_email: String,
    pub password_hash: String,
}

#[derive(Queryable)]
pub(crate) struct Users {
    pub id: String,
}

pub(crate) fn get_user_credentials(
    connection: &mut PgConnection,
    user_email: &str,
) -> Result<UserCredentials, ErrorDetails> {
    use crate::schema::{user_emails, user_passwords};
    let query = user_emails::table
        .inner_join(user_passwords::table.on(user_emails::user_id.eq(user_passwords::user_id)))
        .filter(user_emails::email.eq(user_email))
        .select((
            user_emails::user_id,
            user_emails::email,
            user_passwords::password_hash,
        ));
    let result = query.get_result::<UserCredentials>(connection);
    match result {
        Ok(user) => Ok(user),
        Err(e) => Err(ERR_DATABASE_VALUE_NOT_FOUND.with_internal_error(e.to_string())),
    }
}
