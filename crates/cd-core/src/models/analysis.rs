use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::capture::BoundingBox;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnalysisMode {
    Language,
    Dev,
    Smart,
}

impl std::fmt::Display for AnalysisMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Language => write!(f, "language"),
            Self::Dev      => write!(f, "dev"),
            Self::Smart    => write!(f, "smart"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BlockType {
    Code { lang_hint: Option<String> },
    Terminal,
    Log,
    Paragraph,
    Header,
    Table,
    Ui,
    Unknown,
}

impl BlockType {
    pub fn label(&self) -> &str {
        match self {
            Self::Code { .. }  => "Code",
            Self::Terminal     => "Terminal",
            Self::Log          => "Log",
            Self::Paragraph    => "Text",
            Self::Header       => "Header",
            Self::Table        => "Table",
            Self::Ui           => "UI",
            Self::Unknown      => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextBlock {
    pub id: String,
    pub text: String,
    pub block_type: BlockType,
    pub confidence: f32,
    pub bbox: BoundingBox,
    pub line_count: usize,
}

impl TextBlock {
    pub fn new(text: String, block_type: BlockType, confidence: f32, bbox: BoundingBox) -> Self {
        let line_count = text.lines().count();
        Self {
            id: Uuid::new_v4().to_string(),
            text,
            block_type,
            confidence,
            bbox,
            line_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzedBlock {
    pub block_id: String,
    pub original: String,
    pub output: String,
    pub block_type: BlockType,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResult {
    pub id: String,
    pub frame_id: String,
    pub mode: AnalysisMode,
    pub target_language: Option<String>,
    pub blocks: Vec<AnalyzedBlock>,
    pub total_blocks: usize,
    pub analyzed_at: DateTime<Utc>,
    pub model_used: String,
}

impl AnalysisResult {
    pub fn new(
        frame_id: String,
        mode: AnalysisMode,
        target_language: Option<String>,
        blocks: Vec<AnalyzedBlock>,
        model_used: String,
    ) -> Self {
        let total_blocks = blocks.len();
        Self {
            id: Uuid::new_v4().to_string(),
            frame_id,
            mode,
            target_language,
            total_blocks,
            blocks,
            analyzed_at: Utc::now(),
            model_used,
        }
    }
}
