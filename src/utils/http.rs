use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;

use crate::utils::errors::{AppError, AppResult};

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(timeout: Duration) -> AppResult<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(AppError::from)?;
        Ok(Self { client })
    }

    pub fn post_json(&self, url: &str, headers: HeaderMap, body: &Value) -> AppResult<Value> {
        ensure_secure_url(url)?;
        let response = self.client.post(url).headers(headers).json(body).send()?;
        Self::handle_json_response(url, response)
    }

    pub fn get_json(&self, url: &str, headers: HeaderMap) -> AppResult<Value> {
        ensure_secure_url(url)?;
        let response = self.client.get(url).headers(headers).send()?;
        Self::handle_json_response(url, response)
    }

    pub fn post_text(&self, url: &str, headers: HeaderMap, body: &Value) -> AppResult<String> {
        ensure_secure_url(url)?;
        let response = self.client.post(url).headers(headers).json(body).send()?;
        Self::handle_text_response(url, response)
    }

    pub fn get_text(&self, url: &str, headers: HeaderMap) -> AppResult<String> {
        ensure_secure_url(url)?;
        let response = self.client.get(url).headers(headers).send()?;
        Self::handle_text_response(url, response)
    }

    fn handle_json_response(url: &str, response: reqwest::blocking::Response) -> AppResult<Value> {
        let status = response.status();
        let text = response.text()?;
        if !status.is_success() {
            return Err(AppError::HttpStatus {
                url: url.to_string(),
                status: status.to_string(),
                body: text,
            });
        }
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    fn handle_text_response(url: &str, response: reqwest::blocking::Response) -> AppResult<String> {
        let status = response.status();
        let text = response.text()?;
        if !status.is_success() {
            return Err(AppError::HttpStatus {
                url: url.to_string(),
                status: status.to_string(),
                body: text,
            });
        }
        Ok(text)
    }
}

pub fn headers_from_pairs(pairs: &[(&str, &str)]) -> AppResult<HeaderMap> {
    let mut headers = HeaderMap::new();
    for (key, value) in pairs {
        let name = HeaderName::from_bytes(key.as_bytes())
            .map_err(|err| AppError::Config(format!("Invalid header name {key}: {err}")))?;
        let value = HeaderValue::from_str(value)
            .map_err(|err| AppError::Config(format!("Invalid header value for {key}: {err}")))?;
        headers.insert(name, value);
    }
    Ok(headers)
}

fn ensure_secure_url(url: &str) -> AppResult<()> {
    let parsed = reqwest::Url::parse(url)
        .map_err(|err| AppError::Config(format!("Invalid URL '{url}': {err}")))?;
    if parsed.scheme() == "http" {
        let host = parsed.host_str().unwrap_or("");
        if !is_local_host(host) {
            return Err(AppError::Config(format!(
                "Refusing to send credentials over HTTP to non-local host '{host}'. Use HTTPS or a localhost endpoint."
            )));
        }
    }
    Ok(())
}

fn is_local_host(host: &str) -> bool {
    matches!(host, "localhost" | "127.0.0.1" | "::1" | "0.0.0.0")
}
