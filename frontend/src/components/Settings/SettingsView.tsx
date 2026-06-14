import { useEffect, useState } from "react";
import { Save, RotateCcw, CheckCircle } from "lucide-react";
import { useClarityStore } from "../../stores/clarityStore";
import { Settings } from "../../lib/tauri";
import clsx from "clsx";

export function SettingsView() {
  const { settings, loadSettings, saveSettings, ollamaStatus } = useClarityStore();
  const [form, setForm] = useState<Settings | null>(null);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    if (!settings) loadSettings();
  }, []);

  useEffect(() => {
    if (settings && !form) setForm(structuredClone(settings));
  }, [settings]);

  if (!form) return <div className="p-6 text-muted text-sm">Loading settings…</div>;

  function update<K extends keyof Settings>(key: K, value: Settings[K]) {
    setForm((prev) => prev ? { ...prev, [key]: value } : prev);
  }

  async function handleSave() {
    if (!form) return;
    await saveSettings(form);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  }

  function handleReset() {
    if (settings) setForm(structuredClone(settings));
  }

  return (
    <div className="flex flex-col h-full overflow-y-auto">
      {/* Toolbar */}
      <div className="flex items-center justify-between px-5 py-3 border-b border-surface-3 bg-surface-1 shrink-0">
        <h2 className="text-sm font-semibold">Settings</h2>
        <div className="flex gap-2">
          <button onClick={handleReset} className="btn-ghost text-xs">
            <RotateCcw size={13} /> Reset
          </button>
          <button
            onClick={handleSave}
            className={clsx("btn text-xs", saved ? "btn-ghost text-success" : "btn-primary")}
          >
            {saved ? <><CheckCircle size={13} /> Saved</> : <><Save size={13} /> Save</>}
          </button>
        </div>
      </div>

      <div className="p-5 space-y-6">
        {/* Ollama */}
        <section>
          <h3 className="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3 flex items-center gap-2">
            Local AI (Ollama)
            <span className={clsx("badge", ollamaStatus?.connected ? "bg-success/20 text-success" : "bg-danger/20 text-danger")}>
              {ollamaStatus?.connected ? "Online" : "Offline"}
            </span>
          </h3>
          <div className="space-y-3">
            <div>
              <label className="label">Ollama Host</label>
              <input
                className="input"
                value={form.ollamaHost}
                onChange={(e) => update("ollamaHost", e.target.value)}
                placeholder="http://localhost:11434"
              />
            </div>
            <div>
              <label className="label">Model</label>
              <input
                className="input"
                value={form.ollamaModel}
                onChange={(e) => update("ollamaModel", e.target.value)}
                placeholder="llama3.2"
                list="model-list"
              />
              {ollamaStatus?.availableModels.length ? (
                <datalist id="model-list">
                  {ollamaStatus.availableModels.map((m) => (
                    <option key={m} value={m} />
                  ))}
                </datalist>
              ) : null}
              {ollamaStatus?.availableModels.length ? (
                <div className="flex flex-wrap gap-1.5 mt-2">
                  {ollamaStatus.availableModels.map((m) => (
                    <button
                      key={m}
                      onClick={() => update("ollamaModel", m)}
                      className={clsx("badge cursor-pointer transition-colors", m === form.ollamaModel ? "bg-accent/30 text-accent" : "bg-surface-3 text-muted hover:text-slate-300")}
                    >
                      {m}
                    </button>
                  ))}
                </div>
              ) : null}
            </div>
          </div>
        </section>

        {/* Analysis */}
        <section>
          <h3 className="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">Analysis</h3>
          <div className="space-y-3">
            <div>
              <label className="label">Default Mode</label>
              <select
                className="input"
                value={form.defaultMode}
                onChange={(e) => update("defaultMode", e.target.value as Settings["defaultMode"])}
              >
                <option value="smart">Smart (auto-detect)</option>
                <option value="language">Language (translate)</option>
                <option value="dev">Dev (explain code/logs)</option>
              </select>
            </div>
            <div>
              <label className="label">Target Language</label>
              <input
                className="input"
                value={form.targetLanguage}
                onChange={(e) => update("targetLanguage", e.target.value)}
                placeholder="Deutsch"
              />
            </div>
          </div>
        </section>

        {/* OCR */}
        <section>
          <h3 className="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">OCR</h3>
          <div>
            <label className="label">OCR Language</label>
            <input
              className="input"
              value={form.ocrLanguage}
              onChange={(e) => update("ocrLanguage", e.target.value)}
              placeholder="eng+deu"
            />
            <p className="text-xs text-muted mt-1">
              Tesseract language codes separated by <code>+</code>. Example: <code>eng+deu+fra</code>
            </p>
          </div>
        </section>

        {/* Hotkeys */}
        <section>
          <h3 className="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">Hotkeys</h3>
          <div className="grid grid-cols-2 gap-3">
            {(
              [
                ["startStop",  "Start / Stop"],
                ["devMode",    "Dev Mode"],
                ["smartMode",  "Smart Mode"],
                ["reAnalyze",  "Re-Analyze"],
              ] as const
            ).map(([key, label]) => (
              <div key={key}>
                <label className="label">{label}</label>
                <input
                  className="input font-mono text-xs"
                  value={form.hotkeys[key]}
                  onChange={(e) =>
                    update("hotkeys", { ...form.hotkeys, [key]: e.target.value })
                  }
                />
              </div>
            ))}
          </div>
        </section>

        {/* Privacy */}
        <section>
          <h3 className="text-xs font-semibold text-slate-300 uppercase tracking-wider mb-3">Privacy</h3>
          <div className="space-y-3">
            {(
              [
                ["storeCaptures", "Store captured images on disk"],
                ["storeResults",  "Store analysis results on disk"],
                ["showConsentOnStart", "Show consent dialog on startup"],
              ] as const
            ).map(([key, label]) => (
              <label key={key} className="flex items-center gap-3 cursor-pointer">
                <input
                  type="checkbox"
                  checked={form.privacy[key]}
                  onChange={(e) =>
                    update("privacy", { ...form.privacy, [key]: e.target.checked })
                  }
                  className="w-4 h-4 rounded accent-indigo-500"
                />
                <span className="text-sm text-slate-300">{label}</span>
              </label>
            ))}
          </div>
          <div className="mt-3 p-3 rounded-lg bg-surface-2 text-xs text-muted">
            ClarityDesk processes everything locally. No data leaves your device.
            Tesseract and Ollama run entirely offline.
          </div>
        </section>
      </div>
    </div>
  );
}
