-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "user" 
(
    "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" VARCHAR(50) NOT NULL,
    "game_id" uuid NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP
);

SELECT diesel_manage_updated_at('user');