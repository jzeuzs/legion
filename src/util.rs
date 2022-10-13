use std::process::{self, Command, Stdio};

use anyhow::Result;
use owo_colors::OwoColorize;

pub fn check_if_docker_exists() -> Result<()> {
    let cmd = Command::new("docker")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .status()?;

    if !cmd.success() {
        println!(
            "The {} binary is missing. Maybe its missing on the {} environment variable?",
            "docker".bold().blue(),
            "$PATH".bold()
        );

        process::exit(libc::EXIT_FAILURE);
    }

    Ok(())
}
