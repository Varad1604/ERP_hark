import { X } from "lucide-react";
import type { ReactNode } from "react";
import { Portal, useOverlay } from "./Overlay";

export interface DrawerProps {
  open: boolean;
  onClose: () => void;
  title: string;
  eyebrow?: ReactNode;
  media?: ReactNode;
  footer?: ReactNode;
  children?: ReactNode;
}

/** Right-hand sheet used for record/task detail. */
export function Drawer({ open, onClose, title, eyebrow, media, footer, children }: DrawerProps) {
  const panel = useOverlay(open, onClose);
  if (!open) return null;
  return (
    <Portal>
      <div className="sb-overlay" onClick={onClose} />
      <div ref={panel} className="sb-drawer" role="dialog" aria-modal="true" aria-label={title}>
        <button type="button" className="sb-icon-btn sb-close" aria-label="Close" onClick={onClose}>
          <X strokeWidth={1.75} />
        </button>
        <div>
          {media}
          {eyebrow && <p className="sb-drawer-eyebrow">{eyebrow}</p>}
          <h2>{title}</h2>
        </div>
        {children}
        {footer && <div className="sb-drawer-footer">{footer}</div>}
      </div>
    </Portal>
  );
}
