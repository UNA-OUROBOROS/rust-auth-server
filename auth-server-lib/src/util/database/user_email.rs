use crate::api::errors::*;
use crate::schema::{user_emails, user_passwords, users};
use diesel::{prelude::*, Queryable};
use rand::Rng;

#[derive(Queryable)]
pub(crate) struct UserCredentials {
    pub user_id: String,
    pub user_email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct UserIdentity<'r> {
    pub user_id: &'r str,
}

#[derive(Insertable)]
#[diesel(table_name = user_passwords)]
struct UserPassword<'r> {
    pub user_id: &'r str,
    pub password_hash: &'r str,
}

#[derive(Insertable)]
#[diesel(table_name = user_emails)]
struct UserEmail<'r> {
    pub user_id: &'r str,
    pub email: &'r str,
}

pub(crate) fn get_user_credentials(
    connection: &mut PgConnection,
    user_email: &str,
) -> Result<UserCredentials, ErrorDetails> {
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
        Err(e) => Err(ERR_DATABASE_RESOURCE_NOT_FOUND.with_internal_error(e.to_string())),
    }
}

pub(crate) fn register_new_user_email_password(
    connection: &mut PgConnection,
    user_email: &str,
    password_hash: &str,
) -> Result<String, ErrorDetails> {
    // the user is a random 36 character string(case insensitive, alphanumeric)
    let new_user_id: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(36)
        .map(char::from)
        .collect();
    let transaction_result =
        connection.transaction::<_, diesel::result::Error, _>(|connection: &mut PgConnection| {
            diesel::insert_into(users::table)
                .values(UserIdentity {
                    user_id: &new_user_id,
                })
                .execute(connection)?;
            diesel::insert_into(user_emails::table)
                .values(UserEmail {
                    user_id: &new_user_id,
                    email: user_email,
                })
                .execute(connection)?;
            diesel::insert_into(user_passwords::table)
                .values(UserPassword {
                    user_id: &new_user_id,
                    password_hash,
                })
                .execute(connection)?;
            Ok(())
        });
    match transaction_result {
        Ok(_) => Ok(new_user_id),
        Err(e) => Err(ERR_DATABASE_TRANSACTION_FAILED.with_internal_error(e.to_string())),
    }
}
