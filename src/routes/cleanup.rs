use anyhow::Result;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;
use rocket_okapi::openapi;

use crate::docker::kill_containers;
use crate::Config;

/// # Kill all containers
#[openapi(tag = "Management")]
#[post("/cleanup")]
pub fn cleanup(config: &State<Config>) -> Result<Status, Custom<String>> {
    kill_containers(&config.language.enabled)
        .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

    Ok(Status::NoContent)
}
