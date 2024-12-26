SELECT
    images.id AS "id!",
    images.url AS "url!",
    images.file_size AS "size!",
    images.format AS "format!: _",
    images.created_at AS "created_at!"
FROM
    images
WHERE
    images.id = $1;