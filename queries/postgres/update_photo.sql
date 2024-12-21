UPDATE photos
SET
    title = CASE
               WHEN $3::text IS NOT NULL THEN $3
               ELSE title
            END,
    album_id = CASE
                WHEN $2::uuid != uuid_nil() THEN $2
                ELSE album_id
               END
FROM images
WHERE photos.image_id = images.id
AND photos.id = $1
AND (
    $2::uuid = uuid_nil() OR EXISTS (
        SELECT 1
        FROM albums
        WHERE albums.id = $2
    )
) RETURNING
    photos.id AS "photo_id!",
    photos.title AS "title!",
    photos.description AS "description!",
    photos.visibility AS "visibility!: _",
    photos.owner_user_id AS "owner_user_id!",
    photos.tags AS "tags!: Vec<String>",
    photos.category AS "category!: _",
    photos.album_id AS "album_id?",
    photos.image_id AS "image_reference_id!",
    photos.is_deleted AS "is_deleted!",
    photos.created_at AS "photo_created_at!",

    images.id AS "image_id!",
    images.url AS "url!",
    images.file_size AS "size!",
    images.format AS "format!: _",
    images.created_at AS "image_created_at!";
