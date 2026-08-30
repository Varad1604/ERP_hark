# SutraERP L4 Canonical Production Workspace

**Version:** 1.2.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architectural Level:** L4 — System Behavior Specification  
**Last Updated:** 2026-08-29  
**Depends On:** `../00_Governance/L4_Full_Corpus_Parallel_Authoring_Authorization_2026-08-29.md`; `../00_Governance/L4_Full_Corpus_Parallel_Authoring_Execution_Plan_1084_Scenarios_2026-08-29.md`; `../00_Governance/L4_Full_Corpus_Parent_Readiness_Baseline_1084_2026-08-29.md`; `../00_Governance/L4_All_Section_Production_Folder_PreMaterialization_Decision_2026-08-28.md`; `../00_Governance/L4_Governance_and_Architecture_Standard.md`; `../00_Governance/L4_System_Behavior_Specification_Template_v1.0.md`; `../00_Governance/L4_Validation_Standard.md`  

---

## Purpose

This folder is the canonical production location for L4 System Behavior Specifications.

`00_Pilot/` remains historical methodology evidence and shall never be treated as the production location merely because a pilot file has the same inherited parent identity.

---

## Current Production Structure

All canonical sections are pre-materialized as repository structure:

```text
01_Production/
├── section-a/
├── section-b/
├── section-c/
├── section-d/
├── section-e/
├── section-f/
├── section-g/
├── section-h/
├── section-i/
└── section-j/
```

Folder existence carries no parent-readiness or authoring-completion semantics.

---

## Placement Rule

A canonical L4 file is placed by the section encoded in its L3 parent ID.

Examples:

```text
L3-G-279 → 01_Production/section-g/L4-G-279_<Canonical_L3_Scenario_Slug>.md
L3-H-037 → 01_Production/section-h/L4-H-037_<Canonical_L3_Scenario_Slug>.md
```

Human author, reviewer, participating domains, complexity and status do not determine file placement.

---

## Production Rules

1. The active workload population is the complete **1,084-scenario canonical L3 corpus**, with human assignments inherited from the locked L3 Phase 5 allocation.
2. All 1,084 canonical L3 parents are currently `Locked` and ready for L4 derivation under the final corpus lock decision.
3. Before creating each L4 file, perform a normal no-drift check. If a later governed change has reopened a specific parent, pause only that affected parent until its successor approved/locked state is established.
4. Use the Locked `L4_System_Behavior_Specification_Template_v1.0.md`.
5. Preserve the canonical L3 scenario slug in the filename.
6. One eligible L3 parent maps to one parent L4 file by default.
7. BHs remain inside the parent file unless a governed physical-split exception is approved.
8. Every new production specification begins `Draft`.
9. Pilot content may be consulted as historical methodology evidence but is never copied/promoted without current-parent revalidation.
10. L4 may not repair L3/L2 gaps silently.
11. No APIs, persistence, services/classes, queues, technical IAM/RBAC, infrastructure, UI design, or other L5/L6 implementation content is permitted.

---

## Status Flow

```text
Draft → Review → Architecture Approved → Locked
```

Authoring authorization never grants Architecture Approval or Lock automatically.

---

## Current Scope

**Canonical L3 workload population:** 1,084  
**L4 human author allocation:** inherited exactly from locked L3 Phase 5 allocation  
**Static 48-parent production limit:** superseded 2026-08-29  
**Locked L3 parents ready:** 1,084 / 1,084  
**Production section folders:** A, B, C, D, E, F, G, H, I, J  
**Pilot files promoted automatically:** 0

The historical 48-parent baseline and execution records remain decision provenance only and no longer constrain active L4 workload scope.
