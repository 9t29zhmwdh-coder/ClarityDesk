use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{CdError, Result};
use crate::models::settings::OllamaStatus;

#[derive(Debug, Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Debug, Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Debug, Deserialize)]
struct ModelInfo {
    name: String,
}

#[derive(Debug, Deserialize)]
struct VersionResponse {
    version: String,
}

pub struct OllamaClient {
    host: String,
    model: String,
    http: Client,
}

impl OllamaClient {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            model: model.into(),
            http: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("HTTP client"),
        }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.host);
        let body = GenerateRequest {
            model: &self.model,
            prompt,
            stream: false,
        };

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CdError::Ollama(format!("Request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(CdError::Ollama(format!("HTTP {status}: {text}")));
        }

        let gen: GenerateResponse = resp
            .json()
            .await
            .map_err(|e| CdError::Ollama(format!("Parse error: {e}")))?;

        Ok(gen.response.trim().to_string())
    }

    pub async fn status(&self) -> OllamaStatus {
        let version_url = format!("{}/api/version", self.host);
        let tags_url = format!("{}/api/tags", self.host);

        let version = self
            .http
            .get(&version_url)
            .send()
            .await
            .ok()
            .and_then(|r| r.json::<VersionResponse>().ok().map(|v| v.version))
            .await;

        let models = self
            .http
            .get(&tags_url)
            .send()
            .await
            .ok()
            .and_then(|r| r.json::<TagsResponse>().ok())
            .map(|t| t.models.into_iter().map(|m| m.name).collect::<Vec<_>>())
            .await
            .unwrap_or_default();

        OllamaStatus {
            connected: version.is_some(),
            version,
            available_models: models,
            host: self.host.clone(),
        }
    }

    pub fn with_model(&self, model: impl Into<String>) -> Self {
        Self {
            host: self.host.clone(),
            model: model.into(),
            http: self.http.clone(),
        }
    }
}
