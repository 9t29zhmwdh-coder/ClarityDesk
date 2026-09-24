import { useEffect } from "react";
import { Sidebar } from "./components/Layout/Sidebar";
import { DashboardView } from "./components/Dashboard/DashboardView";
import { AnalysisView } from "./components/Analysis/AnalysisView";
import { SettingsView } from "./components/Settings/SettingsView";
import { useClarityStore } from "./stores/clarityStore";
import { onHotkeyError, onHotkeyResult } from "./lib/tauri";
import { ErrorBanner } from "./components/Layout/ErrorBanner";

export function App() {
  const { activeTab, loadSettings, showHotkeyResult } = useClarityStore();

  useEffect(() => {
    loadSettings();
    // The shortcuts capture and analyze in Rust; the page only shows what they found.
    const done = onHotkeyResult(({ frame, result }) => showHotkeyResult(frame, result));
    const failed = onHotkeyError((message) => useClarityStore.setState({ error: message }));
    return () => {
      done.then((unlisten) => unlisten());
      failed.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <div className="flex h-screen bg-surface overflow-hidden">
      <Sidebar />
      <main className="flex-1 overflow-hidden flex flex-col">
        <ErrorBanner />
        {activeTab === "dashboard" && <DashboardView />}
        {activeTab === "analysis"  && <AnalysisView />}
        {activeTab === "settings"  && <SettingsView />}
      </main>
    </div>
  );
}
