use serde_json::json;

use crate::config::ProviderSettings;
use crate::utils::errors::{AppError, AppResult};
use crate::utils::http::{HttpClient, headers_from_pairs};

use super::Message;
use super::messages::split_system_prompt;

pub(super) fn generate(
    http: &HttpClient,
    settings: &ProviderSettings,
    prompt: &str,
    messages: Option<&[Message]>,
) -> AppResult<String> {
    let url = format!("{}/messages", settings.base_url.trim_end_matches('/'));
    let (system, user_prompt) = split_system_prompt(prompt, messages);
    let message_payload = vec![json!({ "role": "user", "content": user_prompt })];

    let body = json!({
        "model": settings.model,
        "max_tokens": 1024,
        "messages": message_payload,
        "system": system,
    });

    let headers = headers_from_pairs(&[
        ("Content-Type", "application/json"),
        ("anthropic-version", "2023-06-01"),
    ])?;
    let headers = add_anthropic_key(headers, settings.api_key.as_deref())?;

    let response = http.post_json(&url, headers, &body)?;
    let text = response
        .get("content")
        .and_then(|content| content.as_array())
        .and_then(|array| array.first())
        .and_then(|item| item.get("text"))
        .and_then(|val| val.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    if text.is_empty() {
        return Err(AppError::InvalidResponse {
            provider: settings.name.clone(),
            details: "Empty response content".to_string(),
        });
    }
    Ok(text)
}

pub(super) fn list_models(
    http: &HttpClient,
    settings: &ProviderSettings,
) -> AppResult<Vec<String>> {
    let url = format!("{}/models", settings.base_url.trim_end_matches('/'));
    let headers = headers_from_pairs(&[
        ("Content-Type", "application/json"),
        ("anthropic-version", "2023-06-01"),
    ])?;
    let headers = add_anthropic_key(headers, settings.api_key.as_deref())?;
    let response = http.get_json(&url, headers)?;

    let models = response
        .get("data")
        .and_then(|data| data.as_array())
        .map(|data| {
            data.iter()
                .filter_map(|item| item.get("id").and_then(|id| id.as_str()))
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok(models)
}

fn add_anthropic_key(
    mut headers: reqwest::header::HeaderMap,
    api_key: Option<&str>,
) -> AppResult<reqwest::header::HeaderMap> {
    if let Some(key) = api_key {
        headers.insert(
            reqwest::header::HeaderName::from_static("x-api-key"),
            reqwest::header::HeaderValue::from_str(key)
                .map_err(|err| AppError::Config(err.to_string()))?,
        );
    }
    Ok(headers)
}
