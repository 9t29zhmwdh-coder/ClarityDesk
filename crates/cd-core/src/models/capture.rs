use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CaptureSource {
    FullScreen { index: usize },
    ActiveWindow,
    Region { x: i32, y: i32, width: u32, height: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenInfo {
    pub index: usize,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureFrame {
    pub id: String,
    pub image_png_b64: String,
    pub width: u32,
    pub height: u32,
    pub captured_at: DateTime<Utc>,
    pub source: CaptureSource,
}

impl CaptureFrame {
    pub fn new(image_png: Vec<u8>, width: u32, height: u32, source: CaptureSource) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            image_png_b64: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &image_png),
            width,
            height,
            captured_at: Utc::now(),
            source,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
