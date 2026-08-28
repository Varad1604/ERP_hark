# SutraERP L4 Canonical Production Workspace

**Version:** 1.1.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architectural Level:** L4 — System Behavior Specification  
**Last Updated:** 2026-08-28  
**Depends On:** `../00_Governance/L4_Production_Parent_Baseline_48_2026-08-27.md`; `../00_Governance/L4_All_Section_Production_Folder_PreMaterialization_Decision_2026-08-28.md`; `../00_Governance/L4_Full_Corpus_Parent_Readiness_Certification_Workplan_2026-08-28.md`; `../00_Governance/L4_Governance_and_Architecture_Standard.md`; `../00_Governance/L4_System_Behavior_Specification_Template_v1.0.md`; `../00_Governance/L4_Validation_Standard.md`  

---

## Purpose

This folder is the canonical production location for L4 System Behavior Specifications.

`00_Pilot/` remains historical methodology evidence and shall never be treated as the production location merely because a pilot file has the same inherited parent identity.

---

## Current Production Structure

All canonical sections are pre-materialized as repository structure under the approved all-section folder decision:

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

Folder existence carries no parent-certification or authoring semantics. A section folder may exist before any parent in that section is certified.

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

1. Author only from parents in the currently authorized certified production baseline.
2. Use the Locked `L4_System_Behavior_Specification_Template_v1.0.md`.
3. Preserve the canonical L3 scenario slug in the filename.
4. One certified L3 parent maps to one parent L4 file by default.
5. BHs remain inside the parent file unless a governed physical-split exception is approved.
6. Every new production specification begins `Draft`.
7. Pilot content may be consulted as historical methodology evidence but is never copied/promoted without current-parent revalidation.
8. L4 may not repair L3/L2 gaps silently.
9. No APIs, persistence, services/classes, queues, technical IAM/RBAC, infrastructure, UI design, or other L5/L6 implementation content is permitted.
10. Pre-materialized folders do not authorize creation of scenario files for uncertified parents.

---

## Status Flow

```text
Draft → Review → Architecture Approved → Locked
```

Authoring authorization never grants Architecture Approval or Lock automatically.

---

## Current Scope

**Certified production parents:** 48  
**Expected parent L4 files under current certified baseline:** 48  
**Production section folders:** A, B, C, D, E, F, G, H, I, J  
**Full-corpus parent-readiness certification:** In progress  
**Pilot files promoted automatically:** 0
