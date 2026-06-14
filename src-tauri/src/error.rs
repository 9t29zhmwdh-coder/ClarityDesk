use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Core error: {0}")]
    Core(#[from] cd_core::CdError),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
