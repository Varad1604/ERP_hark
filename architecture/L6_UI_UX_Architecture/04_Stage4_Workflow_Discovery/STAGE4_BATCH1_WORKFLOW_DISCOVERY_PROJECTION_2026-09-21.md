# SutraERP L6 — Stage 4 Batch 1 — Canonical Workflow Discovery Projection Foundation

**Status:** Implementation Evidence  
**Date:** 2026-09-21  
**Architecture Starting SHA:** `ac3300591c3058340c0b9d990981e7535ad0fdaf`  
**Frontend Starting SHA:** `8273504deb53dd8934eff51515b7bfe60aa14f08`

## Scope

Batch 1 establishes a deterministic frontend-consumable discovery projection without changing L0–L5, business ownership, Stage-3 state authority, workspace authority, or UX-archetype authority.

## Authority

Canonical inputs remain:

- `architecture/L6_UI_UX_Architecture/L6_Workflow_Realization_Register_DRAFT.md` — 1,084 scenario-level realization rows.
- `architecture/L6_UI_UX_Architecture/03_Workflow_Realization_Audit/group-reconciliation/STAGE2_GROUP_15_MASTER_CROSS_GROUP_RECONCILIATION.md` — 595 canonical normalized workflows and workflow-level classification/mock disposition.

The generated frontend artifact is derived material and is not a business authority.

## Projection Architecture

```text
Stage-2 master reconciliation + scenario realization register
        ↓ controlled deterministic sync
checked-in typed generated projection
        ↓
small frontend-owned destination binding
        ↓
future Stage-4 discovery surfaces
```

Normal frontend build does not read architecture Markdown at runtime or require the architecture repository to be present. Regeneration is an explicit repository-maintenance action using `scripts/generate-workflow-discovery.mjs` with `--architecture-root` or `SUTRAERP_ARCHITECTURE_ROOT`.

## Generated Ownership

Generated fields:

- internal `L6-RWF-####` trace identity;
- human-readable workflow label;
- canonical workspace reference;
- presentation profile union from scenario realization rows;
- canonical workflow classification;
- canonical workflow mock disposition;
- presentation-only interaction-availability guard;
- supported existing UX archetypes;
- discovery patterns;
- owner/handoff context;
- complete source-scenario population;
- controlled scenario-specific overrides;
- architecture traceability.

Frontend-owned fields:

- destination workspace resolution through existing `workspaceRegistry`;
- optional presentation context overrides only.

The destination layer does not own business classification, mock disposition, rules, scenario population, authorization, ownership or lifecycle semantics.

## Mechanical Revalidation

- canonical scenarios: **1,084**
- normalized workflows: **595**
- workflow classes: **271 / 154 / 103 / 32 / 28 / 6 / 1**
- workflow mock disposition: **551 / 11 / 33**
- scenario mock disposition: **1,014 / 19 / 51**
- workspaces: **99**
- UX archetypes: **12**
- mixed workflows: **10**

Mixed workflow IDs preserved exactly:

`L6-RWF-0105`, `L6-RWF-0257`, `L6-RWF-0265`, `L6-RWF-0301`, `L6-RWF-0302`, `L6-RWF-0307`, `L6-RWF-0390`, `L6-RWF-0394`, `L6-RWF-0398`, `L6-RWF-0428`

No first-row workflow semantic derivation is used.

## Safety

- `L6-RWF` identities remain internal/reviewer metadata.
- `PROHIBITED` workflows resolve to observation-only presentation.
- `SYSTEM-BACKGROUND-OBSERVABLE` and `NO-DIRECT-END-USER-INTERACTION` workflows cannot be emitted as workflow-entry commands by the projection.
- Presentation profiles remain presentation metadata, not authorization.
- Owner/handoff text is carried for discovery context while canonical ownership remains upstream.
- Existing 99-workspace registry and 12-archetype registry remain authoritative.
- Stage-3 seed/state/preset/time/operational authority is not referenced as workflow truth.

## Files

Frontend:
- `app/registry/workflow-discovery.generated.ts`
- `app/registry/workflow-discovery-bindings.ts`
- `scripts/generate-workflow-discovery.mjs`
- `scripts/validate-workflow-discovery.mjs`
- `package.json`

Architecture:
- this Batch-1 execution/evidence record only.

## Batch-2 Readiness

Batch 2 may consume the projection for user-facing discovery/search work only after Batch 1 validation and merge. Stage 5 has not started.
