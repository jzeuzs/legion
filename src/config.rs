use anyhow::{Error, Result};
use rocket::serde::{json, Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub language: Language,
    #[serde(default = "default_true")]
    pub prepare_containers: bool,
    #[serde(default = "default_cleanup_interval")]
    pub cleanup_interval: f64,
    #[serde(default)]
    pub cache: Cache,
    #[serde(default = "default_true")]
    pub update_images: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "kebab-case")]
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
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "kebab-case")]
pub struct Cache {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_ttl")]
    pub time_to_live: f64,
    #[serde(default = "default_tti")]
    pub time_to_idle: f64,
    #[serde(default = "default_max_capacity")]
    pub max_capacity: u64,
}

impl Config {
    /// Converts the config into a JSON string.
    ///
    /// # Errors
    ///
    /// - When the conversion fails.
    #[inline]
    pub fn stringify(&self) -> Result<String> {
        json::to_string(self).map_err(Error::msg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prepare_containers: true,
            cleanup_interval: 10.0,
            update_images: true,
            cache: Cache::default(),
            language: Language::default(),
            port: None,
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
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            enabled: true,
            time_to_live: 300.0,
            time_to_idle: 60.0,
            max_capacity: 10_000,
        }
    }
}

fn default_memory() -> u32 {
    256
}

fn default_cpus() -> f64 {
    0.25
}

fn default_runtime() -> String {
    String::from("runc")
}

fn default_timeout() -> f64 {
    30.0
}

fn default_retries() -> u8 {
    3
}

fn default_cleanup_interval() -> f64 {
    10.0
}

fn default_ttl() -> f64 {
    300.0
}

fn default_tti() -> f64 {
    60.0
}

fn default_max_capacity() -> u64 {
    10_000
}

fn default_true() -> bool {
    true
}
