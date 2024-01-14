use axum::middleware::from_fn;
use axum::routing::{get, post};
use axum::Router;

use crate::middlewares;

pub mod handlers;

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_current_user))
        .layer(from_fn(middlewares::auth))
        .route("/", post(handlers::register_user))
        .route("/login", post(handlers::login_user))
}
