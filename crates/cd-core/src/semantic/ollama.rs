use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{CdError, Result};
use crate::models::settings::OllamaStatus;

#[derive(Debug, Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    // Qwen models reason out loud by default; that text would end up in the answer.
    think: bool,
    options: GenerateOptions,
}

/// Without num_ctx Ollama reserves the model's full context window: in LifeSort
/// qwen3.5:4b took 12.5 GB instead of 4. A capture group is capped well below this.
#[derive(Debug, Serialize)]
struct GenerateOptions {
    num_ctx: u32,
    temperature: f32,
}

const CONTEXT_TOKENS: u32 = 8192;
const TEMPERATURE: f32 = 0.2;

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
    pub host: String,
    pub model: String,
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
            think: false,
            options: GenerateOptions { num_ctx: CONTEXT_TOKENS, temperature: TEMPERATURE },
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

        let version = match self.http.get(&version_url).send().await {
            Ok(resp) => resp.json::<VersionResponse>().await.ok().map(|v| v.version),
            Err(_) => None,
        };

        let models = match self.http.get(&tags_url).send().await {
            Ok(resp) => resp
                .json::<TagsResponse>()
                .await
                .ok()
                .map(|t| t.models.into_iter().map(|m| m.name).collect::<Vec<_>>())
                .unwrap_or_default(),
            Err(_) => Vec::new(),
        };

        OllamaStatus {
            connected: version.is_some(),
            model_installed: is_installed(&models, &self.model),
            version,
            available_models: models,
            host: self.host.clone(),
            model: self.model.clone(),
        }
    }
}

/// Ollama lists "llama3.2:latest" for a model pulled as "llama3.2".
fn is_installed(models: &[String], wanted: &str) -> bool {
    models
        .iter()
        .any(|m| m == wanted || m.strip_suffix(":latest") == Some(wanted))
}
