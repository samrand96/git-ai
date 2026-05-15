use serde_json::json;

use crate::config::ProviderSettings;
use crate::utils::errors::{AppError, AppResult};
use crate::utils::http::{HttpClient, headers_from_pairs};

pub(super) fn generate(
    http: &HttpClient,
    settings: &ProviderSettings,
    prompt: &str,
) -> AppResult<String> {
    let url = format!("{}/api/generate", settings.base_url.trim_end_matches('/'));
    let body = json!({
        "model": settings.model,
        "prompt": prompt,
        "stream": false
    });
    let headers = headers_from_pairs(&[("Content-Type", "application/json")])?;
    let response = http.post_json(&url, headers, &body)?;
    let text = response
        .get("response")
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
    let url = format!("{}/api/tags", settings.base_url.trim_end_matches('/'));
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
