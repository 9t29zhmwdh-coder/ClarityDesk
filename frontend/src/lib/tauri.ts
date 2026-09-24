import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

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
  | { activeWindow: { app: string } }
  | { region: { x: number; y: number; width: number; height: number } };

export interface CaptureFrame {
  id: string;
  imagePngB64: string;
  width: number;
  height: number;
  capturedAt: string;
  source: CaptureSource;
}

// serde's default enum form: unit variants arrive as a plain string ("terminal"),
// the one variant with data as an object ({"code": {"lang_hint": "rust"}}).
export type BlockType =
  | { code: { lang_hint: string | null } }
  | "terminal"
  | "log"
  | "paragraph"
  | "header"
  | "table"
  | "ui"
  | "unknown";

/** "code", "terminal", ...; never throws, whichever form arrives. */
export function blockTypeKey(bt: BlockType): string {
  return typeof bt === "string" ? bt : "code";
}

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
  showConsentOnStart: boolean;
}

export interface CaptureSettings {
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
  modelInstalled: boolean;
  model: string;
  version: string | null;
  availableModels: string[];
  host: string;
}

export interface SaveOutcome {
  failedHotkeys: string[];
}

export interface HotkeyResult {
  frame: CaptureFrame;
  result: AnalysisResult;
}

/** Results and errors of the system-wide shortcuts, which run in Rust. */
export function onHotkeyResult(handler: (payload: HotkeyResult) => void): Promise<UnlistenFn> {
  return listen<HotkeyResult>("claritydesk://result", (event) => handler(event.payload));
}

export function onHotkeyError(handler: (message: string) => void): Promise<UnlistenFn> {
  return listen<string>("claritydesk://error", (event) => handler(event.payload));
}

// ── API ──────────────────────────────────────────────────────────────────────

export const api = {
  // Capture
  listScreens: (): Promise<ScreenInfo[]> => invoke("list_screens"),
  capturePrimary: (): Promise<CaptureFrame> => invoke("capture_primary"),
  cropLastFrame: (x: number, y: number, width: number, height: number): Promise<CaptureFrame> =>
    invoke("crop_last_frame", { x, y, width, height }),
  getLastFrame: (): Promise<CaptureFrame | null> => invoke("get_last_frame"),

  // Analysis
  analyzeLastFrame: (mode: AnalysisMode): Promise<AnalysisResult> =>
    invoke("analyze_last_frame", { mode }),
  extractText: (pngB64: string): Promise<TextBlock[]> => invoke("extract_text", { pngB64 }),
  getLastResult: (): Promise<AnalysisResult | null> => invoke("get_last_result"),
  isAnalyzing: (): Promise<boolean> => invoke("is_analyzing"),

  // Settings
  getSettings: (): Promise<Settings> => invoke("get_settings"),
  saveSettings: (settings: Settings): Promise<SaveOutcome> => invoke("save_settings", { settings }),
  getProfilesDir: (): Promise<string> => invoke("get_profiles_dir"),
  checkOllama: (): Promise<OllamaStatus> => invoke("check_ollama"),
  getDefaultSettings: (): Promise<Settings> => invoke("get_default_settings"),
};

// ── Helpers ──────────────────────────────────────────────────────────────────

const BLOCK_LABELS: Record<string, string> = {
  code: "Code", terminal: "Terminal", log: "Log", paragraph: "Text",
  header: "Header", table: "Table", ui: "UI",
};

export function blockTypeLabel(bt: BlockType): string {
  return BLOCK_LABELS[blockTypeKey(bt)] ?? "Unknown";
}

export function blockTypeBadgeClass(bt: BlockType): string {
  const key = blockTypeKey(bt);
  return key === "paragraph" || key === "unknown" ? "badge-text" : `badge-${key}`;
}
