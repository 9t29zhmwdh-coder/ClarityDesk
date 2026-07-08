import { useState } from "react";
import { RefreshCw, Copy, ChevronDown, ChevronRight, Loader2, ImageOff } from "lucide-react";
import clsx from "clsx";
import { useClarityStore } from "../../stores/clarityStore";
import { AnalyzedBlock, blockTypeBadgeClass, blockTypeLabel } from "../../lib/tauri";
import { useT } from "../../lib/i18n";

export function AnalysisView() {
  const { lastFrame, lastResult, isAnalyzing, analyze } = useClarityStore();
  const [expandedBlocks, setExpandedBlocks] = useState<Set<string>>(new Set());
  const [tab, setTab] = useState<"original" | "output">("output");
  const [copied, setCopied] = useState<string | null>(null);
  const t = useT();

  function toggle(id: string) {
    setExpandedBlocks((prev) => {
      const next = new Set(prev);
      next.has(id) ? next.delete(id) : next.add(id);
      return next;
    });
  }

  async function copy(text: string, id: string) {
    await navigator.clipboard.writeText(text);
    setCopied(id);
    setTimeout(() => setCopied(null), 1500);
  }

  if (!lastFrame) {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-3 text-muted">
        <ImageOff size={40} strokeWidth={1} />
        <p className="text-sm">{t("noCaptureYet")}</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* Toolbar */}
      <div className="flex items-center gap-3 px-4 py-2.5 border-b border-surface-3 bg-surface-1 shrink-0">
        <div className="flex-1 flex items-center gap-2">
          <p className="text-xs text-muted">
            {lastFrame.width} × {lastFrame.height} px ·{" "}
            {new Date(lastFrame.capturedAt).toLocaleTimeString()}
          </p>
          {lastResult && (
            <span className="badge bg-surface-3 text-muted">
              {t(lastResult.totalBlocks === 1 ? "blockSingular" : "blockPlural", { n: lastResult.totalBlocks })}
            </span>
          )}
        </div>

        {/* Tab toggle */}
        <div className="flex bg-surface-2 rounded-lg p-0.5 text-xs">
          {(["output", "original"] as const).map((tabId) => (
            <button
              key={tabId}
              onClick={() => setTab(tabId)}
              className={clsx(
                "px-3 py-1 rounded-md capitalize transition-colors",
                tab === tabId ? "bg-surface-3 text-slate-100" : "text-muted hover:text-slate-300",
              )}
            >
              {tabId === "output" ? t("analyzed") : t("original")}
            </button>
          ))}
        </div>

        <button
          onClick={analyze}
          disabled={isAnalyzing}
          className="btn-ghost text-xs"
        >
          {isAnalyzing ? <Loader2 size={13} className="animate-spin" /> : <RefreshCw size={13} />}
          {t("reanalyze")}
        </button>
      </div>

      {/* Screenshot preview */}
      <div className="shrink-0 bg-surface border-b border-surface-3 flex justify-center p-2" style={{ maxHeight: 160 }}>
        <img
          src={`data:image/png;base64,${lastFrame.imagePngB64}`}
          alt="Captured screen"
          className="h-full max-h-36 object-contain rounded opacity-80"
        />
      </div>

      {/* Loading overlay */}
      {isAnalyzing && (
        <div className="flex-1 flex flex-col items-center justify-center gap-3 text-muted">
          <Loader2 size={28} className="animate-spin text-accent" />
          <p className="text-sm">{t("analyzingWithAi")}</p>
        </div>
      )}

      {/* Block list */}
      {!isAnalyzing && lastResult && (
        <div className="flex-1 overflow-y-auto p-4 space-y-2">
          {lastResult.blocks.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-muted gap-2">
              <p className="text-sm">{t("noBlocksDetected")}</p>
              <p className="text-xs">{t("tryDifferentRegion")}</p>
            </div>
          ) : (
            lastResult.blocks.map((block) => (
              <BlockCard
                key={block.blockId}
                block={block}
                tab={tab}
                expanded={expandedBlocks.has(block.blockId)}
                onToggle={() => toggle(block.blockId)}
                onCopy={copy}
                copied={copied === block.blockId}
              />
            ))
          )}
        </div>
      )}

      {/* No result yet */}
      {!isAnalyzing && !lastResult && (
        <div className="flex-1 flex flex-col items-center justify-center gap-3 text-muted">
          <p className="text-sm">{t("screenCapturedPrompt")}</p>
        </div>
      )}
    </div>
  );
}

function BlockCard({
  block,
  tab,
  expanded,
  onToggle,
  onCopy,
  copied,
}: {
  block: AnalyzedBlock;
  tab: "original" | "output";
  expanded: boolean;
  onToggle: () => void;
  onCopy: (text: string, id: string) => void;
  copied: boolean;
}) {
  const label = blockTypeLabel(block.blockType);
  const badgeClass = blockTypeBadgeClass(block.blockType);
  const content = tab === "output" ? block.output : block.original;
  const isCode = "code" in block.blockType || "terminal" in block.blockType || "log" in block.blockType;

  const preview = content.length > 120 ? content.slice(0, 120) + "…" : content;

  return (
    <div className="card p-0 overflow-hidden">
      {/* Header */}
      <button
        onClick={onToggle}
        className="flex items-center gap-2 w-full px-3 py-2.5 hover:bg-surface-2 transition-colors text-left"
      >
        {expanded ? (
          <ChevronDown size={13} className="text-muted shrink-0" />
        ) : (
          <ChevronRight size={13} className="text-muted shrink-0" />
        )}
        <span className={`${badgeClass} shrink-0`}>{label}</span>
        <span className="text-xs text-muted flex-1 truncate">{preview}</span>
        <button
          onClick={(e) => { e.stopPropagation(); onCopy(content, block.blockId); }}
          className="btn-ghost py-0.5 px-1.5 text-[11px] shrink-0"
        >
          {copied ? "✓" : <Copy size={11} />}
        </button>
      </button>

      {/* Expanded content */}
      {expanded && (
        <div className="border-t border-surface-3">
          {isCode ? (
            <pre className="text-xs font-mono p-3 overflow-x-auto text-slate-300 leading-relaxed whitespace-pre-wrap break-words">
              {content}
            </pre>
          ) : (
            <p className="text-sm p-3 text-slate-300 leading-relaxed whitespace-pre-wrap">{content}</p>
          )}
        </div>
      )}
    </div>
  );
}
