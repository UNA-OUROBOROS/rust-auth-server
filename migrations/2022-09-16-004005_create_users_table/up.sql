-- Your SQL goes here
-- an user consists of an user id 
-- the user id is at least 36 characters long
-- case insensitive and alphanumeric
-- extra fields like disabled or verified should be handled by the application
-- the is_alias field is used to indicate if the user is an alias of another user
-- for example if the user was mapped to another user
CREATE TABLE users (
    id VARCHAR(36) NOT NULL,
    is_alias BOOLEAN NOT NULL DEFAULT FALSE,
    alias_of VARCHAR(36) NULL,
    PRIMARY KEY (id)
);

-- contains the user id(from users table)
-- the username (max 64 characters)
-- their realm(dns name witouth '-')
-- and their hashed password
-- if the realm is null is an internal user from the app
-- if not this is an email user
create table user_passwords (
    user_id VARCHAR(36) NOT NULL,
    username VARCHAR(64) NOT NULL,
    realm VARCHAR(255) NULL,
    password_hash VARCHAR(255) NOT NULL,
    PRIMARY KEY (user_id, realm),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);