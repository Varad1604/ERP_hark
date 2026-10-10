import type { ReactNode } from "react";

export type BadgeKind = "attention" | "review" | "neutral" | "done" | "info";

export function Badge({ kind = "neutral", children }: { kind?: BadgeKind; children: ReactNode }) {
  return <span className={`sb-badge sb-badge--${kind}`}>{children}</span>;
}

export function KeyTag({ children }: { children: ReactNode }) {
  return <span className="sb-key">{children}</span>;
}
