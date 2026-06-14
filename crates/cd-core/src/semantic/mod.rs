pub mod ollama;
pub mod prompts;

use crate::{
    analyzer::ContentAnalyzer,
    error::Result,
    models::analysis::{AnalysisMode, AnalysisResult, AnalyzedBlock, TextBlock},
};
use ollama::OllamaClient;
use prompts::{language_prompt, smart_prompt};

pub struct SemanticEngine {
    client: OllamaClient,
    analyzer: ContentAnalyzer,
}

impl SemanticEngine {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: OllamaClient::new(host, model.into()),
            analyzer: ContentAnalyzer::new(),
        }
    }

    pub fn update_model(&mut self, model: impl Into<String>) {
        self.client = OllamaClient::new(self.client_host(), model);
    }

    pub fn update_host(&mut self, host: impl Into<String>) {
        let model = self.client.model.clone();
        self.client = OllamaClient::new(host, model);
    }

    fn client_host(&self) -> String {
        self.client.host.clone()
    }

    pub async fn analyze(
        &self,
        frame_id: &str,
        blocks: Vec<TextBlock>,
        mode: AnalysisMode,
        target_language: &str,
    ) -> Result<AnalysisResult> {
        let effective_mode = if mode == AnalysisMode::Smart {
            self.analyzer.infer_mode(&blocks)
        } else {
            mode.clone()
        };

        let model = self.client.model.clone();
        let mut analyzed = Vec::new();

        for block in &blocks {
            if block.text.trim().split_whitespace().count() < 3 {
                continue;
            }

            let prompt = match &effective_mode {
                AnalysisMode::Language => {
                    language_prompt(&block.text, target_language)
                }
                AnalysisMode::Dev => {
                    smart_prompt(&block.text, &block.block_type, target_language)
                }
                AnalysisMode::Smart => {
                    smart_prompt(&block.text, &block.block_type, target_language)
                }
            };

            let result = self.client.generate(&prompt).await;
            analyzed.push(AnalyzedBlock {
                block_id: block.id.clone(),
                original: block.text.clone(),
                output: result.unwrap_or_else(|e| format!("[Error: {e}]")),
                block_type: block.block_type.clone(),
                error: None,
            });
        }

        Ok(AnalysisResult::new(
            frame_id.to_string(),
            mode,
            Some(target_language.to_string()),
            analyzed,
            model,
        ))
    }

    pub async fn status(&self) -> crate::models::settings::OllamaStatus {
        self.client.status().await
    }
}
