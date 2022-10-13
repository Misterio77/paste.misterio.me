BEGIN;
CREATE TABLE pastes (
    id uuid NOT NULL,
    creator varchar NOT NULL,
    creation timestamptz NOT NULL,
    content varchar NOT NULL,
    unlisted boolean NOT NULL,
    title varchar,
    description varchar,

    CONSTRAINT pastes_pkey PRIMARY KEY (id),
    CONSTRAINT pastes_creator_fkey FOREIGN KEY (creator)
        REFERENCES users (username) ON DELETE CASCADE
);
COMMIT;
