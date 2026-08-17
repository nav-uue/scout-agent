use std::time::Duration;
use tokio::time::sleep;

mod logger;
use logger::AppLogger;
mod metrics;
mod notifier;

mod config;
use config::*;


// makes the main fuction async
#[tokio::main]
async fn main() {
    println!("Monitoring agent started successfully");

    let host_id = String::from("HomePC");
    let report = metrics::build_report(host_id);
    let json_result = serde_json::to_string_pretty(&report).unwrap();

    let notifier = notifier::Notifier::new(WEBHOOK_URL.to_string());

    sleep(Duration::from_secs(1)).await;

    loop {

        let logger = AppLogger::new("scout-agent.log");

        match notifier.send_alert(&json_result).await {
            Ok(_) => logger.log("INFO", "Message send successfully"),
            Err(e) => logger.log("Error", &format!("Failed to send message: {}", e))
        }

        sleep(Duration::from_secs(CHECK_INTERVAL_SECS)).await;

    }
}
