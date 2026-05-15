use dialoguer::Select;

use crate::config::{AppConfig, ProviderSettings};
use crate::services::ai::AiClient;
use crate::utils::colors::Colors;
use crate::utils::errors::AppResult;

use super::prompts::{input_with_default, prompt_theme};

pub fn select_model(
    config: &AppConfig,
    provider: &str,
    settings: &ProviderSettings,
) -> AppResult<String> {
    if settings.requires_api_key && settings.api_key.is_none() {
        println!(
            "{}",
            Colors::warning("[warn] API key missing; skipping model discovery")
        );
        return input_with_default("MODEL", &settings.model);
    }

    println!(
        "{}",
        Colors::info(format!(
            "[net] loading models from {} ({})",
            provider, settings.base_url
        ))
    );

    match AiClient::new(config.clone()).and_then(|ai| ai.list_models(provider)) {
        Ok(models) if !models.is_empty() => select_from_models(&models, &settings.model),
        Ok(_) => {
            println!(
                "{}",
                Colors::warning("[warn] endpoint returned an empty model list")
            );
            input_with_default("MODEL", &settings.model)
        }
        Err(err) => {
            println!(
                "{}",
                Colors::warning(format!(
                    "[warn] model discovery failed: {}",
                    err.user_message()
                ))
            );
            input_with_default("MODEL", &settings.model)
        }
    }
}

fn select_from_models(models: &[String], current: &str) -> AppResult<String> {
    let mut items = vec![format!("keep current ({current})")];
    items.extend(models.iter().cloned());
    let default = models
        .iter()
        .position(|model| model == current)
        .map(|idx| idx + 1)
        .unwrap_or(0);
    let idx = Select::with_theme(&prompt_theme())
        .with_prompt("model")
        .items(&items)
        .default(default)
        .interact()?;
    if idx == 0 {
        Ok(current.to_string())
    } else {
        Ok(items[idx].clone())
    }
}
