// @generated automatically by Diesel CLI.

diesel::table! {
    user_emails (user_id) {
        user_id -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    user_passwords (user_id) {
        user_id -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Varchar,
    }
}

diesel::joinable!(user_emails -> users (user_id));
diesel::joinable!(user_passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_emails,
    user_passwords,
    users,
);
