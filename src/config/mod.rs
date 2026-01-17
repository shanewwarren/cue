use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sounds_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sounds_path: dirs_home().join(".cue").join("sounds"),
        }
    }
}

impl Config {
    /// Load configuration with precedence:
    /// 1. CUE_SOUNDS_PATH environment variable
    /// 2. Config file (~/.config/cue/config.toml)
    /// 3. Default (~/.cue/sounds)
    pub fn load() -> Result<Self, ConfigError> {
        // Check environment variable first
        if let Ok(path) = env::var("CUE_SOUNDS_PATH") {
            if !path.is_empty() {
                return Ok(Self {
                    sounds_path: expand_tilde(&path),
                });
            }
        }

        // Try config file
        let config_path = Self::config_path();
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let mut config: Config = toml::from_str(&contents)?;
            config.sounds_path = expand_tilde(config.sounds_path.to_string_lossy().as_ref());
            return Ok(config);
        }

        // Return default
        Ok(Self::default())
    }

    /// Get the config file path
    pub fn config_path() -> PathBuf {
        directories::ProjectDirs::from("", "", "cue")
            .map(|dirs| dirs.config_dir().to_path_buf())
            .unwrap_or_else(|| dirs_home().join(".config").join("cue"))
            .join("config.toml")
    }
}

fn dirs_home() -> PathBuf {
    directories::BaseDirs::new()
        .map(|d| d.home_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("/"))
}

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        dirs_home().join(&path[2..])
    } else if path == "~" {
        dirs_home()
    } else {
        PathBuf::from(path)
    }
}
