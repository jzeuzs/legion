use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::Config;

/// # List all languages
pub async fn languages(State(config): State<Config>) -> Response {
    Json(config.language.enabled.clone()).into_response()
}
