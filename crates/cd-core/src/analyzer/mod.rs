use crate::models::analysis::{AnalysisMode, BlockType, TextBlock};

pub struct ContentAnalyzer;

impl ContentAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Determine the best AnalysisMode for a set of blocks when in Smart mode
    pub fn infer_mode(&self, blocks: &[TextBlock]) -> AnalysisMode {
        let total = blocks.len();
        if total == 0 {
            return AnalysisMode::Language;
        }

        let dev_count = blocks.iter().filter(|b| {
            matches!(
                b.block_type,
                BlockType::Code { .. } | BlockType::Terminal | BlockType::Log
            )
        }).count();

        if dev_count as f32 / total as f32 > 0.4 {
            AnalysisMode::Dev
        } else {
            AnalysisMode::Language
        }
    }

    /// Filter out blocks that are too short or likely noise
    pub fn filter_blocks<'a>(&self, blocks: &'a [TextBlock]) -> Vec<&'a TextBlock> {
        blocks
            .iter()
            .filter(|b| b.text.trim().split_whitespace().count() >= 3)
            .collect()
    }

    /// Group related blocks for context-aware analysis
    pub fn group_for_context<'a>(&self, blocks: &'a [TextBlock]) -> Vec<Vec<&'a TextBlock>> {
        let mut groups: Vec<Vec<&TextBlock>> = Vec::new();
        let mut current: Vec<&TextBlock> = Vec::new();
        let mut last_type: Option<&BlockType> = None;

        for block in blocks {
            let same_type = last_type
                .map(|t| type_key(t) == type_key(&block.block_type))
                .unwrap_or(true);

            if same_type || current.is_empty() {
                current.push(block);
            } else {
                groups.push(std::mem::take(&mut current));
                current.push(block);
            }
            last_type = Some(&block.block_type);
        }
        if !current.is_empty() {
            groups.push(current);
        }
        groups
    }
}

impl Default for ContentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn type_key(t: &BlockType) -> &'static str {
    match t {
        BlockType::Code { .. }  => "code",
        BlockType::Terminal     => "terminal",
        BlockType::Log          => "log",
        BlockType::Paragraph    => "text",
        BlockType::Header       => "header",
        BlockType::Table        => "table",
        BlockType::Ui           => "ui",
        BlockType::Unknown      => "unknown",
    }
}
