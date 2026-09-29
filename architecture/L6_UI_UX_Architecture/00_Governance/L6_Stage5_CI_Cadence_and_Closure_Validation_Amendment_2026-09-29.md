# SutraERP L6 — Stage 5 CI Cadence and Closure Validation Amendment — 2026-09-29

**Version:** 1.0.0  
**Status:** Proposed Governance Amendment  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support Governance  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Decision Date:** 2026-09-29  
**Amends:** Stage-5 execution cadence only  
**Does Not Amend:** L0–L5 authority, protected workflow/scenario/workspace counts, canonical ownership, workflow dispositions, lifecycle states, events, rules, or final validation obligations.

---

## 1. Purpose

Reduce repeated full-corpus browser CI during response-safe Stage-5 chunk execution while preserving complete exact-head validation before canonical batch closure.

The Stage-5 consolidation standard already requires each bounded chunk to run targeted validators/tests and reserves integrated domain validation for the batch integration/closure gate. This amendment makes the CI cadence consistent with that execution model.

---

## 2. Decision

Stage-5 CI shall use two validation cadences:

### 2.1 Automatic pull-request validation

Ordinary pull-request updates shall run the fast integration gate:

- dependency installation;
- review-model validation;
- Stage-4 validation;
- Stage-4 provenance validation;
- mock-seed validation;
- Stage-5 provenance validation;
- Stage-5 baseline validation;
- TypeScript typecheck;
- production build.

The accumulated full Playwright/browser-evidence corpus is not required on every intermediate PR synchronization event.

### 2.2 Explicit full closure validation

The complete browser/evidence suite shall remain available through an explicit workflow-dispatch run against the exact branch/head proposed for batch integration or closure.

A Stage-5 domain batch shall not receive canonical closure solely from the fast PR gate. Final closure requires the applicable full exact-head validation to PASS.

---

## 3. Chunk Execution Rule

For normal response-safe chunks:

```text
bounded implementation
→ targeted static/browser validation relevant to the chunk
→ commit/checkpoint
→ automatic fast PR integration validation where a PR exists
→ continue
```

The full accumulated browser suite is reserved for:

- final domain integration;
- batch closure;
- material shared/global seam reconciliation;
- cross-batch runtime convergence where required;
- Batches 17–20 convergence/reconciliation/final gates;
- any defect investigation where broad regression evidence is materially necessary.

This does not authorize skipping targeted tests for the changed scope.

---

## 4. Self-Hosted Runner Conservation

The SutraERP frontend uses a dedicated self-hosted Linux x64 runner.

To reduce redundant queue time and compute contention:

- superseded automatic PR runs for the same PR should be cancelled when a newer commit arrives;
- one runner instance remains the default execution model;
- parallel Stage-5 authoring does not require parallel full CI execution;
- queued jobs may serialize on the single runner;
- full browser validation should not be triggered merely because a conversational chunk completed.

This is an execution-efficiency rule, not a reduction in validation quality.

---

## 5. Active Parallel Batch Rule

Where Batch 15 Examination and Batch 16 Enterprise Resource Management execute concurrently:

- each retains an isolated branch/PR;
- unfinished implementation from either batch is not architecture authority for the other;
- chunk-local validation remains targeted;
- shared/global seams are reconciled centrally;
- Batch 16 final closure remains subject to the locked dependency-safe parallel-execution convergence rule for any Examination-dependent seam;
- final exact-head full validation remains mandatory before canonical closure.

---

## 6. Evidence Classification

Validation evidence shall be interpreted as follows:

```text
TARGETED PASS
= changed bounded scope has passed its applicable focused validation

FAST PR PASS
= static/provenance/typecheck/build integration gate has passed

FULL CLOSURE PASS
= complete accumulated browser/evidence suite has passed on the exact closure candidate head
```

Only **FULL CLOSURE PASS**, together with the applicable architecture and independent-review gates, can satisfy the final Stage-5 batch CI closure requirement.

A missing full closure run shall be reported as **CI-PENDING**, not silently treated as PASS.

---

## 7. Repository Integrity

This amendment does not alter:

- 595-workflow accountability;
- 1,084 scenario mappings;
- 99 workspace identities;
- 12 archetypes;
- REQUIRED / NOT-REQUIRED / PROHIBITED dispositions;
- canonical owners;
- deterministic state/reset requirements;
- browser-evidence requirements at closure;
- independent batch audit requirements;
- Stage-6 entry criteria.

It only changes **when** the expensive accumulated browser suite runs.

---

## 8. Decision Summary

```text
AUTOMATIC PR GATE        = FAST INTEGRATION VALIDATION
CHUNK VALIDATION         = TARGETED TO CHANGED SCOPE
FULL BROWSER CI          = EXPLICIT CLOSURE / CONVERGENCE RUN
SUPERSEDED PR RUNS       = CANCEL
SINGLE SELF-HOSTED RUNNER = RETAIN
FINAL EXACT-HEAD PASS    = MANDATORY
CI-PENDING               = REQUIRED WHEN FINAL FULL RUN HAS NOT PASSED
VALIDATION COVERAGE      = NOT REDUCED
L0–L5 SEMANTIC CHANGE    = NONE
```
