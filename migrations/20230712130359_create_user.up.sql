CREATE TABLE IF NOT EXISTS "user"
(
    "user_id" uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    "username" VARCHAR(50) NOT NULL,
    "password" VARCHAR(128) NOT NULL,
    "email" VARCHAR(128) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);