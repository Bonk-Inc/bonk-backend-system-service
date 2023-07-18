use std::env;

use actix_web::{middleware::{Logger, self}, App, HttpServer, web};
use diesel::{PgConnection, r2d2::ConnectionManager};
use dotenvy::dotenv;

pub mod controller;
pub mod models;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let port = match env::var("APP_PORT") {
        Ok(val) => val,
        Err(_) => "127.0.0.1".to_owned()
    };

    let db_pool = initialize_db_pool();

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

fn initialize_db_pool() -> DbPool {
    let database_url = env::var("DATAbASE_URL").expect("Database Url must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Database URL should link to the external database server")
}