mod app;
mod error;
mod middlewares;
mod routes;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    app::app().await;
}
