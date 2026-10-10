import type { LucideIcon } from "lucide-react";
import type { ReactNode } from "react";

export function EmptyState({ icon: Icon, title, children }: { icon?: LucideIcon; title: string; children?: ReactNode }) {
  return (
    <div className="sb-empty" role="status">
      {Icon && <Icon strokeWidth={1.75} aria-hidden="true" />}
      <b>{title}</b>
      {children && <p>{children}</p>}
    </div>
  );
}
