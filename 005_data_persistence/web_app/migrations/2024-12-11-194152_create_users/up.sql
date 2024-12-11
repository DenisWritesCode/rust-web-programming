-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    unique_id VARCHAR NOT NULL
);

-- Insert a placeholder user
INSERT INTO users(username, email, password, unique_id) VALUES ('placeholder', 'placeholder@email', 'placeholder_password', 'placeholder_unique_id');

-- Add a value into to_do table.
ALTER TABLE to_do ADD user_id integer default 1 CONSTRAINT user_id REFERENCES users NOT NULL;