import { useEffect } from "react";
import { Sidebar } from "./components/Layout/Sidebar";
import { DashboardView } from "./components/Dashboard/DashboardView";
import { AnalysisView } from "./components/Analysis/AnalysisView";
import { SettingsView } from "./components/Settings/SettingsView";
import { useClarityStore } from "./stores/clarityStore";

export function App() {
  const { activeTab, loadSettings } = useClarityStore();

  useEffect(() => {
    loadSettings();
  }, []);

  return (
    <div className="flex h-screen bg-surface overflow-hidden">
      <Sidebar />
      <main className="flex-1 overflow-hidden">
        {activeTab === "dashboard" && <DashboardView />}
        {activeTab === "analysis"  && <AnalysisView />}
        {activeTab === "settings"  && <SettingsView />}
      </main>
    </div>
  );
}
