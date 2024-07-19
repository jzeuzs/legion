use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::AppState;

#[utoipa::path(
    get,
    path = "/api/languages",
    responses(
        (status = 200, body = Vec<String>),
        (status = 500, description = "Server error.")
    )
)]
pub async fn languages(State(state): State<Arc<AppState>>) -> Response {
    Json(state.config.language.enabled.clone()).into_response()
}
