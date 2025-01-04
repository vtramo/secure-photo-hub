SELECT
    albums.id AS "album_id!",
    albums.title AS "title!",
    albums.description AS "description!",
    albums.visibility AS "visibility!: _",
    albums.owner_user_id AS "album_owner_user_id!",
    albums.cover_image_id AS "image_reference_id!",
    albums.created_at AS "album_created_at!",

    images.id AS "image_id!",
    images.owner_user_id AS "image_owner_user_id!",
    images.url AS "url!",
    images.file_size AS "size!",
    images.format AS "format!: _",
    images.created_at AS "image_created_at!"
FROM
    albums
LEFT JOIN
    images ON albums.cover_image_id = images.id;
