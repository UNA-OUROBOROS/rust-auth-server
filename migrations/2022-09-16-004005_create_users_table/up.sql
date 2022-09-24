-- Your SQL goes here

CREATE TABLE users (
    user_id VARCHAR(36) NOT NULL,
    PRIMARY KEY (user_id)
);

create table user_passwords(
    user_id varchar(36) not null,
    password_hash VARCHAR(255) NOT NULL,
    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

create table user_emails (
    user_id varchar(36) not null,
    email varchar(254) not null unique,
    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);