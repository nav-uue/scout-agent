use std::sync::Arc;
use tokio::sync::RwLock;

mod background;
mod logger;
mod metrics;
mod notifier;
mod config;
use config::*;
mod web;

// makes the main fuction async
#[tokio::main]
async fn main() {

    println!("{}", LABEL);

    // --- GET ALL SYSTEM INFORMATION ---
    let report = metrics::build_report();
    // Convert report to JSON string 
    let json_result = serde_json::to_string_pretty(&report).unwrap();
    let shared_data = Arc::new(RwLock::new(json_result));
    let notifier = notifier::Notifier::new(WEBHOOK_URL.to_string());

    // sleep(Duration::from_secs(1)).await;

    background::start_loop(
        shared_data.clone(), 
        notifier, 
        SEND_JSON, 
        SEND_INTERVAL
    );

    // --- START AXUM WEB SERVER ON LOCAL MACHINE ---
    web::LocalInfo::new(shared_data).run().await;

}
