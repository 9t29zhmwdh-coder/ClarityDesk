import { LayoutDashboard, FileSearch, Settings, Eye } from "lucide-react";
import clsx from "clsx";
import { useClarityStore } from "../../stores/clarityStore";

const tabs = [
  { id: "dashboard" as const, label: "Dashboard", Icon: LayoutDashboard },
  { id: "analysis"  as const, label: "Analysis",  Icon: FileSearch },
  { id: "settings"  as const, label: "Settings",  Icon: Settings },
] as const;

export function Sidebar() {
  const { activeTab, setTab } = useClarityStore();

  return (
    <aside className="w-16 flex flex-col items-center gap-1 py-4 bg-surface-1 border-r border-surface-3 shrink-0">
      {/* Logo */}
      <div className="mb-4 flex items-center justify-center w-10 h-10 rounded-xl bg-accent/10">
        <Eye size={20} className="text-accent" />
      </div>

      <div className="flex-1 flex flex-col gap-1 w-full px-2">
        {tabs.map(({ id, label, Icon }) => (
          <button
            key={id}
            title={label}
            onClick={() => setTab(id)}
            className={clsx(
              "flex flex-col items-center gap-1 py-3 rounded-lg w-full transition-colors",
              activeTab === id
                ? "bg-accent/20 text-accent"
                : "text-muted hover:text-slate-300 hover:bg-surface-2",
            )}
          >
            <Icon size={18} />
            <span className="text-[10px] font-medium">{label}</span>
          </button>
        ))}
      </div>
    </aside>
  );
}
