BEGIN;
CREATE TABLE api_keys (
    id uuid NOT NULL,
    name varchar,
    creator varchar NOT NULL,
    creation timestamptz NOT NULL,
    key varchar NOT NULL,

    CONSTRAINT api_keys_pkey PRIMARY KEY (id),
    CONSTRAINT api_keys_creator_fkey FOREIGN KEY (creator)
        REFERENCES users (username) ON DELETE CASCADE
);
COMMIT;
