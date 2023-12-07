use dotenv::dotenv;

pub mod v1;
pub mod logger;
pub mod tools;
pub mod server;

pub use logger::Logger;
pub use tools::parse_ip;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    // load .env file
    dotenv().ok();

    server::serve().await;
}
