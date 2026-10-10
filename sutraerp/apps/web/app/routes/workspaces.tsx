import { SearchX } from "lucide-react";
import { BentoGrid, BentoTile, EmptyState, FilterChip, IconChip, KeyTag, SearchField } from "@sutra/ui";
import { useMemo, useState } from "react";
import { useNavigate } from "react-router";
import { useWorkspaces } from "../lib/api";
import { useSession } from "../lib/auth";
import { GROUPS, groupMeta } from "../lib/icons";
import { ROLE_CONFIG } from "../lib/roles";

export function meta() {
  return [{ title: "Workspaces · SutraERP" }];
}

export default function Workspaces() {
  const { role } = useSession();
  const navigate = useNavigate();
  const { data, isPending, isError } = useWorkspaces(role);
  const [group, setGroup] = useState("all");
  const [query, setQuery] = useState("");
  const list = useMemo(() => data?.items ?? [], [data]);
  const counts = useMemo(() => list.reduce<Record<string, number>>((a, w) => ((a[w.group] = (a[w.group] ?? 0) + 1), a), {}), [list]);
  const q = query.trim().toLowerCase();
  const shown = list.filter((w) => (group === "all" || w.group === group) && (!q || `${w.name} ${w.key}`.toLowerCase().includes(q)));

  return (
    <BentoGrid>
      <BentoTile className="sb-page-head">
        <div>
          <h1>Workspaces</h1>
          <p>
            {isPending ? "Loading…" : `${list.length} workspaces open to you as ${ROLE_CONFIG[role].title.toLowerCase()}.`}
          </p>
        </div>
        <SearchField label="Search workspaces" placeholder="Search name or key" value={query} onChange={(e) => setQuery(e.target.value)} />
      </BentoTile>
      <div className="sb-filters">
        <FilterChip active={group === "all"} label="All" count={list.length} onClick={() => setGroup("all")} />
        {Object.keys(GROUPS)
          .filter((g) => counts[g])
          .map((g) => {
            const meta = groupMeta(g);
            return <FilterChip key={g} active={group === g} label={meta.label} count={counts[g]} icon={<meta.icon strokeWidth={1.75} />} onClick={() => setGroup(g)} />;
          })}
      </div>
      {isError && <EmptyState title="Workspaces could not be loaded">Check that the API is running.</EmptyState>}
      {!isPending && !isError && shown.length === 0 && (
        <EmptyState icon={SearchX} title="No workspaces match">
          Try another group or search.
        </EmptyState>
      )}
      {shown.map((w) => {
        const g = groupMeta(w.group);
        return (
          <BentoTile key={w.key} as="button" type="button" interactive className="sb-ws-card" data-ws={w.key} onClick={() => navigate(`/workspaces/${w.key}`)}>
            <IconChip icon={g.icon} tone={g.tone} />
            <h3>{w.name}</h3>
            <div className="sb-meta">
              <KeyTag>{w.key}</KeyTag>
              <span>
                {w.workflows.length} workflow{w.workflows.length === 1 ? "" : "s"}
              </span>
            </div>
          </BentoTile>
        );
      })}
    </BentoGrid>
  );
}
