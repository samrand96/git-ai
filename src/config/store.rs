use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::colors::{Colors, ThemePalette};
use crate::utils::env::{get_env, load_env_files};
use crate::utils::errors::{AppError, AppResult};

use super::paths::{config_env_path, config_file_path};
use super::provider::{
    ProviderSettings, env_provider_override, provider_api_key, provider_defaults,
};
use super::schema::{ConfigFile, ProviderFile, ThemeFile};

#[derive(Debug, Clone)]
pub struct AppConfig {
    file: ConfigFile,
    config_path: PathBuf,
    env_path: PathBuf,
}

impl AppConfig {
    pub fn load() -> AppResult<Self> {
        let config_path = config_file_path()?;
        let env_path = config_env_path()?;

        load_env_files(Some(&env_path));

        let file = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            toml::from_str(&content).map_err(|err| AppError::Config(err.to_string()))?
        } else {
            ConfigFile::default()
        };

        let config = Self {
            file,
            config_path,
            env_path,
        };

        if !config.config_path.exists() {
            config.save()?;
        }

        Colors::set_theme(config.theme_palette());

        Ok(config)
    }

    pub fn save(&self) -> AppResult<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized =
            toml::to_string_pretty(&self.file).map_err(|err| AppError::Config(err.to_string()))?;
        fs::write(&self.config_path, serialized)?;
        Ok(())
    }

    pub fn provider(&self) -> String {
        get_env("AI_PROVIDER")
            .or_else(|| self.file.provider.clone())
            .unwrap_or_else(|| "openai".to_string())
    }

    pub fn commit_format(&self) -> String {
        get_env("AI_COMMIT_FORMAT")
            .or_else(|| self.file.commit_format.clone())
            .unwrap_or_else(|| "detailed".to_string())
    }

    pub fn theme(&self) -> String {
        get_env("AI_THEME")
            .or_else(|| self.file.theme.clone())
            .map(|theme| normalize_theme_key(&theme))
            .unwrap_or_else(|| "auto".to_string())
    }

    pub fn set_provider(&mut self, provider: &str) {
        self.file.provider = Some(provider.to_string());
    }

    pub fn set_commit_format(&mut self, format: &str) {
        self.file.commit_format = Some(format.to_string());
    }

    pub fn set_theme(&mut self, theme: &str) {
        let key = normalize_theme_key(theme);
        self.file.theme = Some(key.clone());
        if let Some(palette) = self.theme_palette_by_name(&key) {
            Colors::set_theme(palette);
        }
    }

    pub fn set_custom_theme(&mut self, name: &str, theme: ThemeFile) -> String {
        let key = normalize_theme_key(name);
        self.file.themes.insert(key.clone(), theme);
        key
    }

    pub fn set_provider_option(&mut self, provider: &str, key: &str, value: &str) -> AppResult<()> {
        let entry = self.file.providers.entry(provider.to_string()).or_default();
        match key.to_lowercase().as_str() {
            "model" => entry.model = Some(value.to_string()),
            "base_url" | "host" => entry.base_url = Some(value.to_string()),
            _ => {
                return Err(AppError::Config(format!(
                    "Unknown provider option '{key}'. Use MODEL or BASE_URL."
                )));
            }
        }
        Ok(())
    }

    pub fn provider_settings(
        &self,
        provider_name: &str,
        model_override: Option<&str>,
        base_url_override: Option<&str>,
    ) -> ProviderSettings {
        let provider_key = provider_name.to_lowercase();
        let defaults = provider_defaults(&provider_key);
        let provider_file = self.file.providers.get(&provider_key);

        let model = model_override
            .map(|val| val.to_string())
            .or_else(|| get_env("AI_MODEL"))
            .or_else(|| env_provider_override(&provider_key, "MODEL"))
            .or_else(|| provider_file.and_then(|p| p.model.clone()))
            .unwrap_or_else(|| defaults.model.to_string());

        let base_url = base_url_override
            .map(|val| val.to_string())
            .or_else(|| get_env("AI_BASE_URL"))
            .or_else(|| env_provider_override(&provider_key, "BASE_URL"))
            .or_else(|| provider_file.and_then(|p| p.base_url.clone()))
            .unwrap_or_else(|| defaults.base_url.to_string());

        let (api_key, api_key_source) = provider_api_key(&provider_key);

        ProviderSettings {
            name: provider_key,
            model,
            base_url,
            api_key,
            api_key_source,
            requires_api_key: defaults.requires_api_key,
        }
    }

    pub fn ensure_provider_ready(&self, provider_name: &str) -> AppResult<()> {
        let settings = self.provider_settings(provider_name, None, None);
        if settings.requires_api_key && settings.api_key.is_none() {
            let env_name = format!("{}_API_KEY", settings.name.to_uppercase());
            return Err(AppError::MissingApiKey {
                provider: provider_name.to_string(),
                env_var: format!("AI_API_KEY or {env_name}"),
            });
        }
        if settings.base_url.trim().is_empty() {
            return Err(AppError::Config(format!(
                "Provider '{}' requires a base URL or host.",
                provider_name
            )));
        }
        if settings.model.trim().is_empty() {
            return Err(AppError::Config(format!(
                "Provider '{}' requires a model name.",
                provider_name
            )));
        }
        Ok(())
    }

    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    pub fn env_path(&self) -> &Path {
        &self.env_path
    }

    pub fn providers(&self) -> &HashMap<String, ProviderFile> {
        &self.file.providers
    }

    pub fn custom_themes(&self) -> &HashMap<String, ThemeFile> {
        &self.file.themes
    }

    pub fn theme_palette(&self) -> ThemePalette {
        self.theme_palette_by_name(&self.theme())
            .unwrap_or_else(Colors::default_theme)
    }

    pub fn theme_palette_by_name(&self, name: &str) -> Option<ThemePalette> {
        let key = normalize_theme_key(name);
        Colors::built_in_theme(&key).or_else(|| {
            self.file
                .themes
                .get(&key)
                .map(|theme| theme.to_palette(Colors::default_theme()))
        })
    }
}

fn normalize_theme_key(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}
