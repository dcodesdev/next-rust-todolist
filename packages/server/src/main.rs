mod app;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    app::app().await;
}
