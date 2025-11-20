mod routes;
mod state;

use axum::serve;
use dotenv::dotenv;
use reqwest::Client;
use state::{AppState, SharedState};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let firebase_project =
        std::env::var("FIREBASE_PROJECT").unwrap_or_else(|_| "your-project-id".to_string());

    let firebase_api_key =
        std::env::var("FIREBASE_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());

    let state: SharedState = Arc::new(AppState {
        client: Client::new(),
        firebase_project,
        firebase_api_key,
    });

    let app = routes::create_router(state);

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
