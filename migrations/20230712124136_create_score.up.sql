CREATE TABLE IF NOT EXISTS score
(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL,
    game_id uuid NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    CONSTRAINT fk_game_score
        FOREIGN KEY (game_id)
            REFERENCES game (game_id)
            ON DELETE CASCADE
);