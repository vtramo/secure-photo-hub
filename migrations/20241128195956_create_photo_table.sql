-- Add migration script here
CREATE TABLE photos(
    id uuid NOT NULL,
    PRIMARY KEY(id),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    visibility visibility NOT NULL,
    owner_user_id uuid NOT NULL,
    tags TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT '',
    album_id uuid NOT NULL
        REFERENCES albums(id)
        ON DELETE CASCADE,
    image_id uuid NOT NULL
        REFERENCES images(id)
        ON DELETE CASCADE,
    is_deleted boolean NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL
);