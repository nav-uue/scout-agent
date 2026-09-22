use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

mod logger;
use logger::AppLogger;
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

    let shared_data = std::sync::Arc::new(RwLock::new(json_result));

    let notifier = notifier::Notifier::new(WEBHOOK_URL.to_string());

    sleep(Duration::from_secs(1)).await;

    // --- BACKGROUND LOOP ---
    if SEND_JSON {

        // clone data for the background loop
        let loop_data = shared_data.clone();

        // start background loop
        tokio::spawn(async move {

            loop {

                let logger = AppLogger::new("scout-agent.log");

                let json = loop_data.read().await;

                match notifier.send_alert(&json).await {
                    Ok(_) => logger.log("INFO", "Message send successfully"),
                    Err(e) => logger.log("Error", &format!("Failed to send message: {}", e))
                }

                sleep(Duration::from_secs(CHECK_INTERVAL_SECS)).await;

            }

        });

    }

    // --- START AXUM WEB SERVER ON LOCAL MACHINE ---
    web::LocalInfo::new(shared_data).run().await;

}
