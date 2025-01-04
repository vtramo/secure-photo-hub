SELECT
    images.id AS "id!",
    images.owner_user_id AS "owner_user_id!",
    images.url AS "url!",
    images.file_size AS "size!",
    images.visibility AS "visibility!: _",
    images.format AS "format!: _",
    images.created_at AS "created_at!"
FROM
    images
WHERE
    images.id = $1;