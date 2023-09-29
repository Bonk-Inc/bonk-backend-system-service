use std::{
    error::Error,
    fs::OpenOptions,
    io::Write, 
    time::Duration,
    env,
    process
};

use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger,
    rt::{time::interval, spawn},
    App, 
    HttpServer, 
    web, dev::{ServiceRequest, ServiceResponse}
};
use config::{
    db::{init_db_pool, run_migration},
    oauth2::OAuth2Client
};
use dotenvy::dotenv;
use log::{info, error};

pub mod config;
pub mod controller;
pub mod error;
pub mod middleware;
pub mod models;
pub mod service;

pub const JWK_FILE_PATH: &str = "jwk.json";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let app_url = format!("{}:{}", app_host, app_port);

    let oauth2_client = OAuth2Client::new().await;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = init_db_pool(&db_url);
    run_migration(&mut db_pool.get().unwrap());

    let jwk_file = fetch_and_save_jwk().await;
    if jwk_file.is_err() {
        error!("Cannot save fetched jwk token, stopping program");
        process::exit(0);
    }

    spawn(refresh_jwk());

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
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .default_service(|req: ServiceRequest| {
                let (http_req, _payload) = req.into_parts();
                async {
                    let response = NamedFile::open("./dist/index.html")?.into_response(&http_req);
                    Ok(ServiceResponse::new(http_req, response))
                }
            })
    })
    .bind(&app_url)?
    .run()
    .await
}

async fn fetch_and_save_jwk() -> Result<(), Box<dyn Error>> {
    let jwsk_url = env::var("OAUTH_JWSK_URL").expect("OAUTH_JWSK_URL must be set");
    let tokens = reqwest::get(jwsk_url)
        .await?
        .text()
        .await?;

    let file_options = OpenOptions::new()
        .write(true)
        .create(true)
        .open(JWK_FILE_PATH);

    if file_options.is_ok() {
        info!("Write new JWK token to file");
        let _ = file_options.unwrap().write_all(tokens.as_bytes());
    }

   Ok(()) 
}

async fn refresh_jwk() {
    let mut delay = interval(Duration::from_secs(604_800));

    loop {
        delay.tick().await;
    
        info!("Refreshing JWK token");
        let _ = fetch_and_save_jwk().await;
    }
}