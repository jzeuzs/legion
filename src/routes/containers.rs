use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bollard::container::ListContainersOptions;

use crate::{AppState, Result};

#[utoipa::path(
    get,
    path = "/api/containers",
    responses(
        (status = 200, body = Vec<String>),
        (status = 500, description = "Server error.")
    )
)]
pub async fn containers(State(state): State<Arc<AppState>>) -> Result<Response> {
    let mut filters = HashMap::new();
    filters.insert("name", vec!["legion-"]);

    let list_container_options = ListContainersOptions {
        filters,
        all: true,
        ..Default::default()
    };

    let output = state.docker.list_containers(Some(list_container_options)).await?;
    let list = output
        .into_iter()
        .filter_map(|summary| summary.names)
        .map(|names| names[0].replace('/', ""))
        .collect::<Vec<_>>();

    Ok(Json(list).into_response())
}
