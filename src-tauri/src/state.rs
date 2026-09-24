use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

use cd_core::{
    models::{analysis::AnalysisResult, capture::CaptureFrame, settings::Settings},
    ocr::OcrEngine,
    profiles::{self, AppProfile},
    semantic::SemanticEngine,
};

pub struct AppState {
    pub settings: Mutex<Settings>,
    pub settings_path: PathBuf,
    pub profiles: Vec<AppProfile>,
    pub profiles_dir: PathBuf,
    pub last_frame: Mutex<Option<CaptureFrame>>,
    pub last_result: Mutex<Option<AnalysisResult>>,
    pub ocr: Mutex<OcrEngine>,
    pub engine: Mutex<SemanticEngine>,
    pub is_analyzing: AtomicBool,
}

impl AppState {
    /// Settings live in `<config dir>/settings.json`, user app profiles in `<config dir>/profiles/`.
    pub fn new(config_dir: PathBuf) -> Arc<Self> {
        let settings_path = config_dir.join("settings.json");
        let profiles_dir = config_dir.join("profiles");
        let settings = Settings::load(&settings_path);
        let ocr = OcrEngine::new(&settings.ocr_language);
        let engine = SemanticEngine::new(&settings.ollama_host, &settings.ollama_model);

        Arc::new(Self {
            profiles: profiles::load(&profiles_dir),
            profiles_dir,
            settings: Mutex::new(settings),
            settings_path,
            last_frame: Mutex::new(None),
            last_result: Mutex::new(None),
            ocr: Mutex::new(ocr),
            engine: Mutex::new(engine),
            is_analyzing: AtomicBool::new(false),
        })
    }

    /// Claims the single analysis slot; the returned guard frees it when dropped,
    /// also when the analysis fails half way.
    pub fn begin_analysis(&self) -> Option<AnalysisGuard<'_>> {
        self.is_analyzing
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .ok()
            .map(|_| AnalysisGuard(&self.is_analyzing))
    }
}

pub struct AnalysisGuard<'a>(&'a AtomicBool);

impl Drop for AnalysisGuard<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}
