import type { ReactNode } from "react";
import { Portal, useOverlay } from "./Overlay";

export interface ModalProps {
  open: boolean;
  onClose: () => void;
  label: string;
  children: ReactNode;
  /** Wrap children in the padded body (default true). */
  padded?: boolean;
}

export function Modal({ open, onClose, label, children, padded = true }: ModalProps) {
  const panel = useOverlay(open, onClose);
  if (!open) return null;
  return (
    <Portal>
      <div className="sb-overlay" onClick={onClose} />
      <div ref={panel} className="sb-modal" role="dialog" aria-modal="true" aria-label={label}>
        {padded ? <div className="sb-modal-body">{children}</div> : children}
      </div>
    </Portal>
  );
}
