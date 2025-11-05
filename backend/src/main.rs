//refactor the whole backend 
use axum::{routing::get, Router, Json, extract::State};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use reqwest::Client;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    client: Client,
    firebase_project: String,
    firebase_api_key: String,
}

async fn health() -> &'static str {
    "OK"
}

async fn firebase_example(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let url = format!(
        "https://firebaseremoteconfig.googleapis.com/v1/projects/{}/namespaces/default:fetch?key={}",
        state.firebase_project, state.firebase_api_key
    );

    let res = state.client
        .get(&url)
        .send()
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("request failed: {}", e)))?;

    let status = res.status();
    let body = res.text().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("read body failed: {}", e)))?;

    Ok(Json(json!({
        "status": status.as_u16(),
        "body": body
    })))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let firebase_project = std::env::var("FIREBASE_PROJECT").unwrap_or_else(|_| "your-project-id".to_string());
    let firebase_api_key = std::env::var("FIREBASE_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());

    let state = Arc::new(AppState {
        client: Client::new(),
        firebase_project,
        firebase_api_key,
    });

    let app = Router::new()
        .route("/", get(health))
        .route("/firebase", get(firebase_example))
        .with_state(state);


    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
