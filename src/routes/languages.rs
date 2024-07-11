use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::Config;

#[utoipa::path(
    get,
    path = "/api/languages",
    responses(
        (status = 200, body = Vec<String>),
        (status = 500, description = "Server error.")
    )
)]
pub async fn languages(State(config): State<Config>) -> Response {
    Json(config.language.enabled.clone()).into_response()
}
