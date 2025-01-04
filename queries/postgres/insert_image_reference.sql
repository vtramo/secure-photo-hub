INSERT INTO images ( id, owner_user_id, visibility, url, file_size, format )
VALUES ( $1, $2, $3, $4, $5, $6 )
RETURNING id, owner_user_id, visibility AS "visibility!: _", url, file_size AS "size!", format AS "format: _", created_at AS "created_at!"
