use std::time::Duration;
use tokio::time::sleep;

mod logger;
mod metrics;
mod notifier;

mod config;
use config::*;


// makes the main fuction async
#[tokio::main]
async fn main() {
    println!("Monitoring agent started successfully");

    let mut collector = metrics::MetricsCollector::new();
    let notifier = notifier::Notifier::new(WEBHOOK_URL.to_string());

    sleep(Duration::from_secs(1)).await;

    loop {
        let metrics = collector.collect();
        println!(
            "Metrics: CPU: {:.1}%, RAM: {:.1}%",
            metrics.cpu_usage, metrics.ram_usage_percent
        );

        // Checking threshold breaches
        if metrics.cpu_usage > CPU_THRESHOLD {
            let msg = format!("🚨 Warning! High CPU usage: {:.1}%", metrics.cpu_usage);
            match notifier.send_alert(&msg).await {
                Ok(_) => logger.log("INFO", "Message sent successfully"),
                Err(e) => logger.log("ERROR", &format!("Failed to send alert: {}", e))
            }
        }

        
    }
}
