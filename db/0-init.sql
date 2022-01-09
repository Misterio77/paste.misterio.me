BEGIN;
CREATE TABLE users (
    username varchar NOT NULL,
    email varchar NOT NULL,
    password varchar NOT NULL,
    totp varchar,
    CONSTRAINT users_pkey PRIMARY KEY (username)
);
CREATE TABLE sessions (
    token varchar NOT NULL,
    creator varchar NOT NULL,
    source inet NOT NULL,
    creation timestamptz NOT NULL,
    CONSTRAINT sessions_pkey PRIMARY KEY (creator, creation),
    CONSTRAINT sessions_creator_fkey FOREIGN KEY (creator)
        REFERENCES users (username) ON DELETE CASCADE
);
COMMIT;
