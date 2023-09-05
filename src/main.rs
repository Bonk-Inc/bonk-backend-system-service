use std::env;

use actix_web::{
    middleware::Logger,
    web, App, HttpServer,
};
use config::{db::{init_db_pool, run_migration}, oauth2::OAuth2Client};
use dotenvy::dotenv;

pub mod config;
pub mod controller;
pub mod error;
pub mod models;
pub mod schema;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    

    let app_host = env::var("APP_HOST").expect("APP_PORT must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let app_url = format!("{}:{}", app_host, app_port);

    let oauth2_client = OAuth2Client::new();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = init_db_pool(&db_url);
    run_migration(&mut db_pool.get().unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(oauth2_client.clone()))
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Always,
            ))
            .wrap(Logger::default())
            .service(controller::auth_scope())
            .service(controller::api_scope())
    })
    .bind(&app_url)?
    .run()
    .await
}