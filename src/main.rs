use std::env;

use actix_web::{middleware::{Logger, self}, App, HttpServer, web};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

mod controller;
mod entity;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let port = match env::var("APP_PORT") {
        Ok(val) => val,
        Err(_) => "127.0.0.1".to_owned()
    };

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env::var("DATABASE_URL").expect("Database URL must be set").as_str())
        .await
        .expect("Database connection could not be made");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Always))
            .wrap(Logger::default())
            .service(controller::api_scope())
    })
    .bind((port, 8080))?
    .run()
    .await
}
