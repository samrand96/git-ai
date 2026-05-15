use crate::utils::env::get_env;

#[derive(Debug, Clone)]
pub struct ProviderSettings {
    pub name: String,
    pub model: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub api_key_source: Option<String>,
    pub requires_api_key: bool,
}

pub(super) struct ProviderDefaults {
    pub model: &'static str,
    pub base_url: &'static str,
    pub requires_api_key: bool,
}

pub fn known_provider_names() -> &'static [&'static str] {
    &[
        "openai",
        "anthropic",
        "gemini",
        "ollama",
        "lmstudio",
        "groq",
        "deepseek",
        "perplexity",
    ]
}

pub(super) fn provider_defaults(provider: &str) -> ProviderDefaults {
    match provider {
        "anthropic" => ProviderDefaults {
            model: "claude-haiku-4-5",
            base_url: "https://api.anthropic.com/v1",
            requires_api_key: true,
        },
        "ollama" => ProviderDefaults {
            model: "llama3",
            base_url: "http://localhost:11434",
            requires_api_key: false,
        },
        "gemini" => ProviderDefaults {
            model: "gemini-3.1-flash-lite",
            base_url: "https://generativelanguage.googleapis.com/v1beta",
            requires_api_key: true,
        },
        "lmstudio" => ProviderDefaults {
            model: "default",
            base_url: "http://localhost:1234/v1",
            requires_api_key: false,
        },
        "groq" => ProviderDefaults {
            model: "openai/gpt-oss-120b",
            base_url: "https://api.groq.com/openai/v1",
            requires_api_key: true,
        },
        "deepseek" => ProviderDefaults {
            model: "deepseek-v4-flash",
            base_url: "https://api.deepseek.com",
            requires_api_key: true,
        },
        "perplexity" => ProviderDefaults {
            model: "sonar",
            base_url: "https://api.perplexity.ai",
            requires_api_key: true,
        },
        _ => ProviderDefaults {
            model: "gpt-4o-mini",
            base_url: "https://api.openai.com/v1",
            requires_api_key: true,
        },
    }
}

pub(super) fn provider_api_key(provider: &str) -> (Option<String>, Option<String>) {
    let prefix = provider.to_uppercase();
    let candidates = [
        format!("{prefix}_API_KEY"),
        format!("{prefix}_TOKEN"),
        "AI_API_KEY".to_string(),
    ];

    for key in candidates {
        if let Some(value) = get_env(&key) {
            return (Some(value), Some(key));
        }
    }

    (None, None)
}

pub(super) fn env_provider_override(provider: &str, suffix: &str) -> Option<String> {
    let key = format!("{}_{}", provider.to_uppercase(), suffix);
    get_env(&key)
}
