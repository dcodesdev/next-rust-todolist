pub mod handlers;

use axum::routing::{get, put};
use axum::{middleware::from_fn, Router};

pub fn todo_lists_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(handlers::get_todo_lists).post(handlers::create_todo_list),
        )
        .route("/:id", put(handlers::update_todo_list))
        .layer(from_fn(crate::middlewares::auth))
}
