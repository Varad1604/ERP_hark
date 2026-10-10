import { ArrowUpRight, BadgeCheck, Check, ChevronRight, CircleCheck, ListTodo, Megaphone, Sun } from "lucide-react";
import { Badge, BentoGrid, BentoTile, Button, Drawer, IconChip, StatTile, useToast } from "@sutra/ui";
import { useState } from "react";
import { useNavigate } from "react-router";
import { useSession } from "../lib/auth";
import { ANNOUNCEMENT, BADGE_LABEL, ROLE_CONFIG, routeFor, type Todo } from "../lib/roles";

export function meta() {
  return [{ title: "Home · SutraERP" }];
}

function today(): string {
  return new Intl.DateTimeFormat("en-IN", { weekday: "long", day: "numeric", month: "long" }).format(new Date());
}

export default function Home() {
  const { role } = useSession();
  const cfg = ROLE_CONFIG[role];
  const navigate = useNavigate();
  const toast = useToast();
  const [done, setDone] = useState<Record<number, boolean>>({});
  const [openTodo, setOpenTodo] = useState<number | null>(null);

  const go = (to: string) => {
    if (to.startsWith("todo:")) setOpenTodo(Number(to.slice(5)));
    else navigate(routeFor(to));
  };
  const todo: Todo | undefined = openTodo === null ? undefined : cfg.todos[openTodo];
  const parts = todo?.meta.split(" · ") ?? [];
  const finish = (verb: "Approved" | "Marked done") => {
    if (openTodo === null || !todo) return;
    setDone((d) => ({ ...d, [openTodo]: true }));
    toast(`${verb}: ${todo.title}`);
  };

  return (
    <BentoGrid data-home={role}>
      <BentoTile span={7} variant="accent" className="sb-hero">
        <span className="sb-chip">
          <Sun strokeWidth={1.75} /> {today()}
        </span>
        <h1>
          Hi {cfg.first},
          <br />
          let’s make today easy.
        </h1>
        <p>{cfg.hero.sub}</p>
        <Button className="sb-mt" icon={<ArrowUpRight strokeWidth={1.75} />} onClick={() => go(cfg.hero.to)}>
          {cfg.hero.cta}
        </Button>
      </BentoTile>

      <BentoTile span={5} variant="dark" className="sb-agenda">
        <div className="sb-agenda-label">
          <span>Right now</span>
          <span>{cfg.now.time}</span>
        </div>
        <h3>{cfg.now.title}</h3>
        <small>{cfg.now.sub}</small>
        <div className="sb-agenda-next">
          {cfg.now.next.map(([time, title, where]) => (
            <div key={time} className="sb-agenda-item">
              <time>{time}</time>
              {title}
              <span>{where}</span>
            </div>
          ))}
        </div>
      </BentoTile>

      {cfg.kpis.map((k) => (
        <StatTile key={k.label} icon={k.icon} tone={k.tone} label={k.label} value={k.value} hint={k.hint} onClick={() => go(k.to)} />
      ))}

      <BentoTile
        span={7}
        title="Your to-dos"
        action={
          <button type="button" onClick={() => navigate("/todos")}>
            See all
          </button>
        }
      >
        {cfg.todos.map((t, i) => (
          <button key={t.title} type="button" className={`sb-row${done[i] ? " is-done" : ""}`} onClick={() => setOpenTodo(i)}>
            <i className="sb-check">{done[i] && <Check strokeWidth={2} />}</i>
            <div>
              <b>{t.title}</b>
              <small>{t.meta}</small>
            </div>
            {done[i] ? <Badge kind="done">Done</Badge> : <Badge kind={t.status}>{BADGE_LABEL[t.status]}</Badge>}
          </button>
        ))}
      </BentoTile>

      <div className="sb-side sb-span-5">
        <BentoTile title="Shortcuts">
          <div className="sb-shortcuts">
            {cfg.shortcuts.map((s) => (
              <button key={s.label} type="button" className="sb-shortcut" onClick={() => go(s.to)}>
                <div className="sb-shortcut-icon">
                  <s.icon strokeWidth={1.75} />
                </div>
                {s.label}
              </button>
            ))}
          </div>
        </BentoTile>
        <BentoTile className="sb-notice">
          <IconChip icon={Megaphone} tone="green" size="md" />
          <div>
            <b>{ANNOUNCEMENT.title}</b>
            <small>{ANNOUNCEMENT.meta}</small>
          </div>
          <button type="button" className="sb-icon-btn" aria-label="All notices" onClick={() => navigate("/notices")}>
            <ChevronRight strokeWidth={1.75} />
          </button>
        </BentoTile>
      </div>

      <Drawer
        open={todo !== undefined}
        onClose={() => setOpenTodo(null)}
        title={todo?.title ?? ""}
        eyebrow={`To-do · ${parts[0] ?? ""}`}
        media={<IconChip icon={openTodo !== null && done[openTodo] ? CircleCheck : ListTodo} tone={openTodo !== null && done[openTodo] ? "green" : "orange"} />}
        footer={
          <>
            <Button icon={<Check strokeWidth={1.75} />} iconPosition="start" disabled={openTodo !== null && done[openTodo]} onClick={() => finish("Marked done")}>
              Mark done
            </Button>
            <Button variant="ghost" icon={<BadgeCheck strokeWidth={1.75} />} iconPosition="start" disabled={openTodo !== null && done[openTodo]} onClick={() => finish("Approved")}>
              Approve
            </Button>
          </>
        }
      >
        {todo && (
          <>
            <dl className="sb-dl">
              <dt>Area</dt>
              <dd>{parts[0]}</dd>
              <dt>From</dt>
              <dd>{parts.slice(1, -1).join(" · ") || parts[0]}</dd>
              <dt>Due</dt>
              <dd>{parts[parts.length - 1]}</dd>
              <dt>Status</dt>
              <dd>{openTodo !== null && done[openTodo] ? <Badge kind="done">Done</Badge> : <Badge kind={todo.status}>{BADGE_LABEL[todo.status]}</Badge>}</dd>
            </dl>
            <p style={{ fontSize: 14, color: "var(--sb-body)", lineHeight: 1.55 }}>
              Opened from your home queue. Review the details, then mark it done or approve it to clear it from today’s list.
            </p>
          </>
        )}
      </Drawer>
    </BentoGrid>
  );
}
