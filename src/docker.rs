use std::process::{Output, Stdio};

use anyhow::Result;
use futures_util::stream::{self, StreamExt};
use owo_colors::OwoColorize;
use tokio::process::Command;
use tracing::{info, warn};

use crate::config::Language;
use crate::util::format_string_vec;

/// Executes a docker command.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
#[tracing::instrument(skip(args))]
pub async fn exec(args: &[&str]) -> Result<Output> {
    let output = Command::new("docker")
        .args(args)
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .await?;

    Ok(output)
}

/// Starts a docker container with the provided `language` and an optional `runtime`.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
///
/// # Panics
///
/// - When starting the container fails.
#[tracing::instrument(skip(config))]
pub async fn start_container(language: &str, config: &Language) -> Result<()> {
    let image = format!("legion-{}", language);

    let is_running = exec(&["inspect", &image, "--format", "'{{.State.Status}}'"]).await?;

    if String::from_utf8_lossy(&is_running.stdout).contains("running") {
        return Ok(());
    }

    let output = exec(&[
        "run",
        &format!("--runtime={}", config.runtime),
        "--rm",
        &format!("--name={}", image),
        "-u1000:1000",
        "-w/tmp/",
        "-dt",
        "--net=none",
        &format!("--cpus={}", config.cpus),
        &format!("-m={}m", config.memory),
        &format!("--memory-swap={}m", config.memory),
        &image,
        "/bin/sh",
    ])
    .await?;

    assert!(
        output.status.success(),
        "Starting container {} failed: {}",
        image.bold().underline(),
        String::from_utf8_lossy(if output.stderr.is_empty() {
            &output.stdout
        } else {
            &output.stderr
        })
        .bright_red()
    );

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
#[tracing::instrument]
pub async fn build_images(languages: &[String], update_images: bool) -> Result<()> {
    async fn build_image(language: String, update_images: bool) {
        let output = &exec(&["images", "-q", &format!("legion-{}", language)])
            .await
            .expect("Failed checking images");

        let is_image_present = String::from_utf8_lossy(&output.stdout);

        if update_images || is_image_present.trim().is_empty() {
            info!("Building image {}...", format!("legion-{}", language).bold().underline());

            let output = exec(&[
                "build",
                "-t",
                &format!("legion-{}", language),
                &format!("languages/{}", language),
            ])
            .await
            .expect("Failed building image");

            assert!(
                output.status.success(),
                "Building image {} failed: {}",
                format!("legion-{}", language).bold().underline(),
                String::from_utf8_lossy(if output.stderr.is_empty() {
                    &output.stdout
                } else {
                    &output.stderr
                })
                .bright_red()
            );

            info!("Finished building image {}.", format!("legion-{}", language).bold().underline());
        }
    }

    info!("{}", "Building images...".blue());

    let languages = languages.to_vec();

    stream::iter(
        languages.into_iter().map(|language| tokio::spawn(build_image(language, update_images))),
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
#[tracing::instrument(skip(config))]
pub async fn prepare_containers(languages: &[String], config: &Language) -> Result<()> {
    info!("{}", "Preparing containers...".blue());

    for language in languages {
        let container_exists = container_exists(language).await?;

        if container_exists {
            warn!(
                "{}",
                format!("Container legion-{} already exists. Restarting.", language).bright_red()
            );

            exec(&["kill", &format!("legion-{}", language)]).await?;
            start_container(language, config).await?;
        } else {
            start_container(language, config).await?;
        }
    }

    info!("{}", "Finished preparing containers.".green());

    Ok(())
}

/// Kill running containers.
///
/// # Errors
///
/// - When killing the container fails.
#[tracing::instrument]
pub async fn kill_containers(languages: &[String]) -> Result<()> {
    let formatted_languages = format_string_vec(languages);

    info!("Killing containers {}...", formatted_languages.underline());

    let languages = languages.to_vec();

    stream::iter(languages.into_iter().map(|language| {
        tokio::spawn(async move {
            exec(&["kill", &format!("legion-{}", language)])
                .await
                .expect("Failed killing container")
        })
    }))
    .buffer_unordered(10)
    .collect::<Vec<_>>()
    .await;

    info!("Killed containers {}.", formatted_languages.underline());

    Ok(())
}

/// Check if a container exists.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
#[tracing::instrument]
pub async fn container_exists(language: &str) -> Result<bool> {
    Ok(exec(&["top", &format!("legion-{}", language)]).await?.status.success())
}
