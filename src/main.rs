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
use bollard::Docker;
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
use util::print_intro;
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

#[derive(Debug)]
pub struct AppState {
    config: config::Config,
    docker: Docker,
}

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

    let docker = Docker::connect_with_socket_defaults()?;

    let state = Arc::new(AppState {
        config,
        docker,
    });

    print_intro(&Arc::new(state.config.clone()))?;
    docker::build_images(&state).await?;

    if state.config.prepare_containers {
        docker::prepare_containers(&state).await?;
    }

    let port = state.config.port.unwrap_or(3000);
    let state_2 = state.clone();

    tokio::spawn(async move {
        let mut interval =
            time::interval(Duration::from_secs_f64(state_2.config.cleanup_interval * 60.0));

        // ticks immediately
        interval.tick().await;

        loop {
            interval.tick().await;
            docker::remove_containers(&state_2).await.expect("Failed killing containers");
        }
    });

    let app = app(state.clone());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(state)).await?;

    Ok(())
}

pub fn app(state: Arc<AppState>) -> Router {
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
        .with_state(state)
}

#[allow(clippy::ignored_unit_patterns)]
async fn shutdown_signal(state: Arc<AppState>) {
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
    docker::remove_containers(&state).await.expect("Failed killing containers");
}
