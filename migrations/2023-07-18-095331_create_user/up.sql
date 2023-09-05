CREATE TABLE IF NOT EXISTS "user"
(
    "user_id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "username" VARCHAR(50) NOT NULL,
    "password" VARCHAR(128) NOT NULL,
    "email" VARCHAR(128) NOT NULL UNIQUE,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);