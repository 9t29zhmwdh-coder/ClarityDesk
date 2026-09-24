import { useRef, useState, type PointerEvent } from "react";
import { Crop, X } from "lucide-react";
import { CaptureFrame } from "../../lib/tauri";
import { useT } from "../../lib/i18n";

interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

interface Props {
  frame: CaptureFrame;
  onPick: (region: Rect) => void;
  onClose: () => void;
}

/** Smaller drags are treated as a click, not a region. */
const MIN_DRAG_PX = 12;

/**
 * Shows the capture large and lets the person drag a rectangle over the part they
 * want read. The rectangle is converted from displayed pixels to image pixels,
 * so the crop is exact whatever the window size.
 */
export function RegionPicker({ frame, onPick, onClose }: Props) {
  const imageRef = useRef<HTMLImageElement>(null);
  const [start, setStart] = useState<{ x: number; y: number } | null>(null);
  const [rect, setRect] = useState<Rect | null>(null);
  const t = useT();

  function pointFrom(event: PointerEvent) {
    const box = imageRef.current!.getBoundingClientRect();
    return {
      x: Math.min(Math.max(event.clientX - box.left, 0), box.width),
      y: Math.min(Math.max(event.clientY - box.top, 0), box.height),
    };
  }

  function onDown(event: PointerEvent) {
    event.currentTarget.setPointerCapture(event.pointerId);
    const point = pointFrom(event);
    setStart(point);
    setRect({ ...point, width: 0, height: 0 });
  }

  function onMove(event: PointerEvent) {
    if (!start) return;
    const point = pointFrom(event);
    setRect({
      x: Math.min(start.x, point.x),
      y: Math.min(start.y, point.y),
      width: Math.abs(point.x - start.x),
      height: Math.abs(point.y - start.y),
    });
  }

  function confirm() {
    if (!rect || !imageRef.current) return;
    const scale = frame.width / imageRef.current.getBoundingClientRect().width;
    onPick({
      x: Math.round(rect.x * scale),
      y: Math.round(rect.y * scale),
      width: Math.round(rect.width * scale),
      height: Math.round(rect.height * scale),
    });
  }

  const ready = rect !== null && rect.width >= MIN_DRAG_PX && rect.height >= MIN_DRAG_PX;

  return (
    <div className="fixed inset-0 z-50 bg-black/80 flex flex-col">
      <div className="flex items-center gap-3 px-4 py-3 bg-surface-1 border-b border-surface-3">
        <p className="flex-1 text-sm text-slate-200">{t("regionHint")}</p>
        <button onClick={confirm} disabled={!ready} className="btn-primary text-xs">
          <Crop size={13} /> {t("analyzeRegion")}
        </button>
        <button onClick={onClose} className="btn-ghost text-xs" aria-label={t("cancel")}>
          <X size={14} /> {t("cancel")}
        </button>
      </div>
      <div className="flex-1 overflow-auto flex items-start justify-center p-4">
        <div
          className="relative select-none cursor-crosshair touch-none"
          onPointerDown={onDown}
          onPointerMove={onMove}
          onPointerUp={() => setStart(null)}
        >
          <img
            ref={imageRef}
            src={`data:image/png;base64,${frame.imagePngB64}`}
            alt=""
            draggable={false}
            className="max-w-full max-h-[80vh] block"
          />
          {rect && (
            <div
              className="absolute border-2 border-accent bg-accent/10 pointer-events-none"
              style={{ left: rect.x, top: rect.y, width: rect.width, height: rect.height }}
            />
          )}
        </div>
      </div>
    </div>
  );
}
