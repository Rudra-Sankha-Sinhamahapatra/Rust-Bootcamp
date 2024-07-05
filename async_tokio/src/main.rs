use tokio::time::{sleep, Duration};
use log::info;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    println!("Hello, world!");

    // Log a message at the Info level
    info!("Starting the async main function");

    // Example of using tokio::time
    info!("Sleeping for 2 seconds...");
    sleep(Duration::from_secs(2)).await;
    info!("Woke up after 2 seconds");
}
