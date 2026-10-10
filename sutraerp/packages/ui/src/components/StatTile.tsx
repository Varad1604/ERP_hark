import type { LucideIcon } from "lucide-react";
import { BentoTile, type Span } from "./BentoTile";
import { IconChip } from "./IconChip";
import type { Tone } from "./tones";

export interface StatTileProps {
  icon: LucideIcon;
  tone: Tone;
  label: string;
  value: string;
  hint?: string;
  span?: Span;
  onClick?: () => void;
}

/** KPI tile: icon chip, label, big value, hint. Clickable when `onClick` is set. */
export function StatTile({ icon, tone, label, value, hint, span = 3, onClick }: StatTileProps) {
  return (
    <BentoTile as={onClick ? "button" : "div"} type={onClick ? "button" : undefined} span={span} interactive={!!onClick} className="sb-stat" onClick={onClick}>
      <IconChip icon={icon} tone={tone} />
      <span>{label}</span>
      <b>{value}</b>
      {hint && <small>{hint}</small>}
    </BentoTile>
  );
}
