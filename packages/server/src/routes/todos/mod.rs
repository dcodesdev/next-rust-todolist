pub mod handlers;

use axum::{routing::get, Router};

pub fn totos_routes() -> Router {
    Router::new()
        // [TODO]: Implement todos routes
        .route("/", get(handlers::get_todos))
    // .route("/:id", post().put().delete())
}
