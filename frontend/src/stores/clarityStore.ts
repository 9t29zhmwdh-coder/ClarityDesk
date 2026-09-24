import { create } from "zustand";
import {
  api,
  AnalysisMode,
  AnalysisResult,
  CaptureFrame,
  OllamaStatus,
  Settings,
} from "../lib/tauri";

interface ClarityState {
  // Current state
  activeTab: "dashboard" | "analysis" | "settings";
  mode: AnalysisMode;
  isCapturing: boolean;
  isAnalyzing: boolean;
  consentGiven: boolean;

  // Data
  settings: Settings | null;
  lastFrame: CaptureFrame | null;
  lastResult: AnalysisResult | null;
  ollamaStatus: OllamaStatus | null;
  error: string | null;
  /** Shortcuts the system refused at the last save. */
  failedHotkeys: string[];

  // Actions
  setTab: (tab: ClarityState["activeTab"]) => void;
  setMode: (mode: AnalysisMode) => void;
  giveConsent: () => void;

  loadSettings: () => Promise<void>;
  saveSettings: (s: Settings) => Promise<void>;
  checkOllama: () => Promise<void>;

  capture: () => Promise<void>;
  analyze: () => Promise<void>;
  captureAndAnalyze: () => Promise<void>;
  analyzeRegion: (x: number, y: number, width: number, height: number) => Promise<void>;
  showHotkeyResult: (frame: CaptureFrame, result: AnalysisResult) => void;
  clearError: () => void;
}

export const useClarityStore = create<ClarityState>((set, get) => ({
  activeTab: "dashboard",
  mode: "smart",
  isCapturing: false,
  isAnalyzing: false,
  consentGiven: false,
  settings: null,
  lastFrame: null,
  lastResult: null,
  ollamaStatus: null,
  error: null,
  failedHotkeys: [],

  setTab: (tab) => set({ activeTab: tab }),
  setMode: (mode) => set({ mode }),
  giveConsent: () => {
    set({ consentGiven: true });
    // Asked once: the answer is kept in the settings file.
    const settings = get().settings;
    if (settings?.privacy.showConsentOnStart) {
      get().saveSettings({ ...settings, privacy: { ...settings.privacy, showConsentOnStart: false } });
    }
  },
  clearError: () => set({ error: null }),

  loadSettings: async () => {
    try {
      const settings = await api.getSettings();
      set({ settings, mode: settings.defaultMode, consentGiven: !settings.privacy.showConsentOnStart });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  saveSettings: async (settings) => {
    try {
      const outcome = await api.saveSettings(settings);
      set({ settings, failedHotkeys: outcome.failedHotkeys });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  checkOllama: async () => {
    try {
      const ollamaStatus = await api.checkOllama();
      set({ ollamaStatus });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  capture: async () => {
    if (!get().consentGiven) return;
    set({ isCapturing: true, error: null });
    try {
      const lastFrame = await api.capturePrimary();
      set({ lastFrame, activeTab: "analysis" });
    } catch (e) {
      set({ error: String(e) });
    } finally {
      set({ isCapturing: false });
    }
  },

  analyze: async () => {
    if (!get().lastFrame) return;
    set({ isAnalyzing: true, error: null });
    try {
      const lastResult = await api.analyzeLastFrame(get().mode);
      set({ lastResult });
    } catch (e) {
      set({ error: String(e) });
    } finally {
      set({ isAnalyzing: false });
    }
  },

  analyzeRegion: async (x, y, width, height) => {
    try {
      const lastFrame = await api.cropLastFrame(x, y, width, height);
      set({ lastFrame, lastResult: null });
      await get().analyze();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  showHotkeyResult: (lastFrame, lastResult) =>
    set({ lastFrame, lastResult, activeTab: "analysis", error: null }),

  captureAndAnalyze: async () => {
    await get().capture();
    if (get().lastFrame) {
      await get().analyze();
    }
  },
}));
