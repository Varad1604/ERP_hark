# SutraERP L6 — Stage 4 Batch 4 — Full Discovery Coverage & Exit-Gate Evidence

**Version:** 1.0.0  
**Status:** Review  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Date:** 2026-09-22  
**Scope:** Stage 4 Batch 4 — FIND THE WORK only  
**Batch-4 Entry Architecture SHA:** `3784fd78833f183e4a17af79bf5d77691f989c31`  
**Architecture Evidence Branch Base SHA:** `fa7a80abb8b2be997088b9ab167a1d29aeefc882`  
**Batch-4 Entry Frontend SHA:** `e244821f4f845b8a8cfc7ddfaf456ef0624b34c4`  
**Final Frontend SHA:** `eec69310c2078a2af7cd796439c6adff3cf86e8d`  
**Frontend Branch:** `stage4-batch4-full-discovery-reconciliation`  
**Frontend PR:** #21  
**Final Frontend CI:** Frontend PR Validation #151 / Run ID `35712022674` / SUCCESS  
**Visual Evidence Artifact:** `stage4-batch4-visual-evidence` / Artifact ID `10686552588` / SHA-256 `39f1d3efd02ffcc3452ea95100aadd078060bfcd92f2df0b1180ddc536e07714`

## 1. Batch-3 Canonicalization Gate

Batch 4 began only after the complete validated Batch-3 state was canonical.

- Frontend PR #20 — merged into `main` at `e244821f4f845b8a8cfc7ddfaf456ef0624b34c4`.
- Architecture PR #55 — merged into `main` at `3784fd78833f183e4a17af79bf5d77691f989c31`.
- No Batch-3 file was cherry-picked, copied manually, or bypassed into Batch 4.

Concurrent unrelated L5 work advanced Architecture `main` after Batch-4 entry. This evidence branch therefore starts from current Architecture `main` SHA `fa7a80abb8b2be997088b9ab167a1d29aeefc882` while recording the actual Batch-4 entry SHA separately.

## 2. Protected Canonical Counts

The Batch-4 mechanical reconciliation preserves:

```text
595 normalized workflows
1,084 canonical scenarios
99 workspaces
12 UX archetypes
10 mixed workflows
```

Validation additionally confirms:

- 1,084 scenario assignments are unique;
- the application remains on one existing route;
- no second workflow authority exists;
- no second workspace registry exists;
- no new UX archetype exists;
- no workflow-specific route was added.

## 3. Discovery Coverage Model

Stage 4 remains:

```text
workspace
+
workflow
+
task/context
+
exception/result/status/history
```

The existing Batch-1 reviewer projection, Batch-2 end-user-safe runtime projection and Batch-3 workspace-local workflow discovery remain the canonical workflow-discovery foundation.

Batch 4 adds only the minimum remaining discovery seam for deterministic Stage-3 operational context.

No Workflow Explorer was added because the Stage-4 exit gate is satisfied through:

- workflow-aware Global Search;
- workspace-local workflow discovery;
- explicitly bound deterministic task/context discovery;
- exception/review-attention discovery where an explicit deterministic reference supports a responsible workspace;
- status/history context where an explicit deterministic reference supports a responsible workspace.

A 595-row or 1,084-row workflow explorer would add review density without closing a remaining discovery gap and is therefore explicitly deferred.

## 4. Task Discovery Treatment

Stage-3 operational task records remain deterministic demo/reviewer context and do not become workflow authority.

Batch 4 introduces explicit record-to-workspace bindings only for records whose existing deterministic references support one responsible workspace. The same evidence rule is applied across both canonical Stage-3 presets:

- `demo-task-review-aarav-student-context` → Student Workspace / W-C02;
- `demo-task-review-meera-assignment-context` → Faculty Academic Assignment / W-D02;
- `demo-task-exception-rich-meera-review` → Faculty Academic Assignment / W-D02.

The cross-domain task `demo-task-review-aarav-cross-domain-context` remains deliberately unbound because its references span Student, Attendance, Examination and Finance context and no governed task-to-workflow or single-workspace relationship exists.

No title similarity, free-text inference, guessed workflow ID or invented task-to-workflow mapping is used.

## 5. Exception / Result / Status / History Treatment

The deterministic review-attention records tied to the existing faculty academic-assignment reference are exposed contextually in W-D02 in both canonical presets, including `demo-exception-meera-assignment-review-attention` and `demo-exception-meera-course-review-attention`.

Cross-domain review-attention without one legitimate responsible destination remains unbound, including the exception-rich Aarav finance review-attention record.

The seeded Student Context activity record is surfaced as recent status/history context in W-C02.

All such records are labelled and described as deterministic demo context. They are explicitly not represented as:

- canonical exception lifecycles;
- authoritative results;
- audit events;
- SLAs;
- business-state authority;
- workflow identities.

Observation-only workflow semantics from Batch 2/3 remain unchanged.

## 6. Contextual Cross-Links and Owner Preservation

