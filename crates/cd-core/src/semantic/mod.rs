pub mod ollama;
pub mod prompts;

use crate::{
    analyzer::ContentAnalyzer,
    error::Result,
    models::analysis::{AnalysisMode, AnalysisResult, AnalyzedBlock, TextBlock},
};
use ollama::OllamaClient;
use prompts::{language_prompt, smart_prompt};

/// About two screen pages of text per model call.
const MAX_GROUP_CHARS: usize = 3000;
/// Upper bound on model calls per capture, so a huge page does not keep the person waiting.
const MAX_GROUPS: usize = 6;

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
            mode
        };

        let model = self.client.model.clone();
        let mut analyzed = Vec::new();

        for group in self.analyzer.merge_for_prompts(&blocks, &effective_mode, MAX_GROUP_CHARS, MAX_GROUPS) {
            let prompt = match &effective_mode {
                AnalysisMode::Language => language_prompt(&group.text, target_language),
                AnalysisMode::Dev | AnalysisMode::Smart => {
                    smart_prompt(&group.text, &group.block_type, target_language)
                }
            };
            let result = self.client.generate(&prompt).await;
            analyzed.push(AnalyzedBlock {
                block_id: group.id.clone(),
                original: group.text.clone(),
                output: result.as_ref().map(String::clone).unwrap_or_default(),
                block_type: group.block_type.clone(),
                error: result.err().map(|e| e.to_string()),
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