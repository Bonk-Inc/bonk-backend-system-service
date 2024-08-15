use actix_web::{web, Scope, middleware::Compat};

use crate::middleware::auth_middleware::Authentication;

pub mod api;

pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(api::healthcheck)
        .service(api::score_scope())
        .service(
            api::game_scope()
                .wrap(Compat::new(Authentication))
        )
        .service(
            api::level_scope()
                .wrap(Compat::new(Authentication))
        )
        .service(
            api::stats_scope()
                .wrap(Compat::new(Authentication))
        )
}