INSERT INTO photos ( id, title, description, visibility, owner_user_id, tags, category, album_id, image_id )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9 )
RETURNING id,
          title,
          description,
          visibility AS "visibility!: _",
          owner_user_id AS "owner_user_id!",
          tags AS "tags!: Vec<String>",
          category AS "category!: _",
          album_id AS "album_id?",
          image_id,
          is_deleted,
          created_at AS "created_at!"