# SutraERP L6 — Stage 5 Batch 13 — Campus Operations — Final Evidence & Closure

**Version:** 1.0.0  
**Status:** Architecture Approved — Terminal Closure Evidence  
**Evidence Type:** Batch Closure Evidence  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Decision Date:** 2026-09-28  
**Batch Result:** PASS  
**Frontend State:** PASS / COMPLETE / MERGED  
**Architecture Closure State:** TERMINAL CLOSURE PR — READY FOR VALIDATION / MERGE  
**Domain Result:** Section G — Campus Operations COMPLETE at Stage-5 domain-realization scope  

---

## 1. Closure Principle

This Batch-13 closure does **not** maintain a moving frontend or Architecture `main` SHA.

Canonical immutable evidence is the Git/GitHub history of:
- frontend PR #39;
- exact validated frontend PR head `9d7dc3688755e713da92580aefc1add34725b9a6`;
- exact PR-head CI run #429 / run ID `36384446493`;
- browser artifact `stage5-batch13-browser-evidence`;
- canonical Architecture PR #125 resolving the G07 physical-security-asset ownership contradiction;
- this single terminal Architecture closure change.

Merging the terminal Architecture closure PR containing this document canonicalizes Batch 13 at repository level.

No follow-up PR shall be created merely to write Architecture or frontend merge SHAs back into this document.

---

## 2. Frontend Evidence

Batch-13 frontend implementation:

```text
PR #39 — L6 Stage 5 Batch 13 — Campus Operations
State: MERGED
```

Validated PR head:

```text
9d7dc3688755e713da92580aefc1add34725b9a6
```

Exact PR-head validation:

```text
Workflow: Frontend PR Validation
Run number: 429
Run ID: 36384446493
Conclusion: SUCCESS
```

Browser evidence:

```text
Artifact:
stage5-batch13-browser-evidence

Artifact ID:
10954485456

Artifact digest:
sha256:db5687f362e86659c4f12a7100758421b2b1cc097865bf8fdfee89940e7001fd
```

The successful run passed:
- review-model validation;
- Stage-4 workflow-discovery validation;
- Stage-4 provenance validation;
- mock seed/state/preset validation;
- Stage-5 provenance validation;
- all Batch-13 bounded chunk validators;
- integrated Batch-13 corpus validation;
- final 16-workflow Finance convergence validation;
- TypeScript typecheck;
- production build;
- all accumulated prior Stage-5 browser regressions;
- all five targeted Batch-13 Campus browser tests;
- browser artifact upload.

---

## 3. Scope

Stage 5 Batch 13 covers Section G — Campus Operations:

- W-G00 — Campus Operations Governance
- W-G01 — Facilities
- W-G02 — Laboratories
- W-G03 — Hostel
- W-G04 — Transport
- W-G05 — Library
- W-G06 — Health / Medical Services
- W-G07 — Security Operations
- W-G08 — Housekeeping & Service Governance
- W-G09 — Teacher Guardian Scheme
- W-G10 — Infrastructure Management

Exact realization range:

```text
L6-RWF-0235 … L6-RWF-0388
```

This is the consolidated Campus Operations governance batch established by the approved Stage-5 remaining-batch topology. Former Campus implementation groupings are internal execution chunks only and are not independent governance batches.

---

## 4. Coverage and Dispositions

Canonical Campus population:

```text
Workflows:        154 / 154
Scenarios:        320 / 320
Workspaces:        11 / 11
```

Scenario dispositions:

```text
REQUIRED:         312
NOT-REQUIRED:       6
PROHIBITED:         2
```

Workflow-level disposition position:

```text
REQUIRED-only workflows:       147
MIXED workflows:                 5
NOT-REQUIRED-only workflows:     2
```

Final executable/non-executable realization:

```text
Executable workflow bindings:                  152
Standalone NOT-REQUIRED executable bindings:     0 / 2
```

The two standalone NOT-REQUIRED workflows remain observation/history only:

