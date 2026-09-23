# SutraERP L5 — R1/R2 Upstream Authority Consolidation Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architecture Level:** Cross-level remediation governance (L2–L5 programme control)  
**Programme:** Post-Validation Upstream Blocker Resolution & Final-Lock Readiness  
**Scope:** Consolidation of validated R1A/R1B/R1C/R2A/R2B upstream authority remediation  
**Final L5 Corpus Lock:** NOT AUTHORIZED

---

## 1. Decision

The validated upstream-authority remediation from PRs #60, #61, #63, #64 and #65 is consolidated onto one fresh branch created from the canonical post-R4C `main` baseline.

The original five PRs were materially stale after R4C integration and shall not be merged independently.

This decision consolidates already reviewed upstream authority only. It does not perform R5 L4→L5 traceability validation, R6 L5 revalidation, or any L5 lifecycle promotion.

---

## 2. Canonical Baseline

The consolidation branch was created from post-R4C `main` after PR #74 merged.

Source remediation lines were each 63 commits behind current main and therefore unsafe for direct integration.

---

## 3. R1 Authority Carried Forward

### R1A — Tenant / Institution

Architecture Approved L4 parents:

- `L4-A-001`
- `L4-A-002`
- `L4-A-003`
- `L4-B-001`
- `L4-B-002`
- `L4-B-003`

### R1B — Workflow / Event / Module

Architecture Approved L4 parents:

- `L4-A-015`
- `L4-A-019`
- `L4-A-028`
- `L4-A-029`
- `L4-A-030`
- `L4-A-031`
- `L4-A-032`

### R1C — Workspace

Canonical Locked upstream amendment:

- `L2-A04_Workspace_Governance_Authority_and_Lifecycle_Amendment_2026-09-23.md`

Canonical Workspace decisions preserved:

- Workspace = one governed aggregate;
- Workspace Mapping = governed Workspace-version configuration;
- lifecycle: Draft → Under Review → Active → Retired;
- return: Under Review → Draft;
- material Active change creates a governed successor Draft;
- canonical authority: Workspace Governance Authority;
- canonical events: `WorkspaceActivated`, `WorkspaceRetired`;
- canonical publisher: Authorization & Policy / L2-A04;
- Workspace does not grant Roles/Permissions or alter Authorization Decisions.

Reconciled downstream parents:

- `L3-A-050` — Locked;
- `L4-A-050` v1.0.1 — Architecture Approved.

---

## 4. R2 Authority Carried Forward

### R2A — Reporting Projection

Architecture Approved:

- `L4-F-077`
- `L4-H-048`
- `L4-H-057`

### R2B — Notification

Architecture Approved:

- `L4-H-010`
- `L4-H-011`
- `L4-H-012`
- `L4-H-013`
- `L4-H-014`
- `L4-H-015`
- `L4-H-016`

Non-blocking missing actors/events remain preserved where the source architecture does not define them. No new business actor, event or ownership relation is invented by this consolidation.

---

## 5. Mechanical Consolidation Result

```text
Substantive validated source artifacts carried forward    31
Architecture Approved production L4 parents               24
Locked L2 amendments                                        1
Reconciled Locked L3 parents                                1
Duplicate source artifact paths                             0
Direct stale-branch merges                                  0
```

All 24 affected production L4 parents carry their source-approved lifecycle states on the consolidated branch.

---

## 6. Repository Integrity Boundary

This consolidation:

- preserves canonical business ownership;
- does not create L5 traceability relations;
- does not fabricate BH mappings;
- does not promote any L5 specification;
- does not alter the 105-row L5 register;
- does not allocate an ADR;
- does not reopen R3/R4;
- does not alter historical Batch-9 decisions;
- does not authorize Final L5 Corpus Lock.

The next governed actions remain:

1. R5 — affected L4→L5 traceability validation;
2. R6 — targeted revalidation of remaining affected L5 specifications;
3. R7 — lifecycle/register reconciliation;
4. R8 — final-lock re-entry audit.

---

## 7. Supersession

Once this consolidation is merged, PRs #60, #61, #63, #64 and #65 are superseded as integration lines and shall be closed without merge.

Their branch history remains repository provenance.

---

## 8. Decision

**Decision:** `PASS — R1/R2 VALIDATED UPSTREAM AUTHORITY CONSOLIDATED ON CURRENT MAIN; READY FOR R5 TRACEABILITY VALIDATION.`

**Final L5 Corpus Lock:** **NOT AUTHORIZED**
