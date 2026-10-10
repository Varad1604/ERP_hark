import type { ComponentType, ReactNode } from "react";
import type { NavLinkLikeProps } from "./Shell";

export interface TabItem {
  id: string;
  label: string;
  to: string;
}

/** Segmented pill tabs bound to routes. */
export function Tabs({ items, linkComponent: Link, label }: { items: TabItem[]; linkComponent: ComponentType<NavLinkLikeProps>; label: string }) {
  return (
    <nav className="sb-tabs" aria-label={label}>
      {items.map((t) => (
        <Link key={t.id} to={t.to} end className={({ isActive }) => (isActive ? "on" : "")}>
          {t.label}
        </Link>
      ))}
    </nav>
  );
}

/** Pill filter chip. */
export function FilterChip({ active, label, count, onClick, icon }: { active: boolean; label: string; count?: number; onClick: () => void; icon?: ReactNode }) {
  return (
    <button type="button" className={active ? "sb-filter on" : "sb-filter"} aria-pressed={active} onClick={onClick}>
      {icon}
      {label} {count !== undefined && <em>{count}</em>}
    </button>
  );
}
