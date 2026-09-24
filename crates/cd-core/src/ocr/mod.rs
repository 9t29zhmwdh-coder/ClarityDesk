use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;

use base64::Engine;

use crate::{
    error::{CdError, Result},
    models::{
        analysis::{BlockType, TextBlock},
        capture::BoundingBox,
    },
};

/// Places Tesseract lands when installed by Homebrew, MacPorts or the Windows
/// installer. An app started from Finder gets a PATH without any of them.
const TESSERACT_CANDIDATES: [&str; 4] = [
    "/opt/homebrew/bin/tesseract",
    "/usr/local/bin/tesseract",
    "/opt/local/bin/tesseract",
    r"C:\Program Files\Tesseract-OCR\tesseract.exe",
];

pub struct OcrEngine {
    pub lang: String,
}

impl OcrEngine {
    pub fn new(lang: impl Into<String>) -> Self {
        Self { lang: lang.into() }
    }

    /// Runs Tesseract with the image on stdin and hOCR on stdout, so no capture
    /// ever touches the disk.
    pub fn extract_text(&self, png_b64: &str) -> Result<Vec<TextBlock>> {
        let png = base64::engine::general_purpose::STANDARD
            .decode(png_b64)
            .map_err(|e| CdError::Ocr(format!("Base64 decode failed: {e}")))?;
        let binary = tesseract_path()?;
        let langs = usable_languages(&binary, &self.lang)?;
        let hocr = run_tesseract(&binary, &langs, &png)?;
        parse_hocr(&hocr)
    }
}

fn run_tesseract(binary: &PathBuf, langs: &str, png: &[u8]) -> Result<String> {
    let mut child = Command::new(binary)
        .args(["stdin", "stdout", "-l", langs, "hocr"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| CdError::Ocr(format!("could not start tesseract: {e}")))?;
    child.stdin.take().expect("stdin is piped").write_all(png)?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr);
        return Err(CdError::Ocr(format!("tesseract failed: {}", detail.trim())));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn tesseract_path() -> Result<PathBuf> {
    static FOUND: OnceLock<Option<PathBuf>> = OnceLock::new();
    FOUND
        .get_or_init(find_tesseract)
        .clone()
        .ok_or_else(|| CdError::Ocr("Tesseract is not installed. macOS: brew install tesseract tesseract-lang".into()))
}

fn find_tesseract() -> Option<PathBuf> {
    let on_path = Command::new("tesseract").arg("--version").output().is_ok();
    if on_path {
        return Some(PathBuf::from("tesseract"));
    }
    TESSERACT_CANDIDATES.iter().map(PathBuf::from).find(|p| p.is_file())
}

/// Keeps the requested languages that are installed. Homebrew's `tesseract`
/// ships English only; asking for "eng+deu" without `tesseract-lang` made
/// Tesseract fail and the app report an empty screen.
fn usable_languages(binary: &PathBuf, requested: &str) -> Result<String> {
    let installed = installed_languages(binary);
    let usable = filter_languages(requested, &installed);
    if usable.is_empty() {
        return Err(CdError::Ocr(format!(
            "none of the OCR languages {requested} is installed (installed: {}). macOS: brew install tesseract-lang",
            installed.join(", ")
        )));
    }
    if usable != requested {
        tracing::warn!("OCR languages {requested} requested, using {usable}");
    }
    Ok(usable)
}

fn installed_languages(binary: &PathBuf) -> Vec<String> {
    let Ok(output) = Command::new(binary).arg("--list-langs").output() else {
        return Vec::new();
    };
    // The first line is a heading ("List of available languages in ...").
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn filter_languages(requested: &str, installed: &[String]) -> String {
    requested
        .split('+')
        .map(str::trim)
        .filter(|lang| installed.iter().any(|i| i == lang))
        .collect::<Vec<_>>()
        .join("+")
}

/// Reads paragraphs from Tesseract's hOCR, one text block each, keeping line
/// breaks. Tesseract 5 writes some attributes with double quotes and some with
/// single ones, so every pattern accepts both.
fn parse_hocr(hocr: &str) -> Result<Vec<TextBlock>> {
    let re = |pattern: &str| regex_lite::Regex::new(pattern).map_err(|e| CdError::Ocr(e.to_string()));
    let paragraph = re(r#"class=['"]ocr_par['"][^>]*title=['"]bbox (\d+) (\d+) (\d+) (\d+)[^>]*>([\s\S]*?)</p>"#)?;
    let line_start = re(r#"<span class=['"]ocr_(?:line|caption|textfloat|header)['"]"#)?;
    let word = re(r#"class=['"]ocrx_word['"][^>]*>([\s\S]*?)</span>"#)?;
    let tag = re(r"<[^>]+>")?;

    let mut blocks = Vec::new();
    for par in paragraph.captures_iter(hocr) {
        let text = line_start
            .split(&par[5])
            .map(|fragment| words_in(fragment, &word, &tag))
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        if text.is_empty() {
            continue;
        }
        let number = |i: usize| par[i].parse::<i32>().unwrap_or(0);
        let (x1, y1, x2, y2) = (number(1), number(2), number(3), number(4));
        let bbox = BoundingBox { x: x1, y: y1, width: (x2 - x1).max(0) as u32, height: (y2 - y1).max(0) as u32 };
        blocks.push(TextBlock::new(text.clone(), classify_text(&text), 1.0, bbox));
    }
    Ok(blocks)
}

fn words_in(fragment: &str, word: &regex_lite::Regex, tag: &regex_lite::Regex) -> String {
    word.captures_iter(fragment)
        .map(|w| decode_entities(tag.replace_all(&w[1], "").trim()))
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn decode_entities(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
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

#[cfg(test)]
mod hocr_tests {
    use super::*;

    const TESSERACT_5: &str = include_str!("../../tests/fixtures/terminal-tesseract-5.5.hocr");

    #[test]
    fn reads_tesseract_5_output_with_its_mixed_quotes() {
        let blocks = parse_hocr(TESSERACT_5).unwrap();
        assert!(!blocks.is_empty(), "no paragraph found: the quote style is not understood");
        let all: String = blocks.iter().map(|b| b.text.as_str()).collect::<Vec<_>>().join("\n");
        assert!(all.contains("cargo build"));
        assert!(all.contains("E0382"));
    }

    #[test]
    fn keeps_lines_and_decodes_entities() {
        let blocks = parse_hocr(TESSERACT_5).unwrap();
        let first = &blocks[0];
        assert!(first.text.lines().count() > 1, "lines were glued together: {:?}", first.text);
        let all: String = blocks.iter().map(|b| b.text.clone()).collect();
        assert!(!all.contains("&lt;") && !all.contains("&amp;") && !all.contains("&quot;"));
    }

    #[test]
    fn the_terminal_capture_is_classified_as_terminal() {
        let blocks = parse_hocr(TESSERACT_5).unwrap();
        assert_eq!(blocks[0].block_type, BlockType::Terminal);
    }

    #[test]
    fn missing_languages_are_dropped_not_fatal() {
        let installed = vec!["eng".to_string(), "osd".to_string()];
        assert_eq!(filter_languages("eng+deu", &installed), "eng");
        assert_eq!(filter_languages("deu", &installed), "");
    }
}
