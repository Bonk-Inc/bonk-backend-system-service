import type { Level } from "./Level"
import type { User } from "./User"

export interface Score {
    id: string,
    score: number,
    is_hidden: boolean,
    level: Level,
    user: User,
    username: string,
    created_at: string,
    updated_at: string
}

export interface ScoreDTO {
    username: string,
    score: number,
    is_hidden: boolean,
    level_id: string,
    user_id: string
}