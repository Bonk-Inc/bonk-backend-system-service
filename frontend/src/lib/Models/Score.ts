export interface Score {
    id: string,
    username: string,
    score: number,
    is_hidden: boolean,
    level_id: string,
    game_id: string,
    created_at: string,
    updated_at: string
}

export interface ScoreDTO {
    username: string,
    score: number,
    is_hidden: boolean,
    level_id: string
}