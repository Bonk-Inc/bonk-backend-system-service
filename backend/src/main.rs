use std::{
    env, 
    error::Error,
    fs::OpenOptions, 
    io::{self, ErrorKind, Write},
    net::SocketAddr, 
    sync::{Arc, RwLock}, 
    time::Duration
};

use axum::{http::Method, Router};
use config::db::{init_db_pool, run_migration, Pool};
use controller::api::{
    game::{GameApi, GameResponseBody, GamesResponseBody},
    level::{LevelApi, LevelResponseBody, LevelsResponseBody},
    score::{ScoreApi, ScoreResponseBody, ScoresResponseBody},
};
#[cfg(debug_assertions)]
use dotenvy::dotenv;
use log::{error, info};
use models::{
    game::{Game, GameDTO},
    level::{Level, LevelDTO},
    score::{Score, ScoreDTO},
};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_DISPOSITION, CONTENT_TYPE};
use tokio::{net::TcpListener, spawn, time::interval};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};
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
    nest(
        (path = "/game", api = GameApi),
        (path = "/level", api = LevelApi),
        (path = "/score", api = ScoreApi)
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

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().expect(".env file not found");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let app_url = format!("{}:{}", app_host, app_port);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = init_db_pool(&db_url);
    run_migration(&mut db_pool.get().unwrap());

    let state = SharedState::new(RwLock::new(AppState { db: db_pool }));

    let front_end = ServeDir::new("./dist/")
        .append_index_html_on_directories(true);

    let app = Router::new()
        .nest("/api", controller::api_routes())
        .nest_service("/", front_end)
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(setup_cors())
        .with_state(state);

    let addr: SocketAddr = app_url.parse().expect("Cannot parse app url to socket");
    let listener = TcpListener::bind(addr).await.unwrap();

    spawn(refresh_jwk());
    axum::serve(listener, app).await.unwrap();
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

fn setup_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .expose_headers([CONTENT_DISPOSITION])
        .max_age(Duration::from_secs(3600))
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
pub struct AppState {
    db: Pool
}