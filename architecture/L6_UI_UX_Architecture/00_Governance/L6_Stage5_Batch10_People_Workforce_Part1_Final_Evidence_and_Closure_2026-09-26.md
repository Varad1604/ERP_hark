# SutraERP L6 — Stage 5 Batch 10 — People & Workforce Part 1 — Final Evidence & Closure

**Version:** 1.0.0  
**Status:** Architecture Closure Pending  
**Evidence Type:** Batch Closure Evidence  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Decision Date:** 2026-09-26  
**Batch Result:** PASS  
**Frontend State:** PASS / COMPLETE / MERGED  
**Architecture Closure State:** Pending merge of this closure PR  
**Domain Result:** Section E — People & Workforce PARTIALLY COMPLETE; Batch 11 remains  
**Next Batch:** Batch 11 ENTRY-READY / NOT STARTED only after Architecture closure  

---

## 1. Closure Principle

This Batch-10 closure does **not** maintain a moving frontend or Architecture `main` SHA.

Canonical immutable evidence is the Git/GitHub history of:
- frontend PR #35 and its merge event;
- exact validated frontend PR head;
- exact PR-head CI run and browser artifact;
- the Architecture closure PR containing this document.

No post-merge PR shall be created merely to write the merge SHA of the Architecture PR into this document.

---

## 2. Frontend Evidence

Batch-10 frontend implementation:

```text
PR #35 — L6 Stage 5 Batch 10 — People & Workforce Part 1
State: MERGED
Merged: 2026-09-26
Validated PR head:
82597364bd86ca301d4728b0bde7524006aff31c
```

Exact PR-head validation:

```text
Workflow: Frontend PR Validation
Run number: 336
Run ID: 36223482636
Attempt: 1
Event: pull_request
Conclusion: SUCCESS
```

Browser evidence:

```text
Batch-10 browser suite: 27 / 27 PASS

Artifact:
stage5-batch10-browser-evidence

Artifact ID:
10899668280
```

The browser suite executes all 21 REQUIRED workflows, proves all five PROHIBITED workflows remain observation-only, covers duplicate safety, reset behavior, W-E01 and W-E02 exception-preset isolation, W-E02 bulk partial-result semantics, keyboard-only execution, focus transfer, desktop/mobile containment, Faculty read-only visibility and architecture-metadata leakage checks.

---

## 3. Scope

Stage 5 Batch 10 covers Section E — People & Workforce Part 1:

- W-E01 — Workforce & Engagement
- W-E02 — Position & Assignment
- W-E03 — Faculty Workforce

Exact realization range:

```text
L6-RWF-0121 … L6-RWF-0146
```

Batch 11 begins at `L6-RWF-0147` / W-E04. No Batch-11 implementation was introduced.

---

## 4. Coverage and Dispositions

```text
Workflows:        26 / 26
Scenarios:        35 / 35
Workspaces:        3 / 3

REQUIRED:         21
PROHIBITED:        5
NOT-REQUIRED:      0
Mixed workflows:   0
```

Binding result:

| Workflow | Binding Count |
|---|---:|
| L6-RWF-0121 | 0 |
| L6-RWF-0122 | 1 |
| L6-RWF-0123 | 1 |
| L6-RWF-0124 | 1 |
| L6-RWF-0125 | 1 |
| L6-RWF-0126 | 0 |
| L6-RWF-0127 | 0 |
| L6-RWF-0128 | 0 |
| L6-RWF-0129 | 1 |
| L6-RWF-0130 | 1 |
| L6-RWF-0131 | 1 |
| L6-RWF-0132 | 1 |
| L6-RWF-0133 | 1 |
| L6-RWF-0134 | 1 |
| L6-RWF-0135 | 1 |
| L6-RWF-0136 | 1 |
| L6-RWF-0137 | 0 |
| L6-RWF-0138 | 1 |
| L6-RWF-0139 | 1 |
| L6-RWF-0140 | 1 |
| L6-RWF-0141 | 1 |
| L6-RWF-0142 | 1 |
| L6-RWF-0143 | 1 |
| L6-RWF-0144 | 1 |
| L6-RWF-0145 | 1 |
| L6-RWF-0146 | 1 |

Therefore:
- all 21 REQUIRED workflows have exactly one legitimate executable binding;
- `0121`, `0126`, `0127`, `0128`, and `0137` remain PROHIBITED with zero executable bindings;
- all five PROHIBITED workflows have concrete positive observation provenance and status/history/evidence;
- executable-binding counting is distinct from read-only observation provenance.

