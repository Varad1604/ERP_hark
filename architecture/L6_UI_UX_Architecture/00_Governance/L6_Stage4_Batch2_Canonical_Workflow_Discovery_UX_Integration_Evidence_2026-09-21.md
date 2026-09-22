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

## 6. Presentation Relevance

Workflow relevance is derived from exact L6 presentation-profile expressions in the canonical realization register through the controlled `L6_Stage4_Workflow_Presentation_Relevance_Map.json`.

The mapping is exhaustive over source expressions, has no runtime fallback, and generates workflow-level `presentationPersonas` in the end-user-safe runtime projection.

Workspace `presentationProfiles` are not used as a proxy for workflow relevance.

This remains presentation relevance only. It is not authorization, RBAC, eligibility, permission, or business authority.

Regression evidence includes:
- Student-visible `Guardian Relationships` workspace while `Guardian relationship lifecycle` remains administrator-only;
- Faculty-visible `Curriculum Framework` workspace while `Curriculum lifecycle` remains administrator-only;
- legitimate Student and Faculty workflows remain discoverable in their respective presentations.

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

Batch-2 hardening now includes:

- `app/components/command-palette.tsx`
- `app/registry/workflow-discovery-search.ts`
- `app/registry/workflow-discovery-bindings.ts`
- `app/registry/workflow-discovery-runtime.generated.ts`
- `app/registry/workflow-discovery-provenance.generated.json`
- `scripts/generate-workflow-discovery.mjs`
- `scripts/validate-workflow-discovery.mjs`
- `scripts/validate-workflow-discovery-ux.mjs`
- `scripts/validate-workflow-discovery-sync.mjs`
- `package.json`
- `.github/workflows/frontend-pr-validation.yml`
- `.github/workflows/stage4-cross-repo-sync.yml`

The rich reviewer projection remains canonical for validation and traceability. Ordinary Global Search consumes only the generated end-user-safe runtime projection.

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

Final corrected frontend branch SHA:

```text
37aa13e61bd1881a3f4c77db50c55458f747d718
```

Fresh Frontend PR validation:

```text
Workflow: Frontend PR Validation
Run: #140
Run ID: 35691123777
Conclusion: SUCCESS
```

Current-head gates:

- `npm run review:validate` — PASS;
- `npm run stage4:validate` — PASS;
- Stage-4 provenance-structure validation — PASS;
- `npm run mock:validate` — PASS;
- `npm run typecheck` — PASS;
- `npm run build` — PASS;
- Stage-3 browser evidence — PASS;
- Batch-2 browser suite — PASS, 8 / 8 tests;
- visual-evidence artifact upload — PASS.

Current-head Stage-4 validation reports:

```text
workflows: 595
studentRelevant: 261
facultyRelevant: 156
workspaceProxyRegressions: 2
mixedWorkflows: 10
routeEntries: 1
reviewerMetadataInRuntime: 0
reviewerProjectionBytes: 690561
runtimeProjectionBytes: 135847
runtimeSourceReductionPercent: 80.3
```

Evidence artifact:

```text
experience-lab-visual-evidence
Artifact ID: 10678207286
SHA-256: 05b94dfcf59e9af04760756cfb494147321eebc5b2ea589d3fd001f0ebd3b6db
```

### Cross-repository synchronization guard

The frontend now records Git blob identities for the three canonical Architecture sources:

```text
STAGE2_GROUP_15_MASTER_CROSS_GROUP_RECONCILIATION.md
22dd6e0f38067936c7171c7e4386ebb58564f3f8

L6_Workflow_Realization_Register_DRAFT.md
53448e586a066c08745ec6be27f9e58016f34483

L6_Stage4_Workflow_Presentation_Relevance_Map.json
b47748cb2cbf9b692d41fbfea2f147f63159bb7a
```

These blob identities were revalidated directly against the canonical Architecture `main` sources for the two established Stage-2 authority files and against the controlled Batch-2 relevance-map source pending its merge to `main`.

`validate-workflow-discovery-sync.mjs` performs:

1. exact source-blob comparison;
2. generator `--check`;
3. byte-for-byte comparison of the reviewer projection;
4. byte-for-byte comparison of the end-user runtime projection;
5. byte-for-byte comparison of the provenance manifest.

Its `--prove-drift-detection` mode creates only a temporary copy of the Architecture sources, intentionally alters one source, requires the guard to reject it, removes the temporary copy, revalidates the pristine sources, then runs generator `--check`.

A dedicated `Stage 4 Cross-Repository Discovery Sync` workflow executes this guard using a repository secret named `ARCHITECTURE_READ_TOKEN` and checks out canonical `DhruvNaik26/SutraERP_Architecture` `main` as its authority.

The default frontend `GITHUB_TOKEN` was empirically proven unable to checkout the separate private Architecture repository (GitHub returned `Repository not found`). Therefore the cross-repository workflow intentionally fails closed when the dedicated read credential is absent rather than weakening repository privacy.

The credentialed cross-repository workflow has not been claimed as executed in this evidence because this session cannot provision or confirm repository Actions secrets. Normal frontend PR CI validates the provenance structure on every run.

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


## 15. Final Batch-2 Verdict

```text
STAGE 4 — BATCH 2 PASS WITH NON-BLOCKING FINDING
```

Frontend implementation head:

```text
37aa13e61bd1881a3f4c77db50c55458f747d718
```

Frontend PR:

```text
#19 — OPEN / UNMERGED / MERGEABLE
```

Architecture evidence PR:

```text
#54 — OPEN / UNMERGED
```

The four targeted Batch-2 hardening findings have been addressed in repository code and validation:

1. exact workspace/navigation precedence is protected without weakening exact workflow-label matching;
2. ordinary runtime search uses the generated end-user-safe projection instead of reviewer-rich metadata;
3. workflow-level presentation relevance is controlled from exact L6 presentation metadata and is not inherited from workspace visibility;
4. deterministic cross-repository source/projection drift detection is implemented, including reversible intentional-drift proof mode and a dedicated credentialed workflow.

Non-blocking repository-infrastructure finding:

```text
ARCHITECTURE_READ_TOKEN must be configured for the frontend repository
before the private cross-repository synchronization workflow can execute
as a hosted GitHub Actions gate.
```

This is an Actions credential/configuration requirement, not an architecture ambiguity or product-code defect. The default repository-scoped `GITHUB_TOKEN` cannot read the separate private Architecture repository.

The current authoritative source blobs match the checked-in frontend provenance for the present Batch-2 source state. The final corrected frontend head retargets the hosted cross-repository guard to canonical Architecture `main`. Frontend PR Validation #140 (run 35691123777) passed completely on that corrected head.

Batch 3 may begin only after explicit instruction and normal live-state reconstruction. Stage 5 has not started.
