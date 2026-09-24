use anyhow::{bail, Result};
use base64::Engine as _;
use cd_core::{
    capture,
    models::{
        analysis::{AnalysisMode, AnalysisResult, BlockType, TextBlock},
        capture::BoundingBox,
        settings::{default_model, Settings},
    },
    ocr::OcrEngine,
    semantic::SemanticEngine,
};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claritydesk", about = "ClarityDesk CLI: reads text on screen or in an image and explains it")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Model settings shared by every command; the defaults match the app.
#[derive(Args)]
struct ModelArgs {
    /// Ollama address
    #[arg(long, default_value = "http://localhost:11434")]
    ollama: String,
    /// Ollama model
    #[arg(long, default_value_t = default_model().to_string())]
    model: String,
    /// Language for translations
    #[arg(long, default_value = "English")]
    lang: String,
}

#[derive(Args)]
struct AnalyzeArgs {
    /// language, dev or smart
    #[arg(long, default_value = "smart")]
    mode: String,
    /// Tesseract languages, for example eng+deu
    #[arg(long, default_value_t = Settings::default().ocr_language)]
    ocr: String,
    /// Print the full result as JSON instead of text
    #[arg(long)]
    json: bool,
    #[command(flatten)]
    model: ModelArgs,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture the primary screen and analyze it
    Capture(AnalyzeArgs),
    /// Analyze a screenshot or photo from a file (PNG or JPEG)
    Image {
        path: std::path::PathBuf,
        #[command(flatten)]
        analyze: AnalyzeArgs,
    },
    /// Translate a text string
    Translate {
        text: String,
        #[command(flatten)]
        model: ModelArgs,
    },
    /// Check the Ollama connection and whether the model is installed
    Status {
        #[command(flatten)]
        model: ModelArgs,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Capture(args) => {
            eprintln!("Capturing the primary screen...");
            let frame = capture::capture_primary()?;
            analyze_png(&frame.image_png_b64, &frame.id, &args).await
        }
        Commands::Image { path, analyze } => {
            let png = to_png(&std::fs::read(&path)?)?;
            let encoded = base64::engine::general_purpose::STANDARD.encode(png);
            analyze_png(&encoded, &path.display().to_string(), &analyze).await
        }
        Commands::Translate { text, model } => translate(text, &model).await,
        Commands::Status { model } => {
            let status = SemanticEngine::new(model.ollama, model.model).status().await;
            println!("{}", serde_json::to_string_pretty(&status)?);
            Ok(())
        }
    }
}

/// Tesseract is handed PNG; a JPEG from a phone is converted first.
fn to_png(bytes: &[u8]) -> Result<Vec<u8>> {
    let image = image::load_from_memory(bytes)?;
    let mut png = std::io::Cursor::new(Vec::new());
    image.write_to(&mut png, image::ImageFormat::Png)?;
    Ok(png.into_inner())
}

fn parse_mode(mode: &str) -> Result<AnalysisMode> {
    match mode {
        "language" => Ok(AnalysisMode::Language),
        "dev" => Ok(AnalysisMode::Dev),
        "smart" => Ok(AnalysisMode::Smart),
        other => bail!("unknown mode {other}, use language, dev or smart"),
    }
}

async fn analyze_png(png_b64: &str, id: &str, args: &AnalyzeArgs) -> Result<()> {
    let mode = parse_mode(&args.mode)?;
    let blocks = OcrEngine::new(&args.ocr).extract_text(png_b64)?;
    eprintln!("Found {} text blocks. Asking {}...", blocks.len(), args.model.model);
    let engine = SemanticEngine::new(&args.model.ollama, &args.model.model);
    let result = engine.analyze(id, blocks, mode, &args.model.lang).await?;
    print_result(&result, args.json)
}

async fn translate(text: String, model: &ModelArgs) -> Result<()> {
    let bbox = BoundingBox { x: 0, y: 0, width: 800, height: 100 };
    let block = TextBlock::new(text, BlockType::Paragraph, 1.0, bbox);
    let engine = SemanticEngine::new(&model.ollama, &model.model);
    let result = engine.analyze("cli", vec![block], AnalysisMode::Language, &model.lang).await?;
    print_result(&result, false)
}

fn print_result(result: &AnalysisResult, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(result)?);
        return Ok(());
    }
    for block in &result.blocks {
        println!("── {} ──", block.block_type.label());
        match &block.error {
            Some(error) => println!("[error] {error}"),
            None => println!("{}", block.output),
        }
        println!();
    }
    if result.blocks.iter().any(|b| b.error.is_some()) {
        bail!("the model could not answer every block");
    }
    Ok(())
}
