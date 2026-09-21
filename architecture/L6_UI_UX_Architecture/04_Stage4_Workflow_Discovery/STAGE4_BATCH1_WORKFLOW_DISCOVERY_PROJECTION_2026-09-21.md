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


## Repository / Validation Evidence

- Frontend PR: **#18**
- Architecture PR: **#53**
- Frontend branch head at implementation review: `10a8e4a518c009bbe3b8dffef79cdc324af96901`
- Deterministic regeneration comparison against canonical architecture sources: **exact byte-for-byte match**
- Existing frontend PR validation:
  - `npm run review:validate`: **PASS** (includes Stage-4 discovery validator)
  - `npm run mock:validate`: **PASS**
  - `npm run typecheck`: **PASS**
  - `npm run build`: **PASS**
  - existing screenshot/regression evidence: **PASS**

Repository diff inspection confirms the frontend branch changes only the generated projection, destination-binding seam, generator/validator scripts, and package validation wiring. The architecture branch changes only this L6 Stage-4 evidence record.

## Independent Self-Review

- competing workflow authority: **not introduced**; generated artifact is explicitly derived and non-authoritative;
- canonical Stage-2 source: **preserved**;
- first-row deduplication: **not used**;
- mixed workflows: **all 10 preserved with explicit scenario overrides**;
- normalized workflow count: **595**;
- canonical scenario accounting: **1,084 unique**;
- workflow class distribution: **271 / 154 / 103 / 32 / 28 / 6 / 1**;
- workflow mock distribution: **551 / 11 / 33**;
- scenario mock distribution: **1,014 / 19 / 51**;
- runtime Markdown coupling: **none**;
- ordinary-user L6-RWF exposure: **none introduced**;
- PROHIBITED workflow commands: **none**;
- background workflow commands: **none**;
- presentation profile authorization semantics: **none**;
- new workflow routes: **none**;
- new workspace authority: **none**;
- new archetype: **none**;
- Stage-3 state/fixture authority change: **none**;
- canonical owner preservation: **retained through source owner/handoff context plus workspace binding**;
- L0–L5 modifications: **none**;
- Stage 5 execution behavior: **not introduced**.

## Devil's-Advocate Review

| Concern | Evidence / Result |
|---|---|
| Projection may drift from architecture | Generator has an explicit canonical-source sync path; deterministic regeneration produced an exact match. Generated file is marked do-not-edit. |
| Parser may silently lose workflows | Hard-fails unless exactly 595 workflows and 1,084 scenarios are parsed; duplicates/missing scenario references are rejected. |
| Register format may change | Parser requires exact governed table headers/shape and fails loudly when required workflow/scenario identities disappear. |
| Mixed workflows may flatten | Explicit expected set of 10 mixed workflows is asserted; scenario overrides are generated by comparison to master workflow semantics. |
| Destination bindings may become duplicate registry | Binding resolves through existing `workspaceRegistry`; hand-authored override map contains presentation context only and is empty at Batch 1. |
| Workspace names may become routing keys | Canonical binding uses workspace key; workspace name is resolved from existing registry only for presentation/destination use. |
| PROHIBITED workflows may gain actions | Validator requires `OBSERVATION_ONLY` for every PROHIBITED workflow. |
| Background workflows may become commands | Validator requires observation-only for SYSTEM-BACKGROUND-OBSERVABLE and NO-DIRECT-END-USER-INTERACTION. |
| Reviewer metadata may leak | Projection is not yet connected to ordinary Global Search/UI; internal-ID boundary is documented and validated. |
| Stage-4 metadata may become authorization | No permission/RBAC/authorization fields exist; presentation profiles remain presentation data. |
| Global Search may be overloaded | Batch 1 does not modify the command palette. |
| Stage-3 demo state may become business truth | Generated projection imports no Stage-3 mock authority and derives only from Stage-2 realization sources. |
| Runtime Markdown coupling | None; normal frontend build consumes a static typed artifact. |
| Two workflow truths may exist | The generated artifact is explicitly derived material with source traceability; architecture remains authority. |

## Remaining Risks / Open Questions

1. Future architecture-table structural changes require an intentional generator update; current behavior is fail-loud rather than heuristic.
2. Future user-facing discovery must keep internal IDs and architecture paths out of ordinary result copy.
3. Batch 2 should define ranking/grouping behavior without turning presentation profiles into access control or workflow existence into execution permission.


## Final Batch-1 Gate State

Frontend CI run **#105 / Frontend PR Validation**: **PASS**.

All required Batch-1 execution gates are green. PRs remain open for repository review/merge; Batch 2 and Stage 5 are not started.


## Merge Synchronization

Frontend PR **#18** merged successfully to `main`.

**Frontend ending main SHA:** `a6c2bff7c3cd903f9f7e5ff28021354c765b180e`

This synchronization records the actual merged frontend state used by the Batch-1 closure evidence.
