// @generated automatically by Diesel CLI.

diesel::table! {
    user_passwords (user_id, realm) {
        user_id -> Varchar,
        realm -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
    }
}

diesel::joinable!(user_passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_passwords,
    users,
);
