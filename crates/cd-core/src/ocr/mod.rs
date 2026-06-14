use std::process::Command;
use std::io::Write;
use base64::Engine;

use crate::{
    error::{CdError, Result},
    models::{
        analysis::{BlockType, TextBlock},
        capture::BoundingBox,
    },
};

pub struct OcrEngine {
    pub lang: String,
}

impl OcrEngine {
    pub fn new(lang: impl Into<String>) -> Self {
        Self { lang: lang.into() }
    }

    pub fn extract_text(&self, png_b64: &str) -> Result<Vec<TextBlock>> {
        let png_bytes = base64::engine::general_purpose::STANDARD
            .decode(png_b64)
            .map_err(|e| CdError::Ocr(format!("Base64 decode failed: {e}")))?;

        let mut tmp_in = tempfile::Builder::new()
            .suffix(".png")
            .tempfile()
            .map_err(|e| CdError::Ocr(e.to_string()))?;
        tmp_in.write_all(&png_bytes)?;
        let in_path = tmp_in.path().to_path_buf();

        let tmp_out = tempfile::Builder::new()
            .suffix("")
            .tempfile()
            .map_err(|e| CdError::Ocr(e.to_string()))?;
        let out_base = tmp_out.path().to_str().unwrap().to_string();

        let status = Command::new("tesseract")
            .arg(&in_path)
            .arg(&out_base)
            .arg("-l")
            .arg(&self.lang)
            .arg("hocr")
            .status()
            .map_err(|e| CdError::Ocr(format!("tesseract binary not found: {e}")))?;

        if !status.success() {
            return Err(CdError::Ocr("tesseract exited with error".into()));
        }

        let hocr_path = format!("{out_base}.hocr");
        let hocr = std::fs::read_to_string(&hocr_path)
            .map_err(|e| CdError::Ocr(format!("HOCR read failed: {e}")))?;
        let _ = std::fs::remove_file(&hocr_path);

        parse_hocr(&hocr)
    }
}

fn parse_hocr(hocr: &str) -> Result<Vec<TextBlock>> {
    // Extract ocr_par blocks — paragraph-level grouping
    let mut blocks = Vec::new();
    let par_re = regex_lite::Regex::new(r#"class='ocr_par'[^>]*title='bbox (\d+) (\d+) (\d+) (\d+)[^']*'[^>]*>([\s\S]*?)</p>"#)
        .map_err(|e| CdError::Ocr(e.to_string()))?;
    let word_re = regex_lite::Regex::new(r#"class='ocrx_word'[^>]*>([\s\S]*?)</span>"#)
        .map_err(|e| CdError::Ocr(e.to_string()))?;
    let tag_re = regex_lite::Regex::new(r"<[^>]+>")
        .map_err(|e| CdError::Ocr(e.to_string()))?;

    for cap in par_re.captures_iter(hocr) {
        let x1: i32 = cap[1].parse().unwrap_or(0);
        let y1: i32 = cap[2].parse().unwrap_or(0);
        let x2: i32 = cap[3].parse().unwrap_or(0);
        let y2: i32 = cap[4].parse().unwrap_or(0);
        let inner = &cap[5];

        // Collect words
        let mut words = Vec::new();
        for wc in word_re.captures_iter(inner) {
            let word = tag_re.replace_all(&wc[1], "").trim().to_string();
            if !word.is_empty() {
                words.push(word);
            }
        }
        let text = words.join(" ");
        if text.trim().is_empty() {
            continue;
        }

        let bbox = BoundingBox {
            x: x1,
            y: y1,
            width: (x2 - x1).max(0) as u32,
            height: (y2 - y1).max(0) as u32,
        };

        let block_type = classify_text(&text);
        blocks.push(TextBlock::new(text, block_type, 1.0, bbox));
    }

    Ok(blocks)
}

// Heuristic content classification
fn classify_text(text: &str) -> BlockType {
    let trimmed = text.trim();
    let lines: Vec<&str> = trimmed.lines().collect();
    let first_line = lines.first().copied().unwrap_or("");

    // Terminal prompt patterns
    if first_line.contains("$ ") || first_line.contains("% ") || first_line.contains("# ") {
        return BlockType::Terminal;
    }

    // Log patterns: timestamps, log levels
    let log_re = regex_lite::Regex::new(
        r"(?i)\b(debug|info|warn(?:ing)?|error|fatal|critical|trace)\b|\d{4}-\d{2}-\d{2}T?\d{2}:\d{2}",
    );
    if let Ok(re) = log_re {
        if lines.iter().filter(|l| re.is_match(l)).count() as f32 / lines.len() as f32 > 0.4 {
            return BlockType::Log;
        }
    }

    // Code patterns: braces, semicolons, keywords
    let code_keywords = ["fn ", "def ", "class ", "import ", "use ", "let ", "const ",
                         "var ", "if (", "for (", "while (", "return ", "pub ", "func "];
    let code_score = lines
        .iter()
        .filter(|l| code_keywords.iter().any(|k| l.contains(k)) || l.contains('{') || l.contains('}'))
        .count();
    if code_score > 1 || (code_score == 1 && lines.len() <= 3) {
        let lang = detect_lang(trimmed);
        return BlockType::Code { lang_hint: lang };
    }

    // Header: short single line, often title-case or ALL CAPS
    if lines.len() == 1 && trimmed.len() < 80 {
        let upper_count = trimmed.chars().filter(|c| c.is_uppercase()).count();
        let alpha_count = trimmed.chars().filter(|c| c.is_alphabetic()).count();
        if alpha_count > 0 && upper_count as f32 / alpha_count as f32 > 0.5 {
            return BlockType::Header;
        }
    }

    // Table: contains pipe characters
    if lines.iter().filter(|l| l.contains('|')).count() > 1 {
        return BlockType::Table;
    }

    BlockType::Paragraph
}

fn detect_lang(text: &str) -> Option<String> {
    if text.contains("fn ") && text.contains("let ") { return Some("rust".into()); }
    if text.contains("def ") && text.contains(":") { return Some("python".into()); }
    if text.contains("function") || text.contains("const ") || text.contains("=>") {
        return Some("javascript".into());
    }
    if text.contains("class ") && text.contains("{") { return Some("java".into()); }
    if text.contains("#include") { return Some("c".into()); }
    if text.contains("<div") || text.contains("</") { return Some("html".into()); }
    None
}