---

## 5. Authority Basis

The exact Batch-10 parent population is:

```text
L3-E-001 … L3-E-032
L3-E-071
L3-E-072
L3-E-073
```

Live authority reconstruction established:

```text
35 / 35 L3 parents = Locked
35 / 35 corresponding L4 specifications = Draft
```

Accordingly, implementation authority was derived from locked upstream business architecture and protected L6 governance, not from Draft L4 as approved behavior authority.

No direct L4→L5 relation was fabricated where canonical evidence did not establish one. General downstream L5 applicability was not rewritten as direct parent traceability.

---

## 6. Protected Stage-5 Corpus

Protected sources remained unchanged:

| Protected Source | Blob SHA | Result |
|---|---|---|
| Stage-2 Group-15 master reconciliation | `22dd6e0f38067936c7171c7e4386ebb58564f3f8` | MATCH |
| L6 Workflow Realization Register | `53448e586a066c08745ec6be27f9e58016f34483` | MATCH |
| Stage-4 Workflow Presentation Relevance Map | `b47748cb2cbf9b692d41fbfea2f147f63159bb7a` | MATCH |
| `DEPENDENCY_MAP.md` | `6d0dd83dbe4e0b015858ec54b336f4803c329c36` | MATCH |
| Stage-5 reviewed gameplan | `bc9dd50ff148607b7bd6d45def8bb27961f84b8a` | MATCH |
| Frontend generated Stage-5 census | `5514e9fc6400e72ce24f10e497fdb605673fa581` | MATCH |

Protected census remains:

```text
595 workflows
1,084 canonical L3 scenarios
99 workspaces
12 UX archetypes
10 mixed workflows
```

The Stage-5 census was not regenerated.

---

## 7. Ownership Boundaries

### W-E01 — Workforce & Engagement

Preserved:
- E01 owns Workforce Engagement and workforce operational-availability facts;
- E01 owns engagement modification, suspension/restoration, deputation, closure/rejoining history and governed Workforce Profile resolution;
- Person remains Platform/Foundation-owned;
- organisation structure remains Organisation-owned;
- E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship and Assignment History.

Organisational transfer, suspension, deputation, long leave, exit and rejoining do not silently mutate E02-owned assignment aggregates.

### W-E02 — Position & Assignment

Preserved:
- E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship, Assignment History, service jurisdiction, assignment delegation/additional charge, committee membership where canonically represented, correction/rollback, freeze/restoration and assignment-derived organisational authority;
- Position Definition remains Organisation-owned;
- Workforce Engagement remains E01-owned;
- IAM authorization-layer delegation remains separate from E02 assignment delegation.

Runtime organisational identity is represented through active Position Assignment context rather than directly through Person, user account or security role.

### W-E03 — Faculty Workforce

Preserved:
- E03 owns faculty academic-workforce participation, Faculty Capability, qualification association for academic use, Teaching Eligibility, availability, workload and capacity visibility;
- HR employment remains outside E03;
- Position Assignment remains E02-owned;
- Academic Delivery owns actual teaching assignment;
- Timetable/Scheduling owns when and where teaching occurs.

No timetable scheduler, faculty-course allocation engine, workload optimizer, credential-verification API, payroll logic or HR employee lifecycle was introduced.

---

## 8. Positive PROHIBITED Observability

The five PROHIBITED workflows remain non-actionable while exposing concrete evidence:

- `0121` — Workforce Engagement establishment and activation;
- `0126` — Long-leave operational availability;
- `0127` — Workforce Engagement closure and exit consequence;
- `0128` — Workforce rejoining consequence;
- `0137` — Organisational authority resolution.

Each has:
- zero executable operation binding;
- explicit source-workflow provenance;
- concrete business status;
- outcome/history/context evidence;
- no user-executed command path.

---

## 9. State and Operation Architecture

Batch 10 preserves:

```text
one DemoState authority
one mock-operation framework
one central operation catalog / registry
one result/history model
one reset/preset authority
one Global Experience Lab / Stage3ReviewerContext authority
one workflow-discovery authority
```

No second state root, alternate store, browser-storage authority, backend service, API simulation or persistence mechanism was introduced.

Reviewer behavior remains deterministic, mock-operational, reviewer-local where applicable and non-production.

---

## 10. UX, Accessibility and Browser Evidence