Interactive task discovery occurs only through Global Search when an explicit responsible workspace binding exists.

The destination always resolves through the existing workspace navigation mechanism.

No new route, workflow route, owner transfer or alternate workspace registry is introduced.

Workspace-local operational context is descriptive and intentionally non-interactive. It does not use action-implying CTA wording.

CROSS-WORKSPACE-HANDOFF workflows remain handoff-only and continue resolving through existing responsible workspace ownership.

## 7. Presentation Relevance

Workflow discovery continues using workflow-level `presentationPersonas` from the safe runtime projection.

Workspace visibility does not imply workflow relevance.

Operational-context search additionally respects the existing destination workspace presentation profile. This remains presentation relevance only and is not authorization, RBAC, permission, eligibility or business authority.

Student, Faculty and Administrator browser evidence covers both visible and hidden cases.

## 8. Reviewer / End-User Boundary

Ordinary workflow discovery continues importing `workflow-discovery-runtime.generated.ts`, never `workflow-discovery.generated.ts`.

The Batch-4 contextual surface exposes only human-readable demo titles/details and responsible workspace labels.

It does not expose:

- L6-RWF IDs;
- L3 IDs;
- L4 IDs;
- architecture paths;
- source keys;
- scenario arrays;
- traceability metadata;
- Stage-3 record IDs.

The reviewer-rich projection remains validation/reviewer material only.

## 9. Interaction-Semantics Audit

Every new local task/context row is descriptive and non-interactive.

No inert row uses imperative navigation/execution copy such as:

```text
Open
Continue
View
Go to
Review
Start
Approve
Reject
Retry
Resolve
Complete
Publish
Execute
Finalize
```

Global Search results for explicitly bound demo context are interactive and resolve to an existing workspace destination.

No Stage-5 business execution is introduced.

## 10. Observation / Background / Handoff Safety

Batch 4 preserves:

- `PROHIBITED` mock-interaction safety;
- `SYSTEM-BACKGROUND-OBSERVABLE` observation-only semantics;
- `NO-DIRECT-END-USER-INTERACTION` observation-only semantics;
- `CROSS-WORKSPACE-HANDOFF` handoff-only semantics;
- mixed-workflow scenario overrides.

No background workflow is represented as a human task merely to increase UI coverage.

## 11. Runtime Projection and Global Search Integrity

The ordinary product runtime remains the generated safe projection.

Global Search precedence remains:

```text
exact workflow-label match
↓
exact workspace/navigation match
↓
remaining workflow matches
↓
remaining destination matches
↓
linked demo operational context
↓
other sample records/work items
```

Batch-2 browser regression tests remain in CI and pass.

## 12. Batch-3 Workspace-Local Regression

The reusable Batch-3 workspace-local component remains in place and continues to:

- use the safe runtime projection;
- filter by canonical workspace key;
- filter by workflow-level presentation relevance;
- render deterministic ordering;
- preserve view-only and handoff labels;
- avoid workflow-specific routes;
- avoid reviewer metadata.

Batch-3 browser regression tests remain in CI and pass.

During the Batch-4 accessibility review an inherited invalid `aria-labelledby` reference was discovered in the Batch-3 section. The section now uses a valid accessible section label. This is a compatibility hardening fix only; no discovery semantics changed.

## 13. Stage-3 Regression and Authority

Batch 4 does not modify:

- canonical institutional seed;
- operational seed records;
- demo-state authority;
- deterministic reset;
- mock clock;
- presets;
- fixture-convergence adapters.

Stage-3 records are consumed as contextual review fixtures only.

## 14. Accessibility Assessment

Accessibility was reviewed separately from screenshot evidence.

Validated characteristics include:

- semantic section/list structure;
- valid accessible section names;
- keyboard traversal in Global Search;
- keyboard `Enter` activation for legitimate bound destinations;
- no clickable local context row without a legitimate destination;
- responsive narrow/mobile layout;
- no raw reviewer IDs in accessible ordinary-user copy;
- existing focus behavior retained;
- non-color-only labels retained for Workflow, Handoff, View only, Demo task, Review attention and Recent context.

Screenshots are visual evidence only and are not used as accessibility proof.

## 15. Browser Evidence

Dedicated Batch-4 browser coverage includes:

- mobile Student Workspace task/history context;
- Faculty Academic Assignment task/review-attention context;
- exception-rich Faculty task/review-attention context after live preset switching;
- exception-rich ambiguous Aarav task/review-attention remaining unbound;
- ambiguous cross-domain task remaining undiscoverable as a guessed destination;
- keyboard task discovery and actual navigation to an existing responsible workspace;
- observation-only workflow regression;
- handoff workflow regression;
- high-density Administrator workspace;
- empty contextual state;
- Batch-2 Global Search regression suite;
- Batch-3 workspace-local regression suite.

Final artifact:

