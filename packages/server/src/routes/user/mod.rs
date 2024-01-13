use axum::Router;

pub mod handlers;

pub fn user_routes() -> Router {
    Router::new()
    // [TODO]: Implement user routes
    // .route( "/", get().post())
    // .route("/:id", post().put().delete())
}
