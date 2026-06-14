use std::sync::Arc;
use tokio::sync::Mutex;

use cd_core::{
    models::{analysis::AnalysisResult, capture::CaptureFrame, settings::Settings},
    ocr::OcrEngine,
    semantic::SemanticEngine,
};

pub struct AppState {
    pub settings: Mutex<Settings>,
    pub last_frame: Mutex<Option<CaptureFrame>>,
    pub last_result: Mutex<Option<AnalysisResult>>,
    pub ocr: Mutex<OcrEngine>,
    pub engine: Mutex<SemanticEngine>,
    pub is_analyzing: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        let settings = Settings::default();
        let ocr = OcrEngine::new(&settings.ocr_language);
        let engine = SemanticEngine::new(&settings.ollama_host, &settings.ollama_model);

        Arc::new(Self {
            settings: Mutex::new(settings),
            last_frame: Mutex::new(None),
            last_result: Mutex::new(None),
            ocr: Mutex::new(ocr),
            engine: Mutex::new(engine),
            is_analyzing: Mutex::new(false),
        })
    }
}
