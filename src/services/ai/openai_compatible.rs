use serde_json::json;

use crate::config::ProviderSettings;
use crate::utils::errors::{AppError, AppResult};
use crate::utils::http::{HttpClient, headers_from_pairs};
use crate::utils::json::first_array_item;

use super::Message;

pub(super) fn generate(
    http: &HttpClient,
    settings: &ProviderSettings,
    prompt: &str,
    messages: Option<&[Message]>,
) -> AppResult<String> {
    let url = format!(
        "{}/chat/completions",
        settings.base_url.trim_end_matches('/')
    );
    let message_payload = if let Some(messages) = messages {
        messages
            .iter()
            .map(|msg| json!({ "role": msg.role, "content": msg.content }))
            .collect::<Vec<_>>()
    } else {
        vec![json!({ "role": "user", "content": prompt })]
    };

    let body = json!({
        "model": settings.model,
        "messages": message_payload,
    });

    let headers = headers_from_pairs(&[("Content-Type", "application/json")])?;
    let headers = add_auth_header(headers, settings.api_key.as_deref())?;

    let response = http.post_json(&url, headers, &body)?;
    let choice =
        first_array_item(&response, "choices").ok_or_else(|| AppError::InvalidResponse {
            provider: settings.name.clone(),
            details: "Missing choices".to_string(),
        })?;

    let content = choice
        .get("message")
        .and_then(|msg: &serde_json::Value| msg.get("content"))
        .and_then(|val: &serde_json::Value| val.as_str())
        .map(|val: &str| val.to_string())
        .or_else(|| {
            choice
                .get("text")
                .and_then(|val: &serde_json::Value| val.as_str())
                .map(|val: &str| val.to_string())
        })
        .ok_or_else(|| AppError::InvalidResponse {
            provider: settings.name.clone(),
            details: "Missing response content".to_string(),
        })?;

    if content.trim().is_empty() {
        return Err(AppError::InvalidResponse {
            provider: settings.name.clone(),
            details: "Empty response content".to_string(),
        });
    }
    Ok(content.trim().to_string())
}

pub(super) fn list_models(
    http: &HttpClient,
    settings: &ProviderSettings,
) -> AppResult<Vec<String>> {
    let url = format!("{}/models", settings.base_url.trim_end_matches('/'));
    let headers = headers_from_pairs(&[("Content-Type", "application/json")])?;
    let headers = add_auth_header(headers, settings.api_key.as_deref())?;
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

fn add_auth_header(
    mut headers: reqwest::header::HeaderMap,
    api_key: Option<&str>,
) -> AppResult<reqwest::header::HeaderMap> {
    if let Some(key) = api_key {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {key}"))
                .map_err(|err| AppError::Config(err.to_string()))?,
        );
    }
    Ok(headers)
}