```text
stage4-batch4-visual-evidence
Artifact ID: 10690335319
SHA-256: 9118e8357f3f16040606980c5c6d66e63da02bd8ff3a3fbedfa0a78b6e57e9d2
```

## 16. Validation Evidence

Final frontend SHA:

```text
eec69310c2078a2af7cd796439c6adff3cf86e8d
```

Final required CI:

```text
Frontend PR Validation #156
Run ID: 35716209191
State: COMPLETED
Conclusion: SUCCESS
```

Passed gates:

- `npm run review:validate` — PASS;
- `npm run stage4:validate` — PASS;
- `npm run mock:validate` — PASS;
- `npm run typecheck` — PASS;
- `npm run build` — PASS;
- Batch-2 browser regression — PASS;
- Batch-3 browser regression — PASS;
- Batch-4 browser suite — PASS, 8 / 8 tests;
- screenshot/evidence upload — PASS.

## 17. Cross-Repository Synchronization

CI executed the repository's provenance-only Stage-4 sync structure guard successfully.

In addition, the final Frontend provenance manifest was compared directly against current Architecture `main`. All three canonical Architecture source blob SHAs match exactly:

- Group-15 master reconciliation — MATCH;
- Workflow Realization Register — MATCH;
- Stage-4 Workflow Presentation Relevance Map — MATCH.

Canonical provenance counts remain 595 / 1,084 / 99 / 12 / 10.

The full local `npm run stage4:sync-check` command was not executed inside CI because the workflow does not mount a private Architecture checkout. The equivalent source-blob synchronization condition was independently verified against the live Architecture repository without weakening the guard.

## 18. Dead-Code / Cleanup Assessment

The final Batch-4 diff was reviewed before evidence freeze for:

- unused imports;
- unused components;
- dead helpers;
- unused CSS;
- temporary test scaffolding;
- debug code;
- stale comments;
- obsolete feature flags;
- duplicate validation logic;
- temporary files.

Typecheck/build are clean and no material Batch-4 dead scaffolding remains.

The browser-harness correction from the earlier failed CI #149 and the exception-rich test-harness corrections from failed runs #154 and #155 were committed before the final evidence SHA. Those failed runs are retained as history and are not used as pass evidence.

## 19. Risks, Open Questions and Assumptions

### Established limitation

Not every Stage-3 operational record has a legitimate single-workspace discovery destination.

This is intentional. Ambiguous cross-domain records remain reviewer/context evidence until upstream or later product architecture establishes a governed relationship.

### Assumption

The Stage-4 exit gate concerns discoverability of all user-addressable registered workflows, not mandatory task/exception/result channels for every workflow.

This matches the active Stage-4 plan and avoids manufacturing relationships.

### Non-blocking deferred item

A dedicated Workflow Explorer remains deferred because existing search + local discovery + contextual discovery satisfy the exit gate without creating a high-density 595-row browse surface.

## 20. Devil's-Advocate Review

The final review actively checked for:

- user-addressable workflow without discovery path;
- workflow relevance inherited from workspace visibility;
- task fixture promoted into workflow truth;
- title-similarity task/workflow inference;
- actionable-looking exception without action authority;
- background workflow represented as human work;
- ownership loss at handoff;
- inert action-looking copy;
- clickable item without legitimate destination;
- reviewer IDs in ordinary DOM/accessibility copy;
- reviewer-rich projection import;
- duplicate discovery registries;
- 595-item or 1,084-item flooding;
- scenario-per-row expansion;
- new routes;
- mobile overflow;
- keyboard dead ends;
- meaningless contextual sections;
- Global Search precedence regression;
- Batch-3 local discovery regression;
- Stage-3 state mutation;
- stale evidence SHA;
- evidence created before cleanup;
- non-terminal CI;
- screenshots treated as accessibility proof.

The previously identified two-preset discovery gap has been corrected. Current-head CI proves the exception-rich Faculty task/review-attention bindings, preserves ambiguous Aarav records as unbound, and retains all Batch-2/3 regressions. No material blocking finding remains.

## 21. Stage-4 Exit Gate

The repository-level Stage-4 discovery gate is satisfied on Frontend PR #21 head `eec69310c2078a2af7cd796439c6adff3cf86e8d`:

- every registered workflow has deterministic Global Search/runtime projection coverage;
- workspace-local workflow discovery remains available where presentation-relevant;
- explicit task/context discovery exists where a legitimate deterministic relationship exists;
- ambiguous task context is preserved rather than guessed;
- exception/status/history context is safely represented where supported;
- no broken new destination exists;
- background and prohibited workflows remain observation-only;
- handoffs remain owner-preserving;
- no 1,084-item navigation dump exists;
- Stage-3 authority remains intact;
- L0–L5 are unchanged by this evidence record;
- Stage 5 has not started.

## 22. Evidence Verdict

```text
STAGE 4 — BATCH 4 PASS
```

This evidence records a passing implementation state for review. It does not merge Frontend PR #21 or this Architecture evidence PR and does not authorize Stage 5.
