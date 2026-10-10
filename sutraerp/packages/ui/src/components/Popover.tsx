import { useEffect, useRef, type ReactNode } from "react";
import { cx } from "./cx";

/** Anchored header popover (notifications, account menu). Closes on outside click / Escape. */
export function Popover({ open, onClose, children, variant = "default", label }: { open: boolean; onClose: () => void; children: ReactNode; variant?: "default" | "menu"; label: string }) {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!open) return;
    const onDown = (e: MouseEvent) => {
      const target = e.target as Node;
      if (ref.current && !ref.current.contains(target) && !(target as HTMLElement).closest?.("[data-popover-trigger]")) onClose();
    };
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onClose();
    document.addEventListener("mousedown", onDown);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onDown);
      document.removeEventListener("keydown", onKey);
    };
  }, [open, onClose]);
  if (!open) return null;
  return (
    <div ref={ref} className={cx("sb-popover", variant === "menu" && "sb-popover--menu")} role="dialog" aria-label={label}>
      {children}
    </div>
  );
}
