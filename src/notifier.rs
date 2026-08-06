use serde::Serialize;


#[derive(Serialize)]
struct WebhookInventory {
    content: String,
}

pub struct Notifier {
    client: reqwest::Client,
    webhook_url: String,
}

impl Notifier {
    pub fn new(webhook_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            webhook_url,
        }
    }

    pub async fn send_alert(&self, message: &str) -> Result<(), reqwest::Error> {
        let inventory = WebhookInventory {
            content: message.to_string(),
        };

        self.client
            .post(&self.webhook_url)
            .json(&inventory)
            .send()
            .await?;
        
        Ok(())
    }
}