import { useEffect, useRef, type ReactNode } from "react";
import { createPortal } from "react-dom";

/** Shared overlay behaviour: portal, backdrop click + Escape to close, focus restore. */
export function useOverlay(open: boolean, onClose: () => void) {
  const panel = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!open) return;
    const previous = document.activeElement as HTMLElement | null;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKey);
    const t = setTimeout(() => panel.current?.querySelector<HTMLElement>("input,button,[tabindex]")?.focus(), 30);
    return () => {
      clearTimeout(t);
      document.removeEventListener("keydown", onKey);
      previous?.focus?.();
    };
  }, [open, onClose]);
  return panel;
}

export function Portal({ children }: { children: ReactNode }) {
  if (typeof document === "undefined") return null;
  return createPortal(children, document.body);
}
