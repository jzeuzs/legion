use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::docker::remove_containers;
use crate::{AppState, Result};

#[utoipa::path(
    post,
    path = "/api/cleanup",
    responses(
        (status = 204),
        (status = 500, description = "Server error.")
    )
)]
pub async fn cleanup(State(state): State<Arc<AppState>>) -> Result<Response> {
    remove_containers(&state).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}
