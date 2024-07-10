#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_lines,
    clippy::module_name_repetitions,
    clippy::uninlined_format_args,
    clippy::missing_panics_doc
)]

use std::sync::Arc;
use std::time::Duration;

use ::config::{Config as ConfigBuilder, Environment, File};
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
use tokio::time;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

mod config;
pub mod docker;
pub mod error;
mod routes;
mod util;

pub type Result<T> = anyhow::Result<T, error::AppError>;
pub type Config = Arc<config::Config>;

#[tokio::main]
async fn main() -> Result<()> {
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

    docker::build_images(&config.language.enabled, config.update_images).await?;

    if config.prepare_containers {
        docker::prepare_containers(&config.language.enabled, &config.language).await?;
    }

    let port = config.port.unwrap_or(3000);

    let config = Arc::new(config);
    let config_2 = Arc::clone(&config);

    tokio::spawn(async move {
        let mut interval =
            time::interval(Duration::from_secs_f64(config_2.cleanup_interval * 60.0));

        // ticks immediately
        interval.tick().await;

        loop {
            interval.tick().await;
            docker::kill_containers(&config_2.language.enabled)
                .await
                .expect("Failed killing containers");
        }
    });

    let app = app(config);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

pub fn app(config: Config) -> Router {
    Router::new()
        .route("/languages", get(routes::languages::languages))
        .route("/containers", get(routes::containers::containers))
        .route("/cleanup", post(routes::cleanup::cleanup))
        .route("/eval", post(routes::eval::eval))
        .with_state(config)
}
