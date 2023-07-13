use actix_web::{get, HttpResponse, web};
use sqlx::{Pool, Postgres};

#[get("/")]
pub async fn index(pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    HttpResponse::Ok().body("Score")
}