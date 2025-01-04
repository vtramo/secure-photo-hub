ALTER TABLE images
ADD owner_user_id uuid NOT NULL,
ADD visibility visibility NOT NULL;