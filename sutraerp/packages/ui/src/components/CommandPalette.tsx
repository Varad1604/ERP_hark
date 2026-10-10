import { Search } from "lucide-react";
import type { LucideIcon } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { cx } from "./cx";
import { IconChip } from "./IconChip";
import { Modal } from "./Modal";
import type { Tone } from "./tones";

export interface PaletteItem {
  id: string;
  title: string;
  subtitle: string;
  group: string;
  icon: LucideIcon;
  tone: Tone;
}

export interface CommandPaletteProps {
  open: boolean;
  onClose: () => void;
  items: PaletteItem[];
  onSelect: (item: PaletteItem) => void;
  placeholder?: string;
  limit?: number;
}

/** ⌘K / Ctrl+K palette with keyboard navigation (↑ ↓ Enter Esc). */
export function CommandPalette({ open, onClose, items, onSelect, placeholder = "Search screens and workspaces", limit = 40 }: CommandPaletteProps) {
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState(0);
  useEffect(() => {
    if (open) {
      setQuery("");
      setSelected(0);
    }
  }, [open]);
  const shown = useMemo(() => {
    const q = query.trim().toLowerCase();
    return items.filter((x) => !q || `${x.title} ${x.subtitle}`.toLowerCase().includes(q)).slice(0, limit);
  }, [items, query, limit]);
  const pick = (i: number) => {
    const item = shown[i];
    if (item) {
      onClose();
      onSelect(item);
    }
  };
  let lastGroup = "";
  return (
    <Modal open={open} onClose={onClose} label="Command palette" padded={false}>
      <div className="sb-palette-input">
        <Search strokeWidth={1.75} aria-hidden="true" />
        <input
          value={query}
          placeholder={placeholder}
          aria-label="Command palette"
          onChange={(e) => {
            setQuery(e.target.value);
            setSelected(0);
          }}
          onKeyDown={(e) => {
            if (e.key === "ArrowDown") {
              setSelected((s) => Math.min(s + 1, shown.length - 1));
              e.preventDefault();
            } else if (e.key === "ArrowUp") {
              setSelected((s) => Math.max(s - 1, 0));
              e.preventDefault();
            } else if (e.key === "Enter") {
              pick(selected);
            }
          }}
        />
        <kbd className="sb-kbd">Esc</kbd>
      </div>
      <div className="sb-palette-list" role="listbox">
        {shown.length === 0 && <p className="sb-palette-group">No matches</p>}
        {shown.map((x, i) => {
          const header = x.group !== lastGroup ? <p className="sb-palette-group">{x.group}</p> : null;
          lastGroup = x.group;
          return (
            <div key={x.id}>
              {header}
              <button type="button" role="option" aria-selected={i === selected} className={cx("sb-pop-item", i === selected && "is-selected")} onClick={() => pick(i)}>
                <IconChip icon={x.icon} tone={x.tone} size="sm" />
                <div>
                  <b>{x.title}</b>
                  <small>{x.subtitle}</small>
                </div>
              </button>
            </div>
          );
        })}
      </div>
    </Modal>
  );
}
