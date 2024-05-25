mod app;
mod error;
mod middlewares;
mod models;
mod routes;
mod utils;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    app::app().await;
}
