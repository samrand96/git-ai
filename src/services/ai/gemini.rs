use serde_json::json;

use crate::config::ProviderSettings;
use crate::utils::errors::{AppError, AppResult};
use crate::utils::http::{HttpClient, headers_from_pairs};

use super::Message;
use super::messages::combine_messages;

pub(super) fn generate(
    http: &HttpClient,
    settings: &ProviderSettings,
    prompt: &str,
    messages: Option<&[Message]>,
) -> AppResult<String> {
    let model = settings.model.replace("models/", "");
    let url = format!(
        "{}/models/{}:generateContent?key={}",
        settings.base_url.trim_end_matches('/'),
        model,
        settings.api_key.clone().unwrap_or_default()
    );

    let combined_prompt = combine_messages(prompt, messages);
    let body = json!({
        "contents": [
            {
                "role": "user",
                "parts": [{ "text": combined_prompt }]
            }
        ]
    });

    let headers = headers_from_pairs(&[("Content-Type", "application/json")])?;
    let response = http.post_json(&url, headers, &body)?;

    let text = response
        .get("candidates")
        .and_then(|c| c.as_array())
        .and_then(|c| c.first())
        .and_then(|candidate| candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.as_array())
        .and_then(|parts| parts.first())
        .and_then(|part| part.get("text"))
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
    let url = format!(
        "{}/models?key={}",
        settings.base_url.trim_end_matches('/'),
        settings.api_key.clone().unwrap_or_default()
    );
    let headers = headers_from_pairs(&[("Content-Type", "application/json")])?;
    let response = http.get_json(&url, headers)?;
    let models = response
        .get("models")
        .and_then(|data| data.as_array())
        .map(|data| {
            data.iter()
                .filter_map(|item| item.get("name").and_then(|id| id.as_str()))
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(models)
}