Final exact PR-head validation passed:
- review-model validation;
- Stage-4 workflow discovery validation;
- Stage-4 provenance validation;
- mock seed/state/preset validation;
- Stage-5 provenance validation;
- full Stage-5 validation;
- strengthened Batch-10 hard validator;
- TypeScript typecheck;
- production build;
- all prior Stage-5 browser regressions;
- Batch-10 browser suite — 27 / 27 PASS;
- all 21 REQUIRED workflows executed through browser evidence;
- all five PROHIBITED workflows proved observation-only;
- W-E01 / W-E02 / W-E03 keyboard-only flows;
- focus management and outcome focus;
- exception-rich preset isolation;
- duplicate protection and reset behavior;
- bulk Best-Effort partial/per-item result proof;
- 390px mobile bulk containment after independent-audit remediation;
- desktop/mobile containment;
- Faculty read-only projection;
- metadata-leakage checks;
- artifact upload.

---

## 11. Independent Audit and Remediation

Chunk 10 independently reconstructed Batch 10 and treated implementation/tests as untrusted.

Material findings corrected:
1. naive string-based binding counting;
2. missing explicit prohibited-observation provenance;
3. incomplete duplicate/preset/bulk browser evidence;
4. real 390px Bulk Position Assignment page overflow;
5. missing global proof that prohibited workflows remain observation-only;
6. hard validator did not initially require 21 distinct workflow-specific business outcomes;
7. validator-pattern defect introduced during strengthening and corrected before final PASS.

The independent audit also confirmed:
- all 35 L3 parents are Locked;
- all 35 L4 parents are Draft;
- Draft L4 was not promoted;
- no second state/operation/reset/workflow-discovery authority exists;
- no backend/API/browser-persistence authority was introduced;
- no uncontrolled nondeterminism was introduced;
- no Batch-11 implementation contamination exists.

Final unresolved independent-audit defects:

```text
0
```

---

## 12. Repository Integrity Review

| Gate | Result |
|---|---|
| Correct L6 boundary | PASS |
| Exact 26-workflow cohort | PASS |
| Exact 35-scenario cohort | PASS |
| Exact 3-workspace cohort | PASS |
| REQUIRED bindings | PASS — 21 / 21 |
| PROHIBITED zero bindings | PASS — 5 / 5 |
| Positive PROHIBITED observability | PASS — 5 / 5 |
| Protected corpus | PASS — unchanged |
| Census regeneration | PASS — not performed |
| Locked L3 authority | PASS — 35 / 35 |
| Draft L4 not promoted | PASS |
| Missing direct L4→L5 relations not fabricated | PASS |
| E01 / E02 ownership precedence | PASS |
| E02 / Organisation / IAM boundaries | PASS |
| E03 / HR / E02 / Academic Delivery / Timetable boundaries | PASS |
| Single state / operation / reset authority | PASS |
| Backend/API/persistence introduction | PASS — none |
| Determinism and preset isolation | PASS |
| Assignment history/correction lineage | PASS |
| Bulk per-item/partial-result semantics | PASS |
| Accessibility / responsive evidence | PASS |
| Prior-batch regression | PASS |
| Frontend canonical merge | PASS — PR #35 merged |
| Batch 11 implementation | PASS — not started |

---

## 13. Closure Decision

Current state before Architecture closure merge:

```text
STAGE 5 BATCH 10 — PASS
FRONTEND — PASS / COMPLETE / MERGED
FRONTEND PR #35 — MERGED

SECTION E PART-1 COHORT — 26 / 26 WORKFLOWS COMPLETE
SECTION E PART-1 SCENARIOS — 35 / 35 ACCOUNTED FOR
WORKSPACES — 3 / 3
REQUIRED — 21 / 21
PROHIBITED — 5 / 5 ZERO-BINDING WITH POSITIVE OBSERVATION
NOT-REQUIRED — 0
MIXED — 0
BROWSER / CI — PASS

ARCHITECTURE CLOSURE — PENDING THIS PR
DOMAIN E — PARTIALLY COMPLETE
BATCH 11 — NOT STARTED
```

After this Architecture closure PR is explicitly authorized and merged, the canonical repository state becomes:

```text
STAGE 5 BATCH 10 — PASS / COMPLETE / MERGED
PEOPLE & WORKFORCE PART 1 — COMPLETE
DOMAIN E — PARTIALLY COMPLETE — BATCH 11 REMAINING
BATCH 11 — ENTRY-READY / NOT STARTED
```

No separate PR shall be created merely to record this Architecture PR's merge SHA.
