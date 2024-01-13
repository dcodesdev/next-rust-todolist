use axum::routing::post;
use axum::Router;

pub mod handlers;

pub fn user_routes() -> Router {
    Router::new()
        .route("/", post(handlers::register_user))
        .route("/login", post(handlers::login_user))
}
