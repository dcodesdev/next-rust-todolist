pub mod handlers;

use axum::{middleware::from_fn, routing::get, Router};

pub fn todo_lists_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(handlers::get_todo_lists).post(handlers::create_todo_list),
        )
        .layer(from_fn(crate::middlewares::auth))
}
