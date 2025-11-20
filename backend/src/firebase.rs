use axum::{extract::State, Json};
use axum::http::StatusCode;
use serde_json::json;

use crate::state::SharedState;

pub async fn firebase_example(
    State(state): State<SharedState>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {

    let url = format!(
        "https://firebaseremoteconfig.googleapis.com/v1/projects/{}/namespaces/default:fetch?key={}",
        state.firebase_project, state.firebase_api_key
    );

    let res = state.client
        .get(&url)
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("request failed: {}", e)))?;

    let status = res.status();
    let body = res.text()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("read body failed: {}", e)))?;

    Ok(Json(json!({
        "status": status.as_u16(),
        "body": body
    })))
}