```text
L6-RWF-0260 — Maintenance consumable and operational-cost history
L6-RWF-0271 — Laboratory consumable-usage history
```

The five mixed workflows preserve scenario-level overrides:

```text
L6-RWF-0257
  L3-G-037 — NOT-REQUIRED
  L3-G-038 — REQUIRED
  L3-G-039 — REQUIRED

L6-RWF-0265
  L3-G-060 — REQUIRED
  L3-G-061 — REQUIRED
  L3-G-062 — NOT-REQUIRED

L6-RWF-0301
  L3-G-134 — REQUIRED
  L3-G-135 — REQUIRED
  L3-G-136 — PROHIBITED

L6-RWF-0302
  L3-G-137 — PROHIBITED
  L3-G-138 — REQUIRED

L6-RWF-0307
  L3-G-149 — REQUIRED
  L3-G-150 — NOT-REQUIRED
```

No PROHIBITED background outcome acquires a fabricated user action.

---

## 5. Authority Basis

Primary Batch-13 authority used for closure includes:
- locked Section-G L2 Campus Operations architecture;
- canonical Section-G L3 scenario population;
- Stage-2 master cross-group reconciliation;
- L6 workflow realization register;
- Stage-4 workflow-discovery evidence;
- Stage-5 reviewed gameplan;
- Stage-5 remaining-batch consolidation decision;
- Stage-5 parallel-batch execution amendment;
- Dependency Map;
- canonical Batch-12 Finance implementation;
- canonical G07 physical-security-asset ownership correction.

Repository evidence, not conversation assumptions, determined final ownership and behavior.

No L0–L5 ownership, lifecycle, event, rule or publisher was silently redefined.

---

## 6. Canonical Ownership Boundaries Preserved

### Campus Governance

G00 owns Campus governance assessments, reviews, risks, action plans, findings, resolutions and governance configuration.

Source-domain operational facts remain with their canonical owners.

### Facilities

G01 owns Facilities service operations, maintenance execution, Facilities work orders, inspections and Facilities operational policy.

Preserved:
- G10 owns physical infrastructure identity, capacity, availability and lifecycle;
- Section J owns procurement and enterprise inventory/resource truth;
- Finance owns monetary/accounting/payment truth;
- Section E owns workforce identity and engagement.

### Laboratories

G02 owns laboratory readiness, operational sessions, operational equipment status, safety, incidents, inspections, compliance and laboratory policy.

Preserved:
- G10 owns laboratory physical spaces/infrastructure;
- Academic Delivery owns academic scheduling;
- Section J owns enterprise inventory/procurement/resource lifecycles;
- G01 owns maintenance work-order execution.

### Hostel

G03 owns hostel admission/residency, leave/curfew, visitors, discipline, operational room allocation, complaints, inspections and mess assignment.

Student identity, physical infrastructure, workforce, maintenance and Finance ownership remain external where established.

### Transport

G04 owns Transport operational configuration, routes/stops, operational allocations, driver/trip/vehicle assignment, passes, diversions and complaints.

Finance remains authoritative for Transport Billing, payment, refund, ledger and accounting consequences.

### Library

G05 owns Library membership, catalogue, Library Copy, circulation, reservation, operational fine calculation, acquisition workflow, Reading Room operations, collection verification and Library clearance.

Preserved:
- Finance owns payment/accounting;
- Section C owns Student identity/lifecycle;
- Section E owns workforce identity/engagement;
- procurement execution remains external;
- G10 owns physical Library / Reading Room infrastructure lifecycle.

Background reservation expiry and fine calculation remain non-user-invocable where classified PROHIBITED.

### Health / Medical Services

G06 owns institutional healthcare operations, Medical Encounters, Emergency Cases, Prescriptions, Medicine Dispense, Observation Stays, Referrals, Medical Certificates, health programmes, vaccination/checkup records and medical clearance.

Preserved:
- Section C owns Student identity/lifecycle;
- Section E owns healthcare workforce identity/engagement;
- G01 owns biomedical/facility maintenance execution;
- Finance owns Medical Billing, payment, receipts, insurance settlement and accounting;
- medical confidentiality and minimum-necessary disclosure controls remain intact.

