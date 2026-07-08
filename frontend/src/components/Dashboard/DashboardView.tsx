import { useEffect } from "react";
import {
  Monitor,
  Cpu,
  Languages,
  Terminal,
  Zap,
  Play,
  AlertCircle,
  CheckCircle,
  XCircle,
  Loader2,
} from "lucide-react";
import clsx from "clsx";
import { useClarityStore } from "../../stores/clarityStore";
import { AnalysisMode } from "../../lib/tauri";
import { useT } from "../../lib/i18n";

export function DashboardView() {
  const {
    mode, setMode,
    isCapturing, isAnalyzing,
    ollamaStatus,
    lastFrame, lastResult,
    consentGiven, giveConsent,
    checkOllama, captureAndAnalyze,
    error, clearError,
  } = useClarityStore();
  const t = useT();

  const MODES: { id: AnalysisMode; label: string; desc: string; Icon: typeof Languages }[] = [
    { id: "language", label: t("modeLanguageLabel"), desc: t("modeLanguageDesc"), Icon: Languages },
    { id: "dev",      label: t("modeDevLabel"),       desc: t("modeDevDesc"),     Icon: Terminal },
    { id: "smart",    label: t("modeSmartLabel"),     desc: t("modeSmartDesc"),   Icon: Zap },
  ];

  useEffect(() => {
    checkOllama();
    const id = setInterval(checkOllama, 30_000);
    return () => clearInterval(id);
  }, []);

  const isBusy = isCapturing || isAnalyzing;

  return (
    <div className="flex flex-col h-full gap-6 p-6 overflow-y-auto">
      {/* Header */}
      <div>
        <h1 className="text-xl font-semibold text-slate-100">ClarityDesk</h1>
        <p className="text-sm text-muted mt-0.5">{t("appTagline")}</p>
      </div>

      {/* Consent Banner */}
      {!consentGiven && (
        <div className="card border-accent/40 bg-accent/5">
          <div className="flex items-start gap-3">
            <AlertCircle size={18} className="text-accent mt-0.5 shrink-0" />
            <div className="flex-1">
              <p className="text-sm font-medium text-slate-100">{t("consentTitle")}</p>
              <p className="text-xs text-muted mt-1">
                {t("consentDesc")}
              </p>
              <button onClick={giveConsent} className="btn-primary mt-3 text-xs">
                {t("consentAllow")}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Ollama Status */}
      <div className="card">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Cpu size={16} className="text-muted" />
            <span className="text-sm font-medium">{t("localAiTitle")}</span>
          </div>
          {ollamaStatus == null ? (
            <Loader2 size={14} className="text-muted animate-spin" />
          ) : ollamaStatus.connected ? (
            <div className="flex items-center gap-1.5">
              <CheckCircle size={14} className="text-success" />
              <span className="text-xs text-success">{t("connected")}</span>
            </div>
          ) : (
            <div className="flex items-center gap-1.5">
              <XCircle size={14} className="text-danger" />
              <span className="text-xs text-danger">{t("offline")}</span>
            </div>
          )}
        </div>
        {ollamaStatus?.connected && (
          <div className="mt-2 flex flex-wrap gap-1.5">
            {ollamaStatus.availableModels.slice(0, 6).map((m) => (
              <span key={m} className="badge bg-surface-3 text-muted">{m}</span>
            ))}
          </div>
        )}
        {ollamaStatus && !ollamaStatus.connected && (
          <p className="text-xs text-muted mt-2">
            {t("startOllamaHintPrefix")} <code className="text-slate-300">ollama serve</code> {t("startOllamaHintSuffix")}
          </p>
        )}
      </div>

      {/* Mode Selector */}
      <div>
        <p className="label">{t("analysisModeLabel")}</p>
        <div className="grid grid-cols-3 gap-2">
          {MODES.map(({ id, label, desc, Icon }) => (
            <button
              key={id}
              onClick={() => setMode(id)}
              className={clsx(
                "flex flex-col items-start gap-2 p-3 rounded-xl border text-left transition-all",
                mode === id
                  ? "border-accent bg-accent/10 text-slate-100"
                  : "border-surface-3 bg-surface-2 text-muted hover:border-surface-3 hover:text-slate-300",
              )}
            >
              <Icon size={16} className={mode === id ? "text-accent" : "text-muted"} />
              <div>
                <p className="text-xs font-semibold">{label}</p>
                <p className="text-[11px] leading-snug mt-0.5 text-muted">{desc}</p>
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Capture Button */}
      <div className="flex gap-3">
        <button
          onClick={captureAndAnalyze}
          disabled={isBusy || !consentGiven}
          className={clsx(
            "flex-1 flex items-center justify-center gap-2 py-3 rounded-xl font-medium text-sm transition-all",
            isBusy || !consentGiven
              ? "bg-surface-2 text-muted cursor-not-allowed"
              : "bg-accent hover:bg-accent-hover text-white",
          )}
        >
          {isBusy ? (
            <>
              <Loader2 size={16} className="animate-spin" />
              {isCapturing ? t("capturing") : t("analyzing")}
            </>
          ) : (
            <>
              <Monitor size={16} />
              {t("captureAnalyze")}
            </>
          )}
        </button>
      </div>

      {/* Hotkeys */}
      <div className="card">
        <p className="label mb-2">{t("hotkeysLabel")}</p>
        <div className="grid grid-cols-2 gap-y-2 gap-x-4 text-xs">
          {[
            ["Alt+Shift+C", t("hkCapture")],
            ["Alt+Shift+D", t("hkDev")],
            ["Alt+Shift+S", t("hkSmart")],
            ["Alt+Shift+E", t("hkReanalyze")],
          ].map(([key, desc]) => (
            <div key={key} className="flex items-center gap-2">
              <kbd className="px-1.5 py-0.5 rounded bg-surface-3 text-slate-300 font-mono text-[10px]">
                {key}
              </kbd>
              <span className="text-muted">{desc}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Stats */}
      {lastFrame && (
        <div className="grid grid-cols-2 gap-3">
          {[
            { label: t("lastCapture"), value: new Date(lastFrame.capturedAt).toLocaleTimeString() },
            {
              label: t("blocksFound"),
              value: lastResult ? `${lastResult.totalBlocks}` : t("noData"),
            },
          ].map(({ label, value }) => (
            <div key={label} className="card text-center">
              <p className="text-lg font-semibold text-slate-100">{value}</p>
              <p className="text-[11px] text-muted mt-0.5">{label}</p>
            </div>
          ))}
        </div>
      )}

      {/* Error */}
      {error && (
        <div className="flex items-start gap-2 p-3 rounded-lg bg-danger/10 border border-danger/20">
          <AlertCircle size={14} className="text-danger mt-0.5 shrink-0" />
          <div className="flex-1 min-w-0">
            <p className="text-xs text-danger">{error}</p>
          </div>
          <button onClick={clearError} className="text-danger/60 hover:text-danger text-xs">
            ✕
          </button>
        </div>
      )}
    </div>
  );
}
