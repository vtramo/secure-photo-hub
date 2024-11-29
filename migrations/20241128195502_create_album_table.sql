-- Add migration script here
CREATE TYPE visibility as ENUM ('Public', 'Private');

CREATE TABLE albums(
    id uuid NOT NULL,
    PRIMARY KEY(id),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    visibility visibility NOT NULL,
    owner_user_id uuid NOT NULL,
    cover_image_id uuid NOT NULL
        REFERENCES images(id)
        ON DELETE CASCADE,
    created_at timestamptz DEFAULT NOW()
);