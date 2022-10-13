use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

use crate::Config;

/// # List all languages
#[openapi(tag = "General")]
#[get("/languages")]
pub fn languages(config: &State<Config>) -> Json<&[String]> {
    Json(&config.language.enabled)
}
