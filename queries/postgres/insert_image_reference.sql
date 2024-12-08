INSERT INTO images ( id, url, file_size, format )
VALUES ( $1, $2, $3, $4 )
RETURNING id, url, file_size AS "size!", format AS "format: _", created_at AS "created_at!"
