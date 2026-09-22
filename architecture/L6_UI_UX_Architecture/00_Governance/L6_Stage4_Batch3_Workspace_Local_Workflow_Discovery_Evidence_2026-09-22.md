# SutraERP L6 — Stage 4 Batch 3 — Workspace-Local Workflow Discovery Evidence

**Version:** 1.0.0  
**Status:** Review  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Date:** 2026-09-22  
**Scope:** Stage 4 Batch 3 — FIND THE WORK only  
**Architecture Starting SHA:** `a8de18b375121483fc14f146c0dc32285c599b91`  
**Frontend Starting SHA:** `afd0e07650456121463e5d520b8881937e233bc3`  
**Frontend Final SHA for Fix-3 Evidence:** `b489eb01562e820af2d1fbf04fea6faf59a955e8`  
**Frontend Branch:** `stage4-batch3-workspace-local-discovery`  
**Frontend PR:** #20  
**Frontend CI:** Frontend PR Validation #147 / Run ID `35702573775` / SUCCESS  
**Architecture Branch:** `l6-stage4-batch3-workspace-local-discovery-evidence`

## 1. Canonical Scope and Counts

Batch 3 extends Stage-4 discovery from Global Search into existing workspace-local surfaces without redefining workflow authority.

Protected canonical counts remain:

```text
595 normalized workflows
1,084 canonical scenarios
99 workspaces
12 UX archetypes
10 mixed workflows
```

No second workflow authority, workspace registry, archetype registry, scenario-derived UI registry, or workflow-route model was created.

## 2. Workspace-Local Discovery Design

Batch 3 uses one reusable workspace-local discovery surface backed by the existing generated end-user-safe workflow projection and canonical workspace registry.

Conceptually:

```text
current workspace
  ↓
canonical workspace key
  ↓
generated end-user-safe workflow projection
  ↓
workflow-level presentation relevance
  ↓
deterministic local workflow population
  ↓
descriptive Stage-4 discovery rows
```

The surface is integrated through existing Student, reference, deep, and breadth workspace families rather than creating a separate implementation per workspace.

One normalized workflow yields one local result. No scenario-per-row expansion is introduced.

## 3. Persona Relevance Boundary

Workflow visibility is filtered from governed workflow-level `presentationPersonas` in the generated runtime projection.

Workspace visibility does not grant workflow relevance.

This is presentation relevance only. It is not authorization, RBAC, eligibility, permission, or business authority.

Regression evidence includes:

- Student-visible workspace with admin-only workflows remaining hidden;
- administrator-visible workflows remaining available where governed;
- Faculty and Student high-density representative workspaces using exact workflow-level relevance.

## 4. Reviewer / End-User Separation

Ordinary workspace-local discovery consumes only:

```text
label
workspaceKey
presentationPersonas
interactionAvailability
```

It does not consume the reviewer-rich projection.

Browser and static validation assert absence of ordinary-user leakage of:

```text
L6-RWF IDs
L3 IDs
L4 IDs
architecture paths
source keys
scenario IDs
traceability metadata
```

Reviewer-rich projection and provenance remain available for validation and architecture review only.

## 5. Interaction Semantics

Batch 3 remains discovery-only.

Rows are intentionally non-interactive in the current Batch-3 implementation. Their copy was hardened to describe state honestly rather than imply navigation:

```text
WORKFLOW_ENTRY_SUPPORTED
→ Relevant workflow in this responsible area

HANDOFF_ONLY
→ Work continues in the responsible area

OBSERVATION_ONLY
→ Status, history or evidence are available for review
```

No workflow execution, mutation, approval, rejection, retry, resolution, processing, publication, completion, or new workflow-specific route is introduced.

## 6. Owner Preservation

Workspace-local discovery is anchored to the workflow's canonical workspace key through the existing workspace registry.

Discovery does not transfer business ownership to the current UI surface.

CROSS-WORKSPACE-HANDOFF entries remain handoff-only. Observation-only and background entries remain non-actionable.

## 7. Stage-3 Preservation

Batch 3 does not modify:

- canonical institutional mock seed;
- demo-state authority;
- reset behavior;
- deterministic mock clock;
- operational records;
- presets;
- fixture-convergence adapters.

Stage-3 task/demo state remains operational context only and is not promoted into canonical workflow authority.

## 8. Browser Evidence

A dedicated Batch-3 Playwright suite is executed in PR CI.

Coverage includes:

- desktop workspace-local discovery;
- narrow/mobile Student Workspace;
- high-density administrator workspace — Infrastructure Management, 21 relevant workflows;
- high-density Student presentation — Health / Medical Services, 18 relevant workflows;
- Faculty workspace — Library, 15 relevant workflows;
- observation-only workflow presentation;
- handoff workflow presentation;
- persona filtering;
- no raw L3/L4/L6 IDs in the local discovery surface.

Screenshots are captured as visual evidence only. They are not treated as proof of accessibility.

Latest visual-evidence artifact from the validated Fix-2 head:

```text
Artifact: experience-lab-visual-evidence
Artifact ID: 10683565314
SHA-256: 8a6583fef7d70c2220e41f1b506a5bb43c06b78a9940df5cec00bf78ef638a9d
```

## 9. Validation Evidence

Frontend SHA:

```text
b489eb01562e820af2d1fbf04fea6faf59a955e8
```

Frontend PR:

```text
#20 — OPEN / UNMERGED / MERGEABLE
```

Final CI used by this evidence:

```text
Workflow: Frontend PR Validation
Run: #147
Run ID: 35702573775
Conclusion: SUCCESS
```

Validated gates:

- `npm run review:validate` — PASS;
- `npm run stage4:validate` — PASS;
- Stage-4 provenance validation — PASS;
- `npm run mock:validate` — PASS;
- `npm run typecheck` — PASS;
- `npm run build` — PASS;
- Batch-3 browser suite — PASS;
- screenshot capture/upload — PASS.

Mechanical protections remain:

```text
595 workflows
1,084 scenarios
99 workspaces
12 archetypes
10 mixed workflows
1 existing application route
0 reviewer metadata fields in ordinary runtime projection
0 second workflow authority
0 second workspace registry
0 Stage-5 execution surfaces
```

## 10. Architecture Integrity

No L0–L5 artifact is modified by this evidence PR.

Batch 3 preserves:

- Stage-2 realization authority;
- Batch-1 reviewer/traceability projection;
- Batch-2 end-user-safe runtime projection;
- canonical workspace ownership;
- workflow-level presentation relevance;
- cross-repository provenance/sync guard;
- Stage-3 deterministic-state authority.

The Architecture repository had advanced after Batch-2 merge because of independent L5 work. Therefore this Batch-3 evidence branch correctly starts from current Architecture `main` SHA `a8de18b375121483fc14f146c0dc32285c599b91` rather than the historical Batch-2 merge SHA.

## 11. Final Fix-3 Evidence Verdict

```text
STAGE 4 — BATCH 3 ARCHITECTURE EVIDENCE COMPLETE
```

Current implementation status:

- workspace-local workflow discovery is implemented;
- workflow-level persona relevance is preserved;
- reviewer/end-user separation is preserved;
- Stage-3 state authority is preserved;
- no Stage-5 behavior has started;
- browser evidence is current and passing;
- Frontend PR #20 remains open and unmerged.

Fix 4 cleanup remains outside this evidence step and is intentionally not performed here.

Do not merge automatically.
