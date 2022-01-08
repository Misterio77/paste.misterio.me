BEGIN;

CREATE TABLE users (
    username varchar NOT NULL PRIMARY KEY,
    -- Required for registration
    email varchar NOT NULL,
    password VARCHAR NOT NULL,
    -- Optional security
    totp varchar,
    -- Optional profile fields
    url varchar,
    location varchar,
    bio varchar
);

CREATE TABLE sessions (
    id int NOT NULL GENERATED ALWAYS AS IDENTITY,
    creator VARCHAR NOT NULL REFERENCES users (username) ON DELETE CASCADE,
    token varchar NOT NULL,
    source inet NOT NULL,
    creation timestamptz NOT NULL
);

COMMIT;
