use crate::{error::ApiError, routes};
use axum::{http::StatusCode, routing, Extension, Json, Router};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn app() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("could not connect to database_url");

    println!("Connected to database");

    let state = ServiceBuilder::new().layer(Extension(AppState { db }));

    // [TODO]: Change this to only allow the frontend domain
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app: Router = Router::new()
        .fallback_service(routing::any(|| async {
            Err::<ApiError, _>(ApiError::NotFound)
        }))
        .nest("/todos", routes::todos::todos_routes())
        .nest("/user", routes::user::user_routes())
        .nest("/lists", routes::todo_lists::todo_lists_routes())
        .route(
            "/_health",
            routing::get(|| async { (StatusCode::OK, "OK") }),
        )
        .route(
            "/",
            routing::get(|| async {
                (
                    StatusCode::OK,
                    Json(json!({"message": "Axum server is running!"})),
                )
            }),
        )
        .layer(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on http://localhost:5000");

    axum::serve(listener, app).await.unwrap();
}
