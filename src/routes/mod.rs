use axum::routing::{get, post};
use axum::Router;
use utoipa::OpenApi;

use crate::Config;

mod cleanup;
mod containers;
mod eval;
mod languages;

#[derive(OpenApi)]
#[openapi(paths(cleanup::cleanup, containers::containers, eval::eval, languages::languages))]
pub struct Routes;

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/cleanup", post(cleanup::cleanup))
        .route("/containers", get(containers::containers))
        .route("/eval", post(eval::eval))
        .route("/languages", get(languages::languages))
        .with_state(config)
}
