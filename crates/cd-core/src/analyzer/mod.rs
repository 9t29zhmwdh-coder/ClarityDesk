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

    /// Joins the blocks of a capture into as few prompts as fit.
    ///
    /// One model call per OCR paragraph made a screenful of text take minutes, and
    /// a compiler error split into command, code, file path and summary was
    /// explained piece by piece without the error itself. In Dev mode the whole
    /// screen is one story; otherwise neighbours of the same kind are joined. A
    /// group closes at `max_chars`, and at most `max_groups` are returned, the rest
    /// of a very long capture is left out rather than keeping the person waiting.
    pub fn merge_for_prompts(
        &self,
        blocks: &[TextBlock],
        mode: &AnalysisMode,
        max_chars: usize,
        max_groups: usize,
    ) -> Vec<TextBlock> {
        let whole_screen = *mode != AnalysisMode::Smart;
        let mut groups: Vec<TextBlock> = Vec::new();
        for block in blocks.iter().filter(|b| !b.text.trim().is_empty()) {
            match groups.last_mut() {
                Some(last) if fits(last, block, whole_screen, max_chars) => append(last, block),
                _ => groups.push(block.clone()),
            }
        }
        groups.retain(|g| g.text.split_whitespace().count() >= 2);
        groups.truncate(max_groups);
        groups
    }
}

fn fits(group: &TextBlock, block: &TextBlock, whole_screen: bool, max_chars: usize) -> bool {
    let same_kind = whole_screen || type_key(&group.block_type) == type_key(&block.block_type);
    same_kind && group.text.len() + block.text.len() < max_chars
}

fn append(group: &mut TextBlock, block: &TextBlock) {
    group.text.push_str("\n\n");
    group.text.push_str(&block.text);
    group.line_count += block.line_count;
    // Terminal output explains best with the terminal prompt, even if code follows.
    if matches!(block.block_type, BlockType::Terminal) {
        group.block_type = BlockType::Terminal;
    }
}

impl Default for ContentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Code, terminal output and log lines on one screen belong to the same story:
/// a command, the code it points at, the error it printed. Split apart, the model
/// explained "1 previous error" without the error. Everything else is plain text.
fn type_key(t: &BlockType) -> &'static str {
    match t {
        BlockType::Code { .. } | BlockType::Terminal | BlockType::Log => "dev",
        _ => "text",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::capture::BoundingBox;

    fn block(text: &str, block_type: BlockType) -> TextBlock {
        let bbox = BoundingBox { x: 0, y: 0, width: 10, height: 10 };
        TextBlock::new(text.into(), block_type, 1.0, bbox)
    }

    #[test]
    fn neighbours_of_the_same_kind_become_one_prompt() {
        let blocks = vec![
            block("Welcome back", BlockType::Header),
            block("Your order has shipped today.", BlockType::Paragraph),
            block("$ cargo build", BlockType::Terminal),
            block("error[E0425]: cannot find value", BlockType::Terminal),
            block("Thanks for reading", BlockType::Paragraph),
        ];
        let groups = ContentAnalyzer::new().merge_for_prompts(&blocks, &AnalysisMode::Smart, 3000, 10);
        assert_eq!(groups.len(), 3);
        assert!(groups[0].text.contains("Welcome back") && groups[0].text.contains("shipped"));
        assert!(groups[1].text.contains("cargo build") && groups[1].text.contains("E0425"));
    }

    #[test]
    fn a_command_its_code_and_its_error_stay_together() {
        let blocks = vec![
            block("$ cargo build", BlockType::Terminal),
            block("let first = names;", BlockType::Code { lang_hint: Some("rust".into()) }),
            block("error: could not compile demo", BlockType::Log),
        ];
        let groups = ContentAnalyzer::new().merge_for_prompts(&blocks, &AnalysisMode::Smart, 3000, 10);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].block_type, BlockType::Terminal);
    }

    #[test]
    fn in_dev_mode_a_file_path_line_does_not_split_the_error() {
        let blocks = vec![
            block("error[E0382]: borrow of moved value", BlockType::Terminal),
            block("--> src/main.rs:7:20", BlockType::Paragraph),
            block("let first = names;", BlockType::Code { lang_hint: None }),
        ];
        let groups = ContentAnalyzer::new().merge_for_prompts(&blocks, &AnalysisMode::Dev, 3000, 10);
        assert_eq!(groups.len(), 1);
        assert!(groups[0].text.contains("src/main.rs"));
    }

    #[test]
    fn short_errors_are_kept_when_they_join_a_group() {
        let blocks = vec![block("Permission denied", BlockType::Terminal)];
        assert_eq!(ContentAnalyzer::new().merge_for_prompts(&blocks, &AnalysisMode::Smart, 3000, 10).len(), 1);
    }

    #[test]
    fn long_captures_are_split_and_capped() {
        let long = "word ".repeat(200);
        let blocks: Vec<_> = (0..20).map(|_| block(&long, BlockType::Paragraph)).collect();
        let groups = ContentAnalyzer::new().merge_for_prompts(&blocks, &AnalysisMode::Language, 3000, 4);
        assert_eq!(groups.len(), 4);
        assert!(groups.iter().all(|g| g.text.len() < 3000 + long.len()));
    }
}
