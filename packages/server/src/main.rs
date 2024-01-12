use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    println!("Hello, world!");
}
