use dialoguer::Select;

use crate::config::{AppConfig, known_provider_names};
use crate::utils::errors::AppResult;

use super::prompts::prompt_theme;

pub fn select_provider(config: &AppConfig) -> AppResult<String> {
    let providers = known_provider_names();
    let current = config.provider();
    let default = providers
        .iter()
        .position(|provider| *provider == current)
        .unwrap_or(0);
    let idx = Select::with_theme(&prompt_theme())
        .with_prompt("provider")
        .items(providers)
        .default(default)
        .interact()?;
    Ok(providers[idx].to_string())
}

pub fn select_commit_format(config: &AppConfig) -> AppResult<String> {
    let formats = ["detailed", "one-line"];
    let current = config.commit_format();
    let default = formats
        .iter()
        .position(|format| *format == current)
        .unwrap_or(0);
    let idx = Select::with_theme(&prompt_theme())
        .with_prompt("commit format")
        .items(formats)
        .default(default)
        .interact()?;
    Ok(formats[idx].to_string())
}