Lightweight institutional medicine inventory is a G06 operational healthcare fact and was not silently reassigned to Section J.

### Security Operations

G07 owns operational Security activities including gates, access, patrols, incidents, emergency coordination, Lost & Found, escorts and operational delegation.

Canonical Architecture correction PR #125 resolved the earlier contradictory G07 sentence:

```text
Physical Security Assets — G01 Facilities Management
```

Accordingly:
- G07 may consume physical security-asset availability/status operationally;
- G07 does not own or mutate the physical security-asset lifecycle;
- Section J remains owner of supplier/procurement contract truth where applicable;
- Finance remains owner of accounting/payment consequences;
- Section E or authorized external security agencies remain owners of personnel identity/employment facts.

### Housekeeping

G08 owns Housekeeping service delivery, cleaning/hygiene operations, consumable monitoring, waste coordination, incidents, service quality, vendor operational coordination and SLA outcomes.

Preserved:
- Section E / authorized vendor owns personnel identity/employment;
- G01 owns maintenance execution;
- Section J owns supplier/procurement, Vendor Contract and enterprise inventory truth;
- Finance owns accounting/payment consequences;
- G06 retains medical authority for biomedical-waste classification/clinical handling where applicable.

### Teacher Guardian Scheme

G09 owns mentoring relationships, mentorship cases, mentoring discussions/notes/guidance, parent interactions, referral records, follow-ups, mentorship history and case status.

Preserved:
- Section C owns Student identity/lifecycle;
- Academic Delivery owns Teacher Guardian assignment and academic scheduling/summaries;
- Section E owns Faculty/Teacher Guardian workforce facts;
- Finance context is read-only or referral-only;
- Hostel, Library and Medical context remains source-owned and minimum-necessary.

### Infrastructure Management

G10 owns the canonical Physical Campus, physical hierarchy, spaces, capacity/availability, infrastructure reservation/allocation, physical compliance/condition and physical lifecycle.

No other Campus workspace creates a competing physical-campus truth.

---

## 7. Finance Convergence

The 2026-09-27 Parallel-Batch Execution Amendment required Batch 13 to revalidate Finance-facing runtime/read-model seams after Batch 12 became canonical.

Batch 12 Finance was merged before final Batch-13 closure validation.

A dedicated Finance-convergence validator ran on the Finance-canonical Batch-13 branch.

Exact result:

```text
Batch 13 Finance convergence PASS —
16 / 16 Campus workflows revalidated against canonical
Batch-12 Finance registration and ownership boundaries.
```

Finance-facing workflow register:

```text
L6-RWF-0286
L6-RWF-0289
L6-RWF-0293
L6-RWF-0296

L6-RWF-0302
L6-RWF-0303
L6-RWF-0307

L6-RWF-0313
L6-RWF-0314
L6-RWF-0316
L6-RWF-0318
L6-RWF-0319

L6-RWF-0353
L6-RWF-0354

L6-RWF-0359
L6-RWF-0361
```

Final Finance/Campus integration preserves:
- one global operation catalog;
- Finance operation registrations;
- Campus operation registrations;
- Finance settlement/accounting authority;
- no Campus assertion of payment settled, refund settled, ledger posted, vendor paid or accounting completed.

---

## 8. Enterprise Resource / Section-J Convergence Position

Batch 13 records seven Section-J-facing workflows for later Section-J Stage-5 domain convergence:

```text
L6-RWF-0260
L6-RWF-0265
L6-RWF-0271
L6-RWF-0304
L6-RWF-0348
L6-RWF-0353
L6-RWF-0354
```

These workflows preserve Section-J ownership now.

They do not create a second Campus-owned supplier, Purchase Order, procurement, enterprise inventory or operational asset truth.

Unlike the Finance parallel-wave gate, the approved Batch-13 instruction requires these workflows to be recorded for later Section-J convergence; current Batch-13 closure does not claim Section-J Stage-5 implementation completion.

