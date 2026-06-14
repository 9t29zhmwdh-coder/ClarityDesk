use thiserror::Error;

#[derive(Error, Debug)]
pub enum CdError {
    #[error("Capture failed: {0}")]
    Capture(String),

    #[error("OCR failed: {0}")]
    Ocr(String),

    #[error("Analyzer error: {0}")]
    Analyzer(String),

    #[error("Ollama error: {0}")]
    Ollama(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, CdError>;
