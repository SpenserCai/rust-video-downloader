use crate::error::{DownloaderError, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Default)]
#[allow(dead_code)] // Some fields are reserved for future use
pub struct Config {
    #[serde(default)]
    pub default_quality: Option<Vec<String>>,
    #[serde(default)]
    pub default_codec: Option<Vec<String>>,
    #[serde(default)]
    pub thread_count: Option<usize>,
    #[serde(default)]
    pub output_template: Option<String>,
    #[serde(default)]
    pub multi_output_template: Option<String>,
    #[serde(default)]
    pub auth: Option<AuthConfig>,
    #[serde(default)]
    pub paths: Option<PathsConfig>,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct AuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PathsConfig {
    pub ffmpeg: Option<PathBuf>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            tracing::debug!("Config file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| DownloaderError::Config(format!("Failed to read config file: {}", e)))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| DownloaderError::Config(format!("Failed to parse config file: {}", e)))?;

        Ok(config)
    }

    pub fn load_default() -> Result<Self> {
        // Try to load from current directory
        let current_dir_config = PathBuf::from("rvd.toml");
        if current_dir_config.exists() {
            return Self::load(&current_dir_config);
        }

        // Try to load from home directory
        if let Some(home) = dirs::home_dir() {
            let home_config = home.join(".config").join("rvd").join("config.toml");
            if home_config.exists() {
                return Self::load(&home_config);
            }
        }

        Ok(Self::default())
    }
}
