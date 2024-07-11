use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub language: Language,
    #[serde(default = "default_true")]
    pub prepare_containers: bool,
    #[serde(default = "default_cleanup_interval")]
    pub cleanup_interval: f64,
    #[serde(default = "default_true")]
    pub update_images: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(default = "default_false")]
    pub skip_docker_check: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    pub enabled: Vec<String>,
    #[serde(default = "default_memory")]
    pub memory: u32,
    #[serde(default = "default_cpus")]
    pub cpus: f64,
    #[serde(default = "default_runtime")]
    pub runtime: String,
    #[serde(default = "default_timeout")]
    pub timeout: f64,
    #[serde(default = "default_retries")]
    pub retries: u8,
    #[serde(default = "default_max_process_count")]
    pub max_process_count: u32,
    #[serde(default = "default_max_open_files")]
    pub max_open_files: u32,
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u32,
}

impl Config {
    /// Converts the config into a JSON string.
    ///
    /// # Errors
    ///
    /// - When the conversion fails.
    #[inline]
    pub fn stringify(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Error::msg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prepare_containers: true,
            cleanup_interval: 10.0,
            update_images: true,
            language: Language::default(),
            port: None,
            skip_docker_check: false,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language {
            enabled: Vec::with_capacity(0),
            memory: 256,
            cpus: 0.25,
            runtime: String::from("runc"),
            timeout: 30.0,
            retries: 3,
            max_process_count: 64,
            max_open_files: 2048,
            max_file_size: 10_000_000,
        }
    }
}

const fn default_memory() -> u32 {
    512
}

const fn default_cpus() -> f64 {
    0.25
}

fn default_runtime() -> String {
    String::from("runc")
}

const fn default_timeout() -> f64 {
    30.0
}

const fn default_retries() -> u8 {
    3
}

const fn default_cleanup_interval() -> f64 {
    10.0
}

const fn default_true() -> bool {
    true
}

const fn default_false() -> bool {
    false
}

const fn default_max_process_count() -> u32 {
    64
}

const fn default_max_open_files() -> u32 {
    2048
}

const fn default_max_file_size() -> u32 {
    10_000_000
}
