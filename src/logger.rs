use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Clone)]
pub struct AppLogger {
    file: Arc<Mutex<File>>,
}

impl AppLogger {
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open log file");
        
        Self {
            file: Arc::new(Mutex::new(file))
        }
    }

    pub fn log(&self, level: &str, message: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let log_line = format!("[{}] [{}] {}\n", timestamp, level, message);
        print!("{}", log_line);

        if let Ok(mut file) = self.file.lock() {
            let _ = file.write_all(log_line.as_bytes());
            let _ = file.flush();
        }
    }
}