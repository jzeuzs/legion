use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use bollard::container::{
    Config,
    CreateContainerOptions,
    ListContainersOptions,
    RemoveContainerOptions,
};
use bollard::image::{BuildImageOptions, BuilderVersion, ListImagesOptions};
use bollard::models::HostConfig;
use futures_util::stream::{self, StreamExt};
use owo_colors::OwoColorize;
use tracing::{error, info, warn};

use crate::util::format_string_vec;
use crate::{AppState, Result};

/// Starts a docker container with the provided `language` and an optional `runtime`.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
///
/// # Panics
///
/// - When starting the container fails.
#[tracing::instrument(skip(state, language))]
pub async fn start_container(state: &Arc<AppState>, language: &str) -> Result<()> {
    let image = format!("legion-{}", language);
    let exists = container_exists(state, language).await?;

    if exists {
        let inspect = state.docker.inspect_container(&image, None).await?;
        let is_running =
            if let Some(state) = inspect.state { state.running.unwrap_or(false) } else { false };

        if is_running {
            return Ok(());
        }
    }

    let create_container_options = CreateContainerOptions {
        name: image.as_str(),
        ..Default::default()
    };

    #[allow(clippy::cast_possible_truncation)]
    let container_config = Config {
        image: Some(image.as_str()),
        user: Some("1000:1000"),
        tty: Some(true),
        cmd: Some(vec!["/bin/sh"]),
        network_disabled: Some(true),
        working_dir: Some("/tmp/"),
        host_config: Some(HostConfig {
            runtime: Some(state.config.language.runtime.clone()),
            cpu_period: Some(100_000),
            cpu_quota: Some((state.config.language.cpus * 100_000_f64) as i64),
            memory: Some(state.config.language.memory * 1_000_000),
            memory_swap: Some(state.config.language.memory * 1_000_000),
            ..Default::default()
        }),
        ..Default::default()
    };

    state.docker.create_container(Some(create_container_options), container_config).await?;
    state.docker.start_container::<String>(image.as_str(), None).await?;

    let inspect = state.docker.inspect_container(&image, None).await?;
    let is_running =
        if let Some(state) = inspect.state { state.running.unwrap_or(false) } else { false };

    assert!(is_running, "Starting container {} failed.", image.bold().underline());

    Ok(())
}

/// Builds multiple docker images.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
///
/// # Panics
///
/// - When building the image fails.
#[tracing::instrument(skip(state))]
pub async fn build_images(state: &Arc<AppState>) -> Result<()> {
    async fn build_image(
        state: Arc<AppState>,
        language: String,
        update_images: bool,
    ) -> Result<()> {
        let list_image_options: ListImagesOptions<String> = ListImagesOptions {
            all: true,
            ..Default::default()
        };

        let output = state.docker.list_images(Some(list_image_options)).await?;
        let is_image_present =
            output.iter().any(|image| image.repo_tags.contains(&format!("legion-{}", language)));

        if update_images || is_image_present {
            info!("Building image {}...", format!("legion-{}", language).bold().underline());

            let build_image_options = BuildImageOptions {
                dockerfile: "Dockerfile".to_string(),
                t: format!("legion-{}", language),
                version: BuilderVersion::BuilderBuildKit,
                pull: true,
                rm: true,
                ..Default::default()
            };

            let compressed = languages::get_language(&language).context("language not found")?;
            let mut output =
                state.docker.build_image(build_image_options, None, Some(compressed.into()));

            while let Some(Ok(ref info)) = output.next().await {
                if info.error.is_some() || info.error_detail.is_some() {
                    error!(
                        "Building image {} failed: {:?}",
                        format!("legion-{}", language).bold().underline(),
                        info.bright_red()
                    );
                }
            }

            info!("Finished building image {}.", format!("legion-{}", language).bold().underline());
        }

        Ok(())
    }

    info!("{}", "Building images...".blue());

    let languages = state.config.language.enabled.clone();
    let update_images = state.config.update_images;

    stream::iter(
        languages
            .into_iter()
            .map(|language| tokio::spawn(build_image(state.clone(), language, update_images))),
    )
    .buffer_unordered(10)
    .collect::<Vec<_>>()
    .await;

    info!("{}", "Finished building images.".green());

    Ok(())
}

/// Starts the docker containers for use.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
///
/// # Panics
///
/// - When starting the container fails.
#[tracing::instrument(skip(state))]
pub async fn prepare_containers(state: &Arc<AppState>) -> Result<()> {
    info!("{}", "Preparing containers...".blue());

    for language in &state.config.language.enabled {
        let container_exists = container_exists(state, language).await?;

        if container_exists {
            warn!(
                "{}",
                format!("Container legion-{} already exists. Restarting.", language).bright_red()
            );

            let remove_container_options = RemoveContainerOptions {
                force: true,
                ..Default::default()
            };

            state
                .docker
                .remove_container(&format!("legion-{}", language), Some(remove_container_options))
                .await?;

            start_container(state, language).await?;
        } else {
            start_container(state, language).await?;
        }
    }

    info!("{}", "Finished preparing containers.".green());

    Ok(())
}

/// Remove running containers.
///
/// # Errors
///
/// - When removing the container fails.
#[tracing::instrument(skip(state))]
pub async fn remove_containers(state: &Arc<AppState>) -> Result<()> {
    let languages = state.config.language.enabled.clone();
    let formatted_languages = format_string_vec(&languages);

    info!("Removing containers {}...", formatted_languages.underline());

    stream::iter(languages.into_iter().map(|language| {
        let state = state.clone();

        tokio::spawn(async move {
            let remove_container_options = RemoveContainerOptions {
                force: true,
                ..Default::default()
            };

            state
                .docker
                .remove_container(&format!("legion-{}", language), Some(remove_container_options))
                .await
                .unwrap();
        })
    }))
    .buffer_unordered(10)
    .collect::<Vec<_>>()
    .await;

    info!("Removed containers {}.", formatted_languages.underline());

    Ok(())
}

/// Check if a container exists.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
#[tracing::instrument]
pub async fn container_exists(state: &Arc<AppState>, language: &str) -> Result<bool> {
    let mut filters = HashMap::new();
    filters.insert("name".to_string(), vec![format!("legion-{}", language)]);

    let list_container_options = ListContainersOptions {
        filters,
        all: true,
        ..Default::default()
    };

    let output = state.docker.list_containers(Some(list_container_options)).await?;
    let exists = !output.is_empty();

    Ok(exists)
}

mod languages {
    include!(concat!(env!("OUT_DIR"), "/languages.rs"));
}
