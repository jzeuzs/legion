use rocket::serde::json::Json;
use rocket::State;

use crate::Config;

#[get("/languages")]
pub fn languages(config: &State<Config>) -> Json<&[String]> {
    Json(&config.language.enabled)
}
