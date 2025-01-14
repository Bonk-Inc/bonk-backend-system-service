use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

// Creates a new error response with a 500 status code
pub fn internal_error(err: String) -> ErrorResponse {
    ErrorResponse {
        status: "fail",
        message: err,
        code: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// Creates a new error response with a 404 status code
pub fn not_found_error(err: String) -> ErrorResponse {
    ErrorResponse {
        status: "fail",
        message: err,
        code: StatusCode::NOT_FOUND,
    }
}

// Creates a new error response with a 401 status code
pub fn unauthorized_error(err: String) -> ErrorResponse {
    ErrorResponse {
        status: "fail",
        message: err,
        code: StatusCode::UNAUTHORIZED,
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
    #[serde(skip)]
    pub code: StatusCode,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.code, Json(self)).into_response()
    }
}
