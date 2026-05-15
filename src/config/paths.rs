use std::path::PathBuf;

use crate::utils::errors::{AppError, AppResult};

pub fn config_dir() -> AppResult<PathBuf> {
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(dir));
    }
    if cfg!(windows) {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return Ok(PathBuf::from(appdata));
        }
        if let Ok(profile) = std::env::var("USERPROFILE") {
            return Ok(PathBuf::from(profile));
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        return Ok(PathBuf::from(home).join(".config"));
    }
    Err(AppError::Config(
        "Unable to resolve config directory".to_string(),
    ))
}

pub fn config_file_path() -> AppResult<PathBuf> {
    Ok(config_dir()?.join("gai").join("config.toml"))
}

pub fn config_env_path() -> AppResult<PathBuf> {
    Ok(config_dir()?.join("gai").join(".env"))
}
