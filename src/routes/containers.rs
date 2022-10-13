use anyhow::Result;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::docker::exec;

/// # List all containers
#[openapi(tag = "Management")]
#[get("/containers")]
pub fn containers() -> Result<Json<Vec<String>>, Custom<String>> {
    let output = exec(&["ps", "--filter", "name=legion-", "--format", "{{.Names}}"])
        .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

    let list = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|ln| ln.trim().to_owned())
        .collect::<Vec<_>>();

    Ok(Json(list))
}
