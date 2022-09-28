-- Your SQL goes here

-- a list of registered applications in the service 
create table login_applications (
    id uuid not null default gen_random_uuid(),
    friendly_name varchar(255) not null,
    audience varchar(255) not null,
    callback_url varchar(255) not null,
    primary key (id)
);