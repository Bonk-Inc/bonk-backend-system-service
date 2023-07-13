use actix_web::{web, Scope};

pub mod api;

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(api::score_scope())
}