use axum::http::StatusCode;

pub fn internal_error(err: String) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err)
}

pub fn not_found_error(err: String) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, err)
}