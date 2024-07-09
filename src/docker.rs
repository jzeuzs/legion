use std::process::{Command, Output, Stdio};

use anyhow::Result;
use owo_colors::OwoColorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::config::Language;

/// Executes a docker command.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
pub fn exec(args: &[&str]) -> Result<Output> {
    let output = Command::new("docker")
        .args(args)
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;

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
pub fn start_container(language: &str, config: &Language) -> Result<()> {
    let image = format!("legion-{}", language);

    let is_running = exec(&["inspect", &image, "--format", "'{{.State.Status}}'"])?;

    if !String::from_utf8_lossy(&is_running.stdout).contains("running") {
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
    ])?;

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
pub fn build_images(languages: &[String], update_images: bool) -> Result<()> {
    info!("{}", "Building images...".blue());

    let build_image = |language: &String| {
        let output = &exec(&["images", "-q", &format!("legion-{}", language)])
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
    };

    languages.par_iter().for_each(build_image);

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
pub fn prepare_containers(languages: &[String], config: &Language) -> Result<()> {
    info!("{}", "Preparing containers...".blue());

    for language in languages {
        let container_exists = container_exists(language)?;

        if container_exists {
            warn!(
                "{}",
                format!("Container legion-{} already exists. Restarting.", language).bright_red()
            );

            exec(&["kill", &format!("legion-{}", language)])?;
            start_container(language, config)?;
        } else {
            start_container(language, config)?;
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
pub fn kill_containers(languages: &[String]) -> Result<()> {
    let kill_container = |language: &String| -> Result<()> {
        info!("Killing container {}...", format!("legion-{}", language).bold().underline());
        exec(&["kill", &format!("legion-{}", language)])?;
        info!("Killed container {}.", format!("legion-{}", language).bold().underline());

        Ok(())
    };

    languages.par_iter().for_each(|lang| kill_container(lang).expect("Failed killing container"));

    Ok(())
}

/// Check if a container exists.
///
/// # Errors
///
/// - When the Docker CLI is not on your `PATH`.
pub fn container_exists(language: &str) -> Result<bool> {
    Ok(exec(&["top", &format!("legion-{}", language)])?.status.success())
}
