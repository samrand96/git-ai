mod anthropic;
mod gemini;
mod messages;
mod ollama;
mod openai_compatible;

use std::time::Duration;

use crate::config::{AppConfig, ProviderSettings};
use crate::utils::errors::{AppError, AppResult};
use crate::utils::http::HttpClient;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct AiClient {
    config: AppConfig,
    http: HttpClient,
}

impl AiClient {
    pub fn new(config: AppConfig) -> AppResult<Self> {
        let http = HttpClient::new(Duration::from_secs(90))?;
        Ok(Self { config, http })
    }

    pub fn generate(
        &self,
        provider_name: &str,
        prompt: &str,
        messages: Option<&[Message]>,
        model_override: Option<&str>,
    ) -> AppResult<String> {
        let settings = self
            .config
            .provider_settings(provider_name, model_override, None);
        self.validate_settings(&settings)?;

        match settings.name.as_str() {
            "ollama" => ollama::generate(&self.http, &settings, prompt),
            "anthropic" => anthropic::generate(&self.http, &settings, prompt, messages),
            "gemini" => gemini::generate(&self.http, &settings, prompt, messages),
            "lmstudio" | "groq" | "deepseek" | "perplexity" => {
                openai_compatible::generate(&self.http, &settings, prompt, messages)
            }
            _ => openai_compatible::generate(&self.http, &settings, prompt, messages),
        }
    }

    pub fn list_models(&self, provider_name: &str) -> AppResult<Vec<String>> {
        let settings = self.config.provider_settings(provider_name, None, None);
        match settings.name.as_str() {
            "ollama" => ollama::list_models(&self.http, &settings),
            "gemini" => gemini::list_models(&self.http, &settings),
            "anthropic" => anthropic::list_models(&self.http, &settings),
            _ => openai_compatible::list_models(&self.http, &settings),
        }
    }

    fn validate_settings(&self, settings: &ProviderSettings) -> AppResult<()> {
        if settings.requires_api_key && settings.api_key.is_none() {
            let env_name = format!("{}_API_KEY", settings.name.to_uppercase());
            return Err(AppError::MissingApiKey {
                provider: settings.name.clone(),
                env_var: format!("AI_API_KEY or {env_name}"),
            });
        }
        Ok(())
    }
}
