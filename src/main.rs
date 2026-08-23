use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

mod logger;
use logger::AppLogger;
mod metrics;
mod notifier;

mod config;
use config::*;


// Create a type alias
type SharedState = Arc<RwLock<String>>;

// makes the main fuction async
#[tokio::main]
async fn main() {

    println!(r#"
***************************************************************
*                       _                               _     *
*   ___  ___ ___  _   _| |_       __ _  __ _  ___ _ __ | |_   *
*  / __|/ __/ _ \| | | | __|____ / _` |/ _` |/ _ \ '_ \| __|  *
*  \__ \ (_| (_) | |_| | ||_____| (_| | (_| |  __/ | | | |_   *
*  |___/\___\___/ \__,_|\__|     \__,_|\__, |\___|_| |_|\__|  *
*                                      |___/                  *
***************************************************************
    "#);

    // --- GET ALL SYSTEM INFORMATION ---
    let report = metrics::build_report();
    // Convert report to JSON string 
    let json_result = serde_json::to_string_pretty(&report).unwrap();

    let shared_data: SharedState = Arc::new(RwLock::new(json_result));

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
    // pass shared_data as the state of Axum
    let app = Router::new()
        .route("/system-info", get(handle_get_data))
        .with_state(shared_data);

    // start server on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server is running at http://127.0.0.1:3000");
    println!("To test, run: curl http://127.0.0.1:3000/system-info");

    axum::serve(listener, app).await.unwrap();

    // Separate clean handler for the HTTP request
    async fn handle_get_data(State(state): State<SharedState>) -> String {
        let read_guard = state.read().await;
        read_guard.clone() // Return a copy of the string to the client
    }

}
