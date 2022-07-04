use anyhow::Result;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;

use crate::docker::kill_containers;
use crate::Config;

#[post("/cleanup")]
pub fn cleanup(config: &State<Config>) -> Result<Status, Custom<String>> {
    kill_containers(&config.language.enabled)
        .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

    Ok(Status::NoContent)
}
