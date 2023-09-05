use actix_web::{web, Scope};

pub mod auth;
pub mod api;

pub fn auth_scope() -> Scope {
    web::scope("/auth")
        .service(auth::login)
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(api::score_scope())
        .service(api::game_scope())
}