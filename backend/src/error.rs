use axum::{http::StatusCode, Json};
use serde::Serialize;

pub fn internal_error(err: String) -> (StatusCode, Json<ErrorResponse>) {
    let error_resp = ErrorResponse {
        status: "fail",
        message: err
    };

    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_resp))
}

pub fn not_found_error(err: String) -> (StatusCode, Json<ErrorResponse>) {
    let error_resp = ErrorResponse {
        status: "fail",
        message: err
    };

    (StatusCode::NOT_FOUND, Json(error_resp))
}

pub fn unauthorized_error(err: String) -> (StatusCode, Json<ErrorResponse>) {
    let error_resp = ErrorResponse {
        status: "fail",
        message: err
    };

    (StatusCode::UNAUTHORIZED, Json(error_resp))
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}