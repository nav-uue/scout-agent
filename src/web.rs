use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use tokio::sync::RwLock;


// Create a type alias
type SharedState = Arc<RwLock<String>>;

pub struct LocalInfo {
    shared_data: SharedState
}

impl LocalInfo {
    pub fn new(shared_data: SharedState) -> Self {
        Self { shared_data }
    }

    pub async fn run(self) {
        // pass shared_data as the state of Axum
        let app = Router::new()
            .route("/system-info", get(Self::handle_get_data))
            .with_state(self.shared_data);

        // start server on port 3000
        let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();

        println!("🚀 Server is running at http://127.0.0.1:3000");
        println!("To test, run: curl http://127.0.0.1:3000/system-info");
        
        // run server
        axum::serve(listner, app).await.unwrap();

    }

    // Separate clean handler for the HTTP request
    async fn handle_get_data(State(state): State<SharedState>) -> String {
        let read_guard = state.read().await;
        read_guard.clone() // Return a copy of the string to the client
    }

}