---

## 9. State, Operation and Product Integrity

Batch 13 preserves:

```text
one DemoState authority
one mock-operation framework
one central operation catalog / registry
one result/history model
one reset/preset authority
one workflow-discovery authority
```

A material integration weakness was found during G-C9:

```text
Initial Campus workflow surfaces were statically represented
but not yet globally registered for mock-operation execution.
```

This was remediated before closure.

Final state:
- all executable Campus workflows use the existing central mock-operation framework;
- no Campus-specific state engine was created;
- deterministic action history is recorded through the existing DemoState authority;
- reset reconstructs clean state;
- preset switching remains centralized;
- duplicate state authority was not introduced.

Ordinary Campus product UX contains no L3/L4/L6 workflow IDs, Batch numbers, chunk IDs or reviewer identifiers.

---

## 10. Browser and Accessibility-Oriented Evidence

Five targeted Campus browser tests passed:

1. Facilities governed action executes deterministically and resets cleanly;
2. standalone observation-only workflows expose no fake business action;
3. Finance-facing Campus UX does not assert Finance completion;
4. ordinary Campus product UX contains no architecture/reviewer identifiers;
5. Teacher Guardian Scheme remains usable at 390px narrow width.

The accumulated prior Stage-5 browser regression suite also passed.

Browser evidence includes:
- desktop rendering;
- narrow/mobile rendering;
- deterministic governed-action behavior;
- reset behavior;
- focus-supported governed action flow inherited from the established Stage-5 pattern;
- no horizontal-product architecture leakage in targeted Campus evidence.

This closure does not claim a separate comprehensive production WCAG certification.

---

## 11. Independent Audit and Remediation

A separate full-Batch audit was performed during G-C9.

Audit challenges included:
- missing workflows;
- duplicate workflow realization;
- incorrect dispositions;
- mixed-workflow override loss;
- ownership leakage;
- duplicate state authority;
- Finance overreach;
- Section-J overreach;
- Academic scheduling overreach;
- Student ownership leakage;
- Workforce ownership leakage;
- fake background interactions;
- dead workspace destinations;
- inconsistent terminology;
- reviewer metadata leakage;
- weak reset/preset behavior;
- false-positive validators;
- browser/mobile evidence gaps.

Material findings remediated before closure:

1. **Static-only realization gap**  
   Campus definitions existed but were not yet globally registered as executable mock operations.  
   **Resolution:** all executable Campus operations were added to the single existing operation catalog and governed-action path.

2. **Product reviewer/architecture leakage**  
   Some early bounded workspace surfaces rendered workflow/scenario identifiers.  
   **Resolution:** reviewer identifiers were removed from ordinary product UX.

3. **Validator false positives**  
   Early validators treated explanatory phrases such as “never asserts payment settled” as forbidden positive assertions.  
   **Resolution:** validators now distinguish explicit negation/boundary language from unsafe completion assertions.

4. **Incomplete Enterprise Resource convergence register**  
   Initial convergence tracking omitted Facilities/Laboratory resource seams.  
   **Resolution:** the final seven-workflow Section-J convergence register includes those seams.

5. **G07 physical-security-asset ownership contradiction**  
   One locked G07 sentence conflicted with G07's own dependency/ownership matrix and G01 evidence.  
   **Resolution:** Architecture issue #124 was governed explicitly; PR #125 corrected the contradictory sentence to G01 Facilities Management and was merged before final Batch-13 validation.

Final material product/architecture defects:

```text
0
```

---

## 12. Repository Diff Integrity

The Finance-converged Batch-13 branch was rebuilt from canonical frontend `main` after Batch 12 merged.

Final pre-merge diff audit:

```text
Files changed:                    43
Batch-13 additions:               38
Intentional shared modifications:  5
Deleted files:                     0
Unexpected unrelated files:        0
Commits behind canonical main:      0
```

The five shared seams were:
- frontend PR-validation workflow;
- mock-operation catalog;
- mock-operation exports;
- home/workspace routing;
- package validation scripts.

