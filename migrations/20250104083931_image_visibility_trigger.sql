CREATE OR REPLACE FUNCTION update_image_visibility()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.visibility IS DISTINCT FROM OLD.visibility THEN
        UPDATE images
        SET visibility = NEW.visibility
        WHERE images.id = NEW.image_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_image_visibility_trigger
AFTER UPDATE OF visibility ON photos
FOR EACH ROW
EXECUTE FUNCTION update_image_visibility();
