import { invoke } from "@tauri-apps/api/core";

// ── Types ────────────────────────────────────────────────────────────────────

export interface ScreenInfo {
  index: number;
  width: number;
  height: number;
  scaleFactor: number;
  isPrimary: boolean;
}

export interface BoundingBox {
  x: number;
  y: number;
  width: number;
  height: number;
}

export type CaptureSource =
  | { fullScreen: { index: number } }
  | { activeWindow: null }
  | { region: { x: number; y: number; width: number; height: number } };

export interface CaptureFrame {
  id: string;
  imagePngB64: string;
  width: number;
  height: number;
  capturedAt: string;
  source: CaptureSource;
}

export type BlockType =
  | { code: { langHint: string | null } }
  | { terminal: null }
  | { log: null }
  | { paragraph: null }
  | { header: null }
  | { table: null }
  | { ui: null }
  | { unknown: null };

export interface TextBlock {
  id: string;
  text: string;
  blockType: BlockType;
  confidence: number;
  bbox: BoundingBox;
  lineCount: number;
}

export type AnalysisMode = "language" | "dev" | "smart";

export interface AnalyzedBlock {
  blockId: string;
  original: string;
  output: string;
  blockType: BlockType;
  error: string | null;
}

export interface AnalysisResult {
  id: string;
  frameId: string;
  mode: AnalysisMode;
  targetLanguage: string | null;
  blocks: AnalyzedBlock[];
  totalBlocks: number;
  analyzedAt: string;
  modelUsed: string;
}

export interface HotkeyConfig {
  startStop: string;
  devMode: string;
  smartMode: string;
  reAnalyze: string;
}

export interface PrivacyConfig {
  storeCaptures: boolean;
  storeResults: boolean;
  appWhitelist: string[];
  showConsentOnStart: boolean;
}

export interface CaptureSettings {
  scaleFactor: number;
  includeCursor: boolean;
  delayMs: number;
}

export interface Settings {
  ollamaHost: string;
  ollamaModel: string;
  ocrLanguage: string;
  defaultMode: AnalysisMode;
  targetLanguage: string;
  hotkeys: HotkeyConfig;
  privacy: PrivacyConfig;
  capture: CaptureSettings;
}

export interface OllamaStatus {
  connected: boolean;
  version: string | null;
  availableModels: string[];
  host: string;
}

// ── API ──────────────────────────────────────────────────────────────────────

export const api = {
  // Capture
  listScreens: (): Promise<ScreenInfo[]> => invoke("list_screens"),
  capturePrimary: (): Promise<CaptureFrame> => invoke("capture_primary"),
  captureScreen: (index: number): Promise<CaptureFrame> => invoke("capture_screen", { index }),
  captureRegion: (x: number, y: number, width: number, height: number): Promise<CaptureFrame> =>
    invoke("capture_region", { x, y, width, height }),
  getLastFrame: (): Promise<CaptureFrame | null> => invoke("get_last_frame"),

  // Analysis
  analyzeLastFrame: (mode: AnalysisMode): Promise<AnalysisResult> =>
    invoke("analyze_last_frame", { mode }),
  analyzeBlocks: (
    frameId: string,
    blocks: TextBlock[],
    mode: AnalysisMode,
  ): Promise<AnalysisResult> => invoke("analyze_blocks", { frameId, blocks, mode }),
  extractText: (pngB64: string): Promise<TextBlock[]> => invoke("extract_text", { pngB64 }),
  getLastResult: (): Promise<AnalysisResult | null> => invoke("get_last_result"),
  isAnalyzing: (): Promise<boolean> => invoke("is_analyzing"),

  // Settings
  getSettings: (): Promise<Settings> => invoke("get_settings"),
  saveSettings: (settings: Settings): Promise<void> => invoke("save_settings", { settings }),
  checkOllama: (): Promise<OllamaStatus> => invoke("check_ollama"),
  getDefaultSettings: (): Promise<Settings> => invoke("get_default_settings"),
};

// ── Helpers ──────────────────────────────────────────────────────────────────

export function blockTypeLabel(bt: BlockType): string {
  if ("code" in bt) return "Code";
  if ("terminal" in bt) return "Terminal";
  if ("log" in bt) return "Log";
  if ("paragraph" in bt) return "Text";
  if ("header" in bt) return "Header";
  if ("table" in bt) return "Table";
  if ("ui" in bt) return "UI";
  return "Unknown";
}

export function blockTypeBadgeClass(bt: BlockType): string {
  if ("code" in bt) return "badge-code";
  if ("terminal" in bt) return "badge-terminal";
  if ("log" in bt) return "badge-log";
  if ("paragraph" in bt) return "badge-text";
  if ("header" in bt) return "badge-header";
  if ("table" in bt) return "badge-table";
  if ("ui" in bt) return "badge-ui";
  return "badge-text";
}
