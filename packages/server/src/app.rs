use crate::{error::ApiError, routes};
use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        StatusCode,
    },
    routing, Extension, Json, Router,
};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

pub async fn app() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("could not connect to database_url");

    tracing::info!("Connected to database");

    let state = ServiceBuilder::new().layer(Extension(AppState { db: Arc::new(db) }));

    let client_url = &*std::env::var("CLIENT_URL")
        .ok()
        .unwrap_or("http://localhost:3000".to_string());

    // [TODO]: Change this to only allow the frontend domain
    let cors = CorsLayer::new()
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods(Any)
        .allow_origin([client_url.parse().unwrap()]);

    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(15)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);

    // a separate background task to clean up
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        tracing::info!("rate limiting storage size: {}", governor_limiter.len());
        governor_limiter.retain_recent();
    });

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
        .layer(GovernorLayer {
            config: Box::leak(governor_conf),
        })
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on http://localhost:5000");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
