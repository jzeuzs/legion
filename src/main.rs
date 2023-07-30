#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::module_name_repetitions, clippy::uninlined_format_args)]

#[macro_use]
extern crate rocket;

use std::env;
use std::sync::Arc;
use std::time::Duration;

use ::config::{Config as ConfigBuilder, Environment, File};
use anyhow::Result;
use moka::future::{Cache as MokaCache, CacheBuilder};
use rocket::tokio::{self, time};
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

mod config;
pub mod docker;
mod routes;
mod util;

pub type Cache = MokaCache<routes::eval::Eval, Arc<routes::eval::EvalResult>>;
pub type Config = Arc<config::Config>;

#[allow(clippy::no_effect_underscore_binding)]
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_env_var("LEGION_LOG")
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(""),
        )
        .init();

    let config: config::Config = ConfigBuilder::builder()
        .add_source(File::with_name("Legion"))
        .add_source(Environment::with_prefix("LEGION"))
        .build()
        .expect("Couldn't find config file")
        .try_deserialize()
        .expect("Deserializing config failed");

    if !config.skip_docker_check {
        util::check_if_docker_exists().expect("Checking for docker failed");
    }

    docker::build_images(&config.language.enabled, config.update_images)
        .expect("Failed building images");

    if config.prepare_containers {
        docker::prepare_containers(&config.language.enabled, &config.language)
            .expect("Failed preparing containers");
    }

    if let Some(port) = config.port {
        env::set_var("ROCKET_PORT", port.to_string());
    }

    let config = Arc::new(config);
    let config_2 = Arc::clone(&config);

    tokio::spawn(async move {
        let mut interval =
            time::interval(Duration::from_secs_f64(config_2.cleanup_interval * 60.0));

        // ticks immediately
        interval.tick().await;

        loop {
            interval.tick().await;
            docker::kill_containers(&config_2.language.enabled).expect("Failed killing containers");
        }
    });

    let cache: Cache =
        CacheBuilder::new(if config.cache.enabled { config.cache.max_capacity } else { 0 })
            .time_to_idle(Duration::from_secs_f64(config.cache.time_to_idle * 60.0))
            .time_to_live(Duration::from_secs_f64(config.cache.time_to_live * 60.0))
            .build();

    let _rocket = rocket::build()
        .manage(config)
        .manage(cache)
        .mount("/", openapi_get_routes![
            routes::eval::eval,
            routes::languages::languages,
            routes::containers::containers,
            routes::cleanup::cleanup
        ])
        .mount(
            "/docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await?;

    Ok(())
}
