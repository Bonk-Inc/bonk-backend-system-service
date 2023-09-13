use actix_web::{web, Scope, middleware::Compat};

use crate::middleware::auth_middleware::Authentication;

pub mod auth;
pub mod api;

pub fn auth_scope() -> Scope {
    web::scope("/auth")
        .service(auth::authorize)
        .service(auth::login)
        .service(auth::refresh)
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(api::score_scope())
        .service(
            api::game_scope()
                .wrap(Compat::new(Authentication))
        )
}