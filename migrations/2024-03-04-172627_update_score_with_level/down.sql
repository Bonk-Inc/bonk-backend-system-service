ALTER TABLE "score"
    DROP COLUMN "level_id" CASCADE;

ALTER TABLE "score"
    ALTER COLUMN "game_id" SET NOT NULL;