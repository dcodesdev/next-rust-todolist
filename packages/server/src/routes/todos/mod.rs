pub mod handlers;

use crate::middlewares;
use axum::{middleware::from_fn, routing::get, Router};

// [TODO]: Implement todos routes
pub fn todos_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_todos))
        .layer(from_fn(middlewares::auth))
}
