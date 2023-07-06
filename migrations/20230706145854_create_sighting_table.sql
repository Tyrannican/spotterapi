-- Add migration script here
CREATE TABLE sightings(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    user_id TEXT NOT NULL,
    lat FLOAT NOT NULL,
    lng FLOAT NOT NULL,
    object TEXT NOT NULL,
    description TEXT,
    created_at timestamp NOT NULL
);
