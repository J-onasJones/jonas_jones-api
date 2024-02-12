use dotenv::dotenv;
use tokio::time::{sleep, Duration};
use v1::{run_kcomebacks_command, run_likedsongs_command, run_projects_command};
use std::fs;

pub mod v1;
pub mod logger;
pub mod tools;
pub mod server;
pub mod error_responses;

pub use logger::Logger;
pub use tools::parse_ip;


async fn periodic_script_runner() {
    loop {
        Logger::info("Running periodic scripts...");
        // Run all Functions
        let _ = run_kcomebacks_command();
        let _ = run_projects_command();
        let _ = run_likedsongs_command();

        // Sleep for 6 hours
        sleep(Duration::from_secs(6 * 60 * 60)).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Start the api
    let server_task = tokio::spawn(async {
        server::serve().await;
    });

    // periodic script runner
    let second_task = tokio::spawn(async {
        // check if the local repository exists, if not, clone it
        if !fs::metadata("./resources/turbo_octo_potato").is_ok() {
            v1::run_setup().unwrap();
        };
        periodic_script_runner().await;
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(server_task, second_task);
}
