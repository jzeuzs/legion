#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_lines,
    clippy::module_name_repetitions,
    clippy::uninlined_format_args,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

#[cfg(not(unix))]
use std::future;
use std::sync::Arc;
use std::time::Duration;

use ::config::{Config as ConfigBuilder, Environment, File};
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::Router;
use docs::Docs;
use routes::{cleanup, containers, eval, languages};
use tokio::net::TcpListener;
use tokio::{signal, time};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info_span, warn, Level};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use util::{check_if_docker_exists, print_intro};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
pub mod docker;
mod docs;
pub mod error;
pub mod routes;
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

    print_intro(&Arc::new(config.clone()))?;

    if !config.skip_docker_check {
        check_if_docker_exists().expect("Checking for docker failed");
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

    let app = app(config.clone());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(config)).await?;

    Ok(())
}

pub fn app(config: Config) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", Docs::openapi()))
        .route("/", get(|| async { Redirect::temporary("/docs") }))
        .route("/api/cleanup", post(cleanup::cleanup))
        .route("/api/containers", get(containers::containers))
        .route("/api/eval", post(eval::eval))
        .route("/api/languages", get(languages::languages))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path =
                        request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new().level(Level::INFO).latency_unit(LatencyUnit::Micros),
                ),
        )
        .with_state(config)
}

#[allow(clippy::ignored_unit_patterns)]
async fn shutdown_signal(config: Config) {
    let ctrl_c = async {
        signal::ctrl_c().await.unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap().recv().await;
    };

    #[cfg(not(unix))]
    let terminate = future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    };

    warn!("Shutdown signal received. Killing containers.");

    docker::kill_containers(&config.language.enabled).await.expect("Failed killing containers");
}
