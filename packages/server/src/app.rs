use crate::routes;
use axum::{http::StatusCode, routing, Extension, Router};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct App {
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

    let state = ServiceBuilder::new().layer(Extension(App { db }));

    // [TODO]: Change this to only allow the frontend domain
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app: Router = Router::new()
        .nest("/todos", routes::todos::todos_routes())
        .nest("/user", routes::user::user_routes())
        .route(
            "/_health",
            routing::get(|| async { (StatusCode::OK, "OK") }),
        )
        .layer(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on http://localhost:5000");

    axum::serve(listener, app).await.unwrap();
}
