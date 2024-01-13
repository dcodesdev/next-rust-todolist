pub mod handlers;

use axum::{routing::get, Router};

pub fn todos_routes() -> Router {
    Router::new()
        // [TODO]: Implement todos routes
        .route("/", get(handlers::get_todos))
    // .route("/:id", post().put().delete())
}
