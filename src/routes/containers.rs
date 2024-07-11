use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::docker::exec;
use crate::Result;

#[utoipa::path(
    get,
    path = "/containers",
    responses(
        (status = 200, body = Vec<String>),
        (status = 500, description = "Server error.")
    )
)]
pub async fn containers() -> Result<Response> {
    let output = exec(&["ps", "--filter", "name=legion-", "--format", "{{.Names}}"]).await?;

    let list = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|ln| ln.trim().to_owned())
        .collect::<Vec<_>>();

    Ok(Json(list).into_response())
}
