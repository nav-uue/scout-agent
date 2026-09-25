use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::{metrics, logger::AppLogger, notifier::Notifier};


pub fn start_loop(shared_data: Arc<RwLock<String>>, notifier: Notifier, send_json: bool, send_interval: u64) {

    let data_clone = Arc::clone(&shared_data);

    tokio::spawn(async move {
        loop {
            // Wait 5 minutes before next update
            sleep(Duration::from_mins(5)).await;

            // Update the JSON data
            let fresh_data = metrics::build_report();
            if let Ok(fresh_json) = serde_json::to_string_pretty(&fresh_data) {

                let mut write_guard = data_clone.write().await;
                *write_guard = fresh_json;
                
            }
        }
    });

    // --- BACKGROUND LOOP ---
    if send_json {

        // clone data for the background loop
        let loop_data = shared_data.clone();

        // start background loop
        tokio::spawn(async move {

            let logger = AppLogger::new("scout-agent.log");

            loop {

                let json_copy = {
                    let guard = loop_data.read().await;
                    guard.clone()
                };

                match notifier.send_alert(&json_copy).await {
                    Ok(_) => logger.log("INFO", "Message send successfully"),
                    Err(e) => logger.log("Error", &format!("Failed to send message: {}", e))
                }

                sleep(Duration::from_secs(send_interval)).await;

            }

        });

    }

}