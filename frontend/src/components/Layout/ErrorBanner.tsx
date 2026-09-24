import { XCircle, X } from "lucide-react";
import { useClarityStore } from "../../stores/clarityStore";
import { useT } from "../../lib/i18n";

/** Errors from any view or from a shortcut, shown wherever the person is. */
export function ErrorBanner() {
  const { error, clearError } = useClarityStore();
  const t = useT();
  if (!error) return null;
  return (
    <div role="alert" className="flex items-start gap-2 px-4 py-2.5 bg-danger/10 border-b border-danger/40 shrink-0">
      <XCircle size={16} className="text-danger mt-0.5 shrink-0" />
      <p className="flex-1 text-xs text-danger break-words">{error}</p>
      <button onClick={clearError} className="text-muted hover:text-slate-200" aria-label={t("dismiss")}>
        <X size={14} />
      </button>
    </div>
  );
}
