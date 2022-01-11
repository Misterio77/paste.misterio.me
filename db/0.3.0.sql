BEGIN;
CREATE TABLE pastes (
    id uuid NOT NULL,
    creator varchar NOT NULL,
    creation timestamptz NOT NULL,
    content varchar NOT NULL,
    name varchar,
    language varchar,

    CONSTRAINT pastes_pkey PRIMARY KEY (id),
    CONSTRAINT pastes_creator_fkey FOREIGN KEY (creator)
        REFERENCES users (username) ON DELETE CASCADE
);
COMMIT;
