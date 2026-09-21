# SutraERP L6 — Stage 4 Batch 2 — Canonical Workflow Discovery UX Integration Evidence

**Version:** 1.0.0  
**Status:** Review  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Date:** 2026-09-21  
**Scope:** Stage 4 Batch 2 — FIND THE WORK only  
**Architecture Starting SHA:** `9e9837eb588fca247005c6cdaf9b4722f8068aa2`  
**Frontend Starting SHA:** `a6c2bff7c3cd903f9f7e5ff28021354c765b180e`  
**Frontend Branch:** `stage4-batch2-workflow-discovery-ux`  
**Frontend PR:** #19  
**Architecture Branch:** `l6-stage4-batch2-workflow-discovery-ux-evidence`

## 1. Authority Reconstruction

Repository authority confirms Stage 4 is the workflow discovery layer and requires:

- workflow-aware Global Search;
- owner-preserving destinations and handoffs;
- human-readable ordinary-user labels;
- reviewer-only architecture IDs;
- presentation-profile relevance without authorization semantics;
- no 1,084-item navigation dump;
- no workflow execution in Stage 4.

The existing Global Search seam is therefore the smallest safe integration point. No parallel search system, second command palette, workflow-route expansion, workspace registry, or UX archetype was justified.

## 2. Batch-1 Integrity

Batch 1 remains the only canonical derived workflow-discovery projection:

```text
1,084 canonical source scenarios
595 normalized workflows
99 workspaces
12 UX archetypes
10 mixed workflows
```

Protected workflow distribution remains:

```text
REVIEW-APPROVAL-WORKFLOW               271
INTERACTIVE-WORKFLOW                   154
EXCEPTION-RECOVERY-WORKFLOW            103
SYSTEM-BACKGROUND-OBSERVABLE             32
CROSS-WORKSPACE-HANDOFF                  28
TRACKING-WORKFLOW                         6
NO-DIRECT-END-USER-INTERACTION            1
TOTAL                                   595
```

Protected workflow mock distribution remains:

```text
REQUIRED       551
NOT-REQUIRED    11
PROHIBITED      33
TOTAL          595
```

Protected scenario mock distribution remains:

```text
REQUIRED       1,014
NOT-REQUIRED      19
PROHIBITED        51
TOTAL          1,084
```

The 10 mixed workflows remain preserved and are not flattened.

## 3. Discovery UX Architecture

Batch 2 extends the existing command palette rather than replacing it.

```text
user query
  ↓
deterministic workflow search adapter
  ↓
Batch-1 generated workflow projection
  ↓
existing Batch-1 destination binding
  ↓
existing workspace registry
  ↓
current home routing seam
```

Empty-query behavior remains unchanged: recent destinations continue to be shown where fixture context exists.

Workflow discovery results are query-driven and capped; the UI does not display all 595 workflows by default.

## 4. Matching and Ranking

Matching is deterministic and explainable:

1. exact workflow-label match;
2. workflow-label prefix;
3. workflow-label contains;
4. responsible-workspace exact match;
5. responsible-workspace prefix;
6. responsible-workspace contains;
7. deterministic lexical tie-break by workflow label and workspace name.

No fuzzy model, speculative synonym dictionary, business-rule-derived keyword set, or architecture-ID search surface was added.

One normalized workflow maps to one top-level workflow result.

## 5. Destination and Ownership Preservation

All workflow result destinations resolve through the existing workspace registry via the Batch-1 destination binding.

Ordinary user copy uses only the workflow label and responsible workspace.

Interaction-boundary copy is deliberately neutral:

- supported entry → **Open responsible area**;
- handoff-only → **Continue in responsible area**;
- observation-only → **View responsible area**.

This wording does not grant authority or imply execution.

Cross-workspace handoffs preserve the canonical responsible workspace. Global Search does not become the owner of the discovered workflow.

## 6. Presentation Profiles

Workspace presentation profiles may reduce irrelevant results for non-administrator presentation contexts.

This is presentation relevance only.

It is not authorization, RBAC, eligibility, permission, or business authority.

## 7. Safety Boundaries

The Batch-2 validator checks that:

- 595 normalized workflows remain available;
- all 10 mixed workflows remain preserved;
- PROHIBITED workflows remain observation-only;
- SYSTEM-BACKGROUND-OBSERVABLE workflows remain observation-only;
- NO-DIRECT-END-USER-INTERACTION remains observation-only;
- CROSS-WORKSPACE-HANDOFF remains handoff-only;
- result code exposes no ordinary L6-RWF/L3/L4 identifiers;
- search consumes the Batch-1 projection and destination binding;
- search ordering is deterministic;
- Stage-3 mock state is not used as workflow authority;
- the existing workspace registry remains authoritative;
- routing remains one existing application route rather than workflow-route expansion;
- presentation filtering is explicitly non-authoritative.

No L0–L5 architecture artifact was modified.

## 8. Files Changed — Frontend

- `app/registry/workflow-discovery-search.ts`
- `app/components/command-palette.tsx`
- `scripts/validate-workflow-discovery-ux.mjs`
- `package.json`
- `.github/workflows/frontend-pr-validation.yml`

## 9. Browser Evidence

PR CI extends the existing Playwright evidence convention with Batch-2-specific checks covering:

- desktop Global Search;
- narrow/mobile Global Search;
- canonical workflow query;
- Enter destination activation;
- keyboard active-result traversal;
- observation-only background workflow wording;
- handoff-oriented wording;
- absence of raw architecture IDs in visible UX;
- desktop and mobile workflow-search screenshots.

Screenshots are treated as visual evidence only, not as proof of accessibility.

## 10. Validation

Required gates:

- `npm run review:validate`
- `npm run mock:validate`
- `npm run typecheck`
- `npm run build`
- `npm run stage4:validate`
- frontend PR CI including targeted browser evidence

CI state and final frontend branch SHA are recorded in the final Batch-2 verdict update before closure.

## 11. Stage-3 Regression Assessment

Batch 2 does not modify:

- the canonical institutional seed;
- demo-state authority;
- reset behavior;
- mock clock;
- operational records;
- deterministic presets;
- fixture-convergence adapters.

Stage-3 state remains consumption context only and is not workflow-discovery authority.

## 12. Independent Self-Review

Current self-review finds:

- no second workflow registry;
- no second search system;
- no scenario-level search duplication;
- no actionability inferred from workflow existence;
- no raw architecture ID ordinary-user copy;
- no presentation-profile authorization model;
- no new workspace identity;
- no new archetype;
- no workflow route expansion;
- no Stage-3 authority duplication;
- no L0–L5 changes;
- no Stage-5 mutation or execution behavior.

## 13. Devil's-Advocate Review

Primary risks reviewed:

- search flooding is constrained by query-only workflow results and a small result cap;
- ranking is deterministic and inspectable;
- workflow/workspace overlap remains separated by result group;
- 595-workflow performance is a bounded in-memory projection scan suitable for this Stage-4 mock frontend;
- raw IDs are excluded from rendered workflow result content and browser assertions;
- PROHIBITED/background results use observation-oriented copy;
- mixed workflows use neutral top-level navigation language;
- persona handling is presentation-only and explicitly documented as non-authoritative;
- workspace destinations use canonical workspace keys through the existing binding;
- empty-query recent destinations remain unchanged;
- Stage-3 state is not a workflow search authority.

## 14. Batch-3 Readiness

Batch 3 must not start until Batch 2 receives its final verdict and explicit instruction.

Stage 5 has not started.
