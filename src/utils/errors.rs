use std::io;

use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Prompt error: {0}")]
    Prompt(#[from] dialoguer::Error),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Git command failed: {0}")]
    Git(String),
    #[error("Missing API key for provider '{provider}'. Set {env_var}.")]
    MissingApiKey { provider: String, env_var: String },
    #[error("Invalid response from provider '{provider}': {details}")]
    InvalidResponse { provider: String, details: String },
    #[error("HTTP request failed with status {status}")]
    HttpStatus {
        url: String,
        status: String,
        body: String,
    },
}

impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Io(err) => format!("[io] {err}"),
            Self::Http(err) => {
                if err.is_timeout() {
                    "[net] request timed out. Check the endpoint and try again.".to_string()
                } else if err.is_connect() {
                    "[net] could not connect to the provider endpoint. Check network access and base URL.".to_string()
                } else {
                    format!("[net] request failed: {err}")
                }
            }
            Self::Json(_) => {
                "[data] provider returned a response the CLI could not parse as JSON.".to_string()
            }
            Self::Prompt(err) => format!("[prompt] {err}"),
            Self::Config(message) => format!("[cfg] {message}"),
            Self::Git(message) => format!("[git] {message}"),
            Self::MissingApiKey { provider, env_var } => {
                format!("[auth] missing API key for provider '{provider}'. Set {env_var}.")
            }
            Self::InvalidResponse { provider, details } => {
                format!("[data] invalid response from provider '{provider}': {details}")
            }
            Self::HttpStatus { url, status, body } => http_status_message(url, status, body),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

fn http_status_message(url: &str, status: &str, body: &str) -> String {
    let host = reqwest::Url::parse(url)
        .ok()
        .and_then(|parsed| parsed.host_str().map(str::to_string))
        .unwrap_or_else(|| url.to_string());
    let provider_message = provider_error_message(body).map(mask_secrets);

    if status.starts_with("401") || status.starts_with("403") {
        let mut message = format!(
            "[auth] provider rejected the request ({status}) at {host}. Check the active API key, provider, and base URL."
        );
        if let Some(provider_message) = provider_message {
            message.push_str(&format!("\n[auth] provider says: {provider_message}"));
        }
        return message;
    }

    if status.starts_with("429") {
        return format!(
            "[rate] provider rate-limited the request ({status}) at {host}. Wait and retry, or lower request volume."
        );
    }

    if status.starts_with('5') {
        return format!(
            "[net] provider endpoint returned {status} at {host}. This is usually a provider-side or gateway problem."
        );
    }

    let mut message = format!("[http] provider request failed ({status}) at {host}.");
    if let Some(provider_message) = provider_message {
        message.push_str(&format!("\n[http] provider says: {provider_message}"));
    }
    message
}

fn provider_error_message(body: &str) -> Option<String> {
    let json = serde_json::from_str::<Value>(body).ok()?;
    json.pointer("/error/message")
        .or_else(|| json.get("message"))
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn mask_secrets(message: String) -> String {
    message
        .split_whitespace()
        .map(mask_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn mask_word(word: &str) -> String {
    let trimmed =
        word.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-' && ch != '_');
    let lower = trimmed.to_ascii_lowercase();
    let looks_like_key = lower.starts_with("sk-")
        || lower.starts_with("sk_")
        || lower.starts_with("ds-")
        || trimmed.len() >= 24 && trimmed.chars().any(|ch| ch.is_ascii_digit());

    if !looks_like_key {
        return word.to_string();
    }

    let masked = mask_token(trimmed);
    word.replacen(trimmed, &masked, 1)
}

fn mask_token(token: &str) -> String {
    let chars = token.chars().collect::<Vec<_>>();
    match chars.len() {
        0..=8 => "*".repeat(chars.len().max(4)),
        len => format!(
            "{}{}{}",
            chars.iter().take(4).collect::<String>(),
            "*".repeat((len - 8).clamp(6, 18)),
            chars.iter().skip(len - 4).collect::<String>()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::AppError;

    #[test]
    fn http_status_user_message_sanitizes_auth_payloads() {
        let err = AppError::HttpStatus {
            url: "https://api.openai.com/v1/chat/completions".to_string(),
            status: "401 Unauthorized".to_string(),
            body: r#"{"error":{"message":"Incorrect API key provided: sk-f32f1aaaaaaaaaaaaaaaaaaaa9fe5.","type":"invalid_request_error"}}"#.to_string(),
        };

        let message = err.user_message();
        assert!(message.contains("[auth]"));
        assert!(message.contains("api.openai.com"));
        assert!(!message.contains("HttpStatus"));
        assert!(!message.contains("body"));
        assert!(!message.contains("sk-f32f1aaaaaaaaaaaaaaaaaaaa9fe5"));
    }
}
