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
    score (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[sql_name = "score"]
        highscore -> Int4,
        is_hidden -> Bool,
        game_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (user_id) {
        user_id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(score -> game (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    score,
    user,
);
