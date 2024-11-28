-- Add migration script here
CREATE TYPE image_format as ENUM (
    'Png',
    'Jpeg',
    'Gif',
    'WebP',
    'Pnm',
    'Tiff',
    'Tga',
    'Dds',
    'Bmp',
    'Ico',
    'Hdr',
    'OpenExr',
    'Farbfeld',
    'Avif',
    'Qoi',
    'Pcx'
);

CREATE TABLE images(
    id uuid NOT NULL,
    PRIMARY KEY(id),
    url TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    format image_format NOT NULL,
    created_at timestamptz NOT NULL
);