use actix_web::{Scope, web};

pub mod score;

pub fn score_scope() -> Scope {
    web::scope("/score")
        .service(score::index)
}