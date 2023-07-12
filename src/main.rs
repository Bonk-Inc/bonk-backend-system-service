use actix_web::{HttpServer, App, middleware::Logger};

mod controller;
mod entity;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(controller::score::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
