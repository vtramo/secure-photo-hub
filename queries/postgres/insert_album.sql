INSERT INTO albums ( id, title, description, visibility, owner_user_id, cover_image_id )
VALUES ( $1, $2, $3, $4, $5, $6 )
RETURNING
    id,
    title,
    description,
    visibility AS "visibility!: _",
    owner_user_id AS "owner_user_id!",
    cover_image_id AS "cover_image_id!",
    created_at AS "created_at!"

