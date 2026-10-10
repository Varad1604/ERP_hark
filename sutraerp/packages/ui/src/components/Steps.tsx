import { Check } from "lucide-react";

/** Vertical stage tracker (Draft → Submitted → In review → Approved). */
export function Steps({ stages, current }: { stages: string[]; current: number }) {
  const complete = current >= stages.length - 1;
  return (
    <ol className="sb-steps">
      {stages.map((s, i) => {
        const ok = i < current || complete;
        const cur = !ok && i === current;
        return (
          <li key={s} className={ok ? "ok" : cur ? "cur" : undefined}>
            <span className="n">{ok ? <Check strokeWidth={2} /> : i + 1}</span>
            <div>
              <b>{s}</b>
              <small>{ok ? "Completed" : cur ? "Current stage" : "Waiting"}</small>
            </div>
          </li>
        );
      })}
    </ol>
  );
}
