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
pub(crate) struct UserPasswords {
    pub user_id: String,
    pub username: String,
    pub realm: String,
    pub password: String,
}

#[derive(Queryable)]
pub(crate) struct Users {
    pub id: String,
    pub is_alias: bool,
    pub alias_of: Option<String>,
}
