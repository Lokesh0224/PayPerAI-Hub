use axum::{Router, routing::get};

use crate::state::SharedState;
use crate::routes::{health::health, firebase::firebase_example};

pub mod health;
pub mod firebase;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(health))
        .route("/firebase", get(firebase_example))
        .with_state(state)
}
