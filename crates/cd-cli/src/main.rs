use anyhow::Result;
use clap::{Parser, Subcommand};
use cd_core::{
    capture,
    ocr::OcrEngine,
    semantic::SemanticEngine,
    models::analysis::AnalysisMode,
};

#[derive(Parser)]
#[command(name = "claritydesk", about = "ClarityDesk CLI: Display Interpreter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture current screen and analyze it
    Capture {
        #[arg(long, default_value = "smart")]
        mode: String,
        #[arg(long, default_value = "Deutsch")]
        lang: String,
        #[arg(long, default_value = "http://localhost:11434")]
        ollama: String,
        #[arg(long, default_value = "llama3.2")]
        model: String,
    },
    /// Translate a text string
    Translate {
        text: String,
        #[arg(long, default_value = "Deutsch")]
        lang: String,
        #[arg(long, default_value = "http://localhost:11434")]
        ollama: String,
        #[arg(long, default_value = "llama3.2")]
        model: String,
    },
    /// Check Ollama connection
    Status {
        #[arg(long, default_value = "http://localhost:11434")]
        ollama: String,
        #[arg(long, default_value = "llama3.2")]
        model: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Capture { mode, lang, ollama, model } => {
            eprintln!("Capturing screen...");
            let frame = capture::capture_primary()?;
            eprintln!("OCR extraction...");
            let ocr = OcrEngine::new("eng+deu");
            let blocks = ocr.extract_text(&frame.image_png_b64)?;
            eprintln!("Found {} blocks. Analyzing with Ollama...", blocks.len());

            let analysis_mode = match mode.as_str() {
                "language" => AnalysisMode::Language,
                "dev"      => AnalysisMode::Dev,
                _          => AnalysisMode::Smart,
            };

            let engine = SemanticEngine::new(ollama, model);
            let result = engine.analyze(&frame.id, blocks, analysis_mode, &lang).await?;

            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::Translate { text, lang, ollama, model } => {
            let engine = SemanticEngine::new(ollama, model);
            let prompt = cd_core::semantic::prompts::language_prompt(&text, &lang);
            // Direct generate via the engine's internal client
            // For CLI simplicity, re-use the analyze path with a single fake block
            use cd_core::models::analysis::{TextBlock, BlockType};
            use cd_core::models::capture::BoundingBox;
            let block = TextBlock::new(
                text,
                BlockType::Paragraph,
                1.0,
                BoundingBox { x: 0, y: 0, width: 800, height: 100 },
            );
            let _ = prompt; // prompt already built inside engine
            let result = engine.analyze("cli", vec![block], AnalysisMode::Language, &lang).await?;
            for b in &result.blocks {
                println!("{}", b.output);
            }
        }
        Commands::Status { ollama, model } => {
            let engine = SemanticEngine::new(ollama, model);
            let status = engine.status().await;
            println!("{}", serde_json::to_string_pretty(&status)?);
        }
    }

    Ok(())
}