They were reconciled to preserve both Finance and Campus behavior.

The original pre-Finance Batch-13 PR #38 was closed unmerged after Finance changed shared seams.

The clean Finance-converged replacement PR #39 became the canonical Batch-13 frontend integration vehicle.

---

## 13. Repository Integrity Review

| Gate | Result |
|---|---|
| Correct L6 boundary | PASS |
| Exact 154-workflow cohort | PASS |
| Exact 320-scenario cohort | PASS |
| Exact 11-workspace cohort | PASS |
| Executable workflow bindings | PASS — 152 |
| Standalone NOT-REQUIRED zero bindings | PASS — 2 / 2 |
| Mixed workflow overrides | PASS — 5 / 5 |
| PROHIBITED sub-scenario fake actions | PASS — none |
| Campus canonical ownership | PASS |
| Student boundary | PASS |
| Workforce boundary | PASS |
| Academic boundary | PASS |
| Finance boundary | PASS |
| Finance post-merge convergence | PASS — 16 / 16 |
| Section-J resource boundary | PASS |
| Section-J later convergence register | PASS — 7 |
| G07 physical-security-asset ownership | PASS — canonical G01 correction |
| Single state / operation / reset authority | PASS |
| Deterministic reset / preset behavior | PASS |
| Reviewer metadata leakage | PASS — none in ordinary UX |
| Typecheck | PASS |
| Production build | PASS |
| Accumulated Stage-5 browser regression | PASS |
| Batch-13 targeted browser evidence | PASS — 5 / 5 |
| Browser artifact upload | PASS |
| Independent audit | PASS |
| Repository diff integrity | PASS |

---

## 14. Domain and Programme Status

Therefore:

```text
SECTION G — CAMPUS OPERATIONS
W-G00 … W-G10 — STAGE-5 DOMAIN REALIZATION COMPLETE
```

This statement is limited to Stage-5 mock-operational UI realization.

It does not imply:
- production backend integration;
- real payment/settlement integration;
- production procurement/inventory integration;
- production medical or security-system integration;
- Stage-6 completion;
- Section-J Stage-5 completion.

Section-J convergence remains a later programme responsibility using the seven recorded Campus-facing seams.

---

## 15. Closure Decision

Canonical Batch-13 state:

```text
STAGE 5 BATCH 13 — PASS / COMPLETE / MERGED
FRONTEND — PASS / COMPLETE / MERGED
FRONTEND PR #39 — MERGED

CAMPUS OPERATIONS — COMPLETE
W-G00 … W-G10 — COMPLETE
WORKFLOWS — 154 / 154
SCENARIOS — 320 / 320
WORKSPACES — 11 / 11

EXECUTABLE WORKFLOW BINDINGS — 152
STANDALONE NOT-REQUIRED — 2 / 2 ZERO EXECUTABLE BINDING
MIXED WORKFLOWS — 5 / 5 OVERRIDES PRESERVED

FINANCE CONVERGENCE — 16 / 16 PASS
SECTION-J CONVERGENCE REGISTER — 7 WORKFLOWS RECORDED

G07 PHYSICAL SECURITY ASSET OWNER — G01 FACILITIES MANAGEMENT
ARCHITECTURE ISSUE #124 — RESOLVED
ARCHITECTURE PR #125 — MERGED

TYPECHECK — PASS
BUILD — PASS
BROWSER / EXPERIENCE LAB — PASS
TARGETED CAMPUS BROWSER TESTS — 5 / 5 PASS
INDEPENDENT AUDIT — PASS
REPOSITORY DIFF AUDIT — PASS

SECTION G — CAMPUS OPERATIONS — COMPLETE AT STAGE-5 DOMAIN-REALIZATION SCOPE
```

Upon merge of this single terminal Architecture closure PR:

```text
STAGE 5 BATCH 13 — CANONICALLY CLOSED
CAMPUS OPERATIONS — STAGE-5 DOMAIN REALIZATION COMPLETE
```

No further Batch-13 status/provenance PR is required.
