use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::docker::kill_containers;
use crate::{Config, Result};

/// # Kill all containers
pub async fn cleanup(State(config): State<Config>) -> Result<Response> {
    kill_containers(&config.language.enabled).await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}
