use anyhow::Context;
use guess_host_triple::guess_host_triple;
use owo_colors::OwoColorize;
use termsize::Size;

use crate::{Config, Result};

macro_rules! println_centered {
    ($width:expr, $text:expr) => {
        println!("{}", console::pad_str($text, $width as usize, console::Alignment::Center, None));
    };
}

pub fn format_string_vec(arr: &[String]) -> String {
    match arr.len() {
        0 => String::new(),
        1 => arr[0].to_string(),
        2 => format!("{} and {}", arr[0], arr[1]),
        _ => {
            let mut result = arr[..arr.len() - 1].join(", ");

            result.push_str(&format!(", and {}", arr[arr.len() - 1]));
            result
        },
    }
}

pub fn print_intro(config: &Config) -> Result<()> {
    let Size {
        cols, ..
    } = termsize::get().context("failed to get terminal width")?;

    println_centered!(
        cols,
        &format!("{} {}", "Legion".bold(), format!("v{}", built_info::PKG_VERSION).bright_blue())
    );

    println_centered!(cols, &format!("Languages: {}", format_string_vec(&config.language.enabled)));

    if let Some(triple) = guess_host_triple() {
        println_centered!(cols, &format!("Running on: {}\n", triple));
    }

    Ok(())
}

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
