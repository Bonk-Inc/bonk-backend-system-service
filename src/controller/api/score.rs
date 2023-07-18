use actix_web::{get, HttpResponse, web};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Score")
}