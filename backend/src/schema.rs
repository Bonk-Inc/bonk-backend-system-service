// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    level (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        game_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    score (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[sql_name = "score"]
        highscore -> Int4,
        is_hidden -> Bool,
        game_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        level_id -> Nullable<Uuid>,
    }
}

diesel::joinable!(level -> game (game_id));
diesel::joinable!(score -> game (game_id));
diesel::joinable!(score -> level (level_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    level,
    score,
);
