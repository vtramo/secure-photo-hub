UPDATE photos
SET album_id = $2
WHERE 
    photos.id = $1
AND EXISTS (
    SELECT 1
    FROM albums
    WHERE albums.id = $2
);
