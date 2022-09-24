use crate::api::errors::*;
use dboilerplate::util::configuration;
use diesel::prelude::*;

pub(crate) fn get_database_connection() -> Result<PgConnection, ErrorDetails> {
    let config = configuration::get_config(None, None);
    let database_url: String = config
        .extract_inner("DATABASE_URL")
        .map_err(|e| ERR_BACKEND_CONNECTION_STRING_NOT_FOUND.with_internal_error(e.to_string()))?;
    let connection = PgConnection::establish(&database_url)
        .map_err(|e| ERR_BACKEND_CONNECTION_FAILED.with_internal_error(e.to_string()))?;
    Ok(connection)
}
