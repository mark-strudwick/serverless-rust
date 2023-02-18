use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    BadRequest,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred",
            ),
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
        };

        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
