-- Your SQL goes here
-- an user consists of an user id 
-- the user id is at least 36 characters long
-- case insensitive and alphanumeric
-- extra fields like disabled or verified should be handled by the application
CREATE TABLE users (
    id VARCHAR(36) NOT NULL,
    PRIMARY KEY (id)
);

-- contains the user id(from users table)
-- their realm(dns name witouth '-')
-- and their hashed password
-- if the realm is null is an internal user from the app
-- if not this is an email user
create table user_passwords (
    user_id VARCHAR(36) NOT NULL,
    realm VARCHAR(255) NULL,
    password_hash VARCHAR(255) NOT NULL,
    PRIMARY KEY (user_id, realm),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);