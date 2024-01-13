mod app;
mod error;
mod middlewares;
mod models;
mod routes;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    app::app().await;
}
