import type { LucideIcon } from "lucide-react";
import { cx } from "./cx";
import { toneStyle, type Tone } from "./tones";

export interface IconChipProps {
  icon: LucideIcon;
  tone?: Tone;
  size?: "default" | "sm" | "md" | "lg";
  className?: string;
}

/** Rounded pastel square holding a Lucide icon. */
export function IconChip({ icon: Icon, tone = "grey", size = "default", className }: IconChipProps) {
  return (
    <div className={cx("sb-icon-chip", size !== "default" && `sb-icon-chip--${size}`, className)} style={toneStyle(tone)} aria-hidden="true">
      <Icon strokeWidth={1.75} />
    </div>
  );
}
