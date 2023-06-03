-- Your SQL goes here
CREATE TABLE users
(
    id             SERIAL PRIMARY KEY NOT NULL UNIQUE,
    username       VARCHAR(100) NOT NULL UNIQUE,
    mobile         VARCHAR(15) NOT NULL UNIQUE,
    email          VARCHAR(50) NOT NULL UNIQUE,
    password_hash  VARCHAR(32) NOT NULL,
    created_at     Timestamp NOT NULL,
    login_at       Timestamp NOT NULL
)