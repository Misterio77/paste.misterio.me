BEGIN;
CREATE TABLE users (
    username varchar NOT NULL,
    email varchar NOT NULL,
    password varchar NOT NULL,
    totp varchar,

    CONSTRAINT users_pkey PRIMARY KEY (username),
    CONSTRAINT users_email_un UNIQUE (email)
);
CREATE TABLE sessions (
    id uuid NOT NULL,
    creator varchar NOT NULL,
    source inet NOT NULL,
    creation timestamptz NOT NULL,

    CONSTRAINT sessions_pkey PRIMARY KEY (id),
    CONSTRAINT sessions_creator_fkey FOREIGN KEY (creator)
        REFERENCES users (username) ON DELETE CASCADE
);
COMMIT;
