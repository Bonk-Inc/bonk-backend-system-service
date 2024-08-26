use std::{
    env,
    error::Error,
    fs::OpenOptions,
    io::{self, ErrorKind, Write},
    process,
    time::Duration,
};

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Logger,
    rt::{spawn, time::interval},
    web, App, HttpServer,
};
use config::db::{init_db_pool, run_migration};
use controller::api::{
    game::{self, GameResponseBody, GamesResponseBody},
    level::{self, LevelResponseBody, LevelsResponseBody},
    score::{self, ScoreResponseBody, ScoresResponseBody},
};
#[cfg(debug_assertions)]
use dotenvy::dotenv;
use log::{error, info};
use models::{
    game::{Game, GameDTO},
    level::{Level, LevelDTO},
    score::{Score, ScoreDTO},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod config;
pub mod controller;
pub mod error;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod service;

pub const JWK_FILE_PATH: &str = "data/jwk.json";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Bonk inc Backend",
        description = "My Api description"
    ),
    servers((url = "https://babs.bonk.group/api")),
    paths(
        game::index, game::show, game::store, game::update, game::destroy,
        level::index, level::game_levels, level::store, level::update, level::destroy,
        score::index, score::show, score::game_scores, score::level_scores,
        score::store, score::update, score::destroy,
    ),
    tags(
        (name = "Game", description = "Game management endpoints."),
        (name = "Level", description = " "),
        (name = "Score", description = "Score management endpoints.")
    ),
    components(
        schemas(
            Game, GameDTO, GameResponseBody, GamesResponseBody, 
            Level, LevelDTO, LevelResponseBody, LevelsResponseBody,
            Score, ScoreDTO, ScoreResponseBody, ScoresResponseBody
        )
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().expect(".env file not found");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let app_url = format!("{}:{}", app_host, app_port);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = init_db_pool(&db_url);
    run_migration(&mut db_pool.get().unwrap());

    let jwks_file = fetch_and_save_jwks().await;
    if jwks_file.is_err() {
        error!("Cannot save fetched jwks token, stopping program");
        process::exit(0);
    }

    spawn(refresh_jwk());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .wrap(setup_cors())
            .service(controller::api_scope())
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
        // .service(Files::new("/", "./dist/").index_file("index.html"))
        // .default_service(|req: ServiceRequest| {
        //     let (http_req, _payload) = req.into_parts();
        //     async {
        //         let response = NamedFile::open("./dist/index.html")?.into_response(&http_req);
        //         Ok(ServiceResponse::new(http_req, response))
        //     }
        // })
    })
    .bind(&app_url)?
    .run()
    .await
}

async fn fetch_and_save_jwks() -> Result<(), Box<dyn Error>> {
    let jwsk_url = env::var("OAUTH_JWKS_URL").expect("OAUTH_JWKS_URL must be set");
    let tokens = reqwest::get(&jwsk_url).await?.text().await;

    if tokens.is_err() {
        error!("Could not fetch token from {}", jwsk_url);
        return Err(Box::new(io::Error::new(
            ErrorKind::Other,
            "Could not fetch JWKS token",
        )));
    }

    info!("JWKS token fetched, saving to file");
    let file_options = OpenOptions::new()
        .write(true)
        .create(true)
        .open(JWK_FILE_PATH);

    if file_options.is_ok() {
        info!("Write new JWKS token to file");
        let _ = file_options.unwrap().write_all(tokens.unwrap().as_bytes());
    } else {
        error!(
            "Cannot write JWKS token to file, reason {}",
            file_options.err().unwrap()
        )
    }

    Ok(())
}

async fn refresh_jwk() {
    let mut delay = interval(Duration::from_secs(604_800));

    loop {
        delay.tick().await;

        info!("Refreshing JWKS token");
        let _ = fetch_and_save_jwks().await;
    }
}

fn setup_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
        ])
        .allowed_header(actix_web::http::header::CONTENT_TYPE)
        .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
        .supports_credentials()
        .max_age(3600)
}
