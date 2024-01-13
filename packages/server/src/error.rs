use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
#[allow(unused)]
pub enum ApiError {
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            ApiError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };

        (status_code, Json(json!({ "message": message }))).into_response()
    }
}
