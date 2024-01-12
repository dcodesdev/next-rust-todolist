mod app;
mod routes;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    app::app().await;
}
