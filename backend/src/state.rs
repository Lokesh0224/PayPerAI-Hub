use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub firebase_project: String,
    pub firebase_api_key: String,
}

pub type SharedState = Arc<AppState>;
