// @generated automatically by Diesel CLI.

diesel::table! {
    user_passwords (relation_id) {
        relation_id -> Int4,
        user_id -> Varchar,
        username -> Varchar,
        realm -> Nullable<Varchar>,
        password_hash -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        is_alias -> Bool,
        alias_of -> Nullable<Varchar>,
    }
}

diesel::joinable!(user_passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_passwords,
    users,
);
