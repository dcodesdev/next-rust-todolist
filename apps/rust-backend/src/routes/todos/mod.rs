pub mod handlers;

use crate::middlewares;
use axum::{
    middleware::from_fn,
    routing::{get, put},
    Router,
};

// [TODO]: Implement todos routes
pub fn todos_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_todos).post(handlers::create_todo))
        .route(
            "/:id",
            put(handlers::update_todo).delete(handlers::delete_todo),
        )
        .layer(from_fn(middlewares::auth))
}
