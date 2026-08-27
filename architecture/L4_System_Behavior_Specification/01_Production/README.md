# SutraERP L4 Canonical Production Workspace

**Version:** 1.0.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architectural Level:** L4 — System Behavior Specification  
**Last Updated:** 2026-08-27  
**Depends On:** `../00_Governance/L4_Production_Parent_Baseline_48_2026-08-27.md`; `../00_Governance/L4_Governance_and_Architecture_Standard.md`; `../00_Governance/L4_System_Behavior_Specification_Template_v1.0.md`; `../00_Governance/L4_Validation_Standard.md`  

---

## Purpose

This folder is the canonical production location for L4 System Behavior Specifications.

`00_Pilot/` remains historical methodology evidence and shall never be treated as the production location merely because a pilot file has the same inherited parent identity.

---

## Current Production Structure

Only sections with currently certified L4 parents are materialized:

```text
01_Production/
├── section-d/
├── section-g/
├── section-h/
└── section-i/
```

Additional section folders may be added only when at least one parent in that section enters the certified production baseline or when a separate governance decision explicitly authorizes pre-materialization.

---

## Placement Rule

A canonical L4 file is placed by the section encoded in its certified L3 parent ID.

Examples:

```text
L3-G-279 → 01_Production/section-g/L4-G-279_<Canonical_L3_Scenario_Slug>.md
L3-H-037 → 01_Production/section-h/L4-H-037_<Canonical_L3_Scenario_Slug>.md
```

Human author, reviewer, participating domains, complexity, and status do not determine file placement.

---

## Production Rules

1. Author only from parents listed in `L4_Production_Parent_Baseline_48_2026-08-27.md`.
2. Use the Locked `L4_System_Behavior_Specification_Template_v1.0.md`.
3. Preserve the canonical L3 scenario slug in the filename.
4. One certified L3 parent maps to one parent L4 file by default.
5. BHs remain inside the parent file unless a governed physical-split exception is approved.
6. Every new production specification begins `Draft`.
7. Pilot content may be consulted as historical methodology evidence but is never copied/promoted without current-parent revalidation.
8. L4 may not repair L3/L2 gaps silently.
9. No APIs, persistence, services/classes, queues, technical IAM/RBAC, infrastructure, UI design, or other L5/L6 implementation content is permitted.
10. No file outside the certified baseline may be created in this workspace by inference.

---

## Status Flow

```text
Draft → Review → Architecture Approved → Locked
```

Authoring authorization never grants Architecture Approval or Lock automatically.

---

## Current Scope

**Certified production parents:** 48  
**Expected parent L4 files under default granularity:** 48  
**Current production sections:** D, G, H, I  
**Pilot files promoted automatically:** 0
