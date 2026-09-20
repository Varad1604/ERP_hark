# SutraERP L6 Stage 2 — Group 7 Section G2 Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section G — Campus Operations G2 (`L3-G-078 … L3-G-154`)  
**Scenario population:** 77  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G7-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Reconciliation Result

```text
Assigned canonical scenarios       77
Reconciled scenario rows           77 / 77
Unique scenario IDs                77
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      38
User-addressable workflows         38
Tracking / non-mock workflows      0
Upstream-blocked workflows         0
```

Group 7 is fully reconciled. The worker artifact was mechanically complete but materially under-classified human Hostel, Transport and Library activity as background/system-owned behavior.

## 2. Canonical Population and Ownership Check

The canonical range `L3-G-078 … L3-G-154` contains exactly 77 scenario identities with no gaps or duplicates and aligns to the locked 1,084-scenario ID baseline.

The range resolves to exactly three canonical L2 owners/workspaces:

- `L3-G-078 … L3-G-092` → L2-G03 Hostel Management → `W-G03`
- `L3-G-093 … L3-G-123` → L2-G04 Transport Management → `W-G04`
- `L3-G-124 … L3-G-154` → L2-G05 Library Management → `W-G05`

No workflow family crosses these workspace boundaries.

## 3. Semantic Reconciliation Findings

Worker G2 incorrectly treated many explicit business actions as `SYSTEM-BACKGROUND-OBSERVABLE`, `NO-DIRECT-END-USER-INTERACTION`, `PROHIBITED`, or non-mock exception paths.

Representative corrections verified against Locked L3 and the authoritative G03/G04/G05 workflows include:

- G080 requires Hostel Warden / Allocation Officer capacity evaluation, priority-rule application and waitlist authorisation; only rank calculation is automatic.
- G081 requires resident-facing exit clearance, Hostel Administrator verification and configured Warden governance before occupancy closure.
- G083 requires a resident request and Hostel Warden / configured-authority decision for authorised late entry.
- G088 requires complaint registration, Hostel Administrator/Warden review, path selection, verification and closure; maintenance execution remains G01-owned.
- G089 requires hostel inspection, findings, resolution selection and owner-preserving hand-offs to G01/F where applicable.
- G094, G095, G097, G103, G104, G107, G111, G116 and G119 all expose Transport Administrator/Coordinator/Supervisor/Manager and/or Driver actions or governed review.
- G110 and G121 are owner-preserving Transport → Facilities maintenance-recommendation hand-offs; G04 does not execute G01 maintenance.
- G120–G122 form one Transport Complaint lifecycle with alternate operational, maintenance-recommendation and driver/service corrective-action paths.
- G125/G126 require library suspension/closure review and configured authority.
- G127/G128 require catalogue preparation, validation, operational review and authorisation.
- G131/G132/G133 require member/circulation-desk issue, return and renewal activity.
- G135 includes member collection and circulation-desk validation; it is not only an availability notification.
- G146 requires Library Verification Authority plus verification personnel and an administrative closure decision.
- G147/G148 require investigation or administrative review following stock-verification findings.
- G152 requires an authoritative Library Management administrative resolution, using C/F source facts without re-owning them.
- G154 requires report preparation, validation, configured business review and governance/administrative outcome.

Two scenario outcomes remain genuinely system-observable within larger user-addressable workflows:

- `L3-G-136` — reservation expiry/closure after the governed expiry condition;
- `L3-G-137` — overdue/fine calculation from authoritative due-date and fine policy.

No fake “expire reservation” or “calculate fine” action is introduced.

`L3-G-150` is an owner-preserving Library-clearance outcome hand-off to C/E institutional exit processes. Its approved/completed clearance fact is consumed downstream; the downstream lifecycle remains source-owned and is not given fake G05 controls.

## 4. Shared Configuration Governance Consumption

The 2026-09-20 A01 configuration-governance completion applies consistently to:

- `L3-G-092` — Hostel operational configuration;
- `L3-G-123` — Transport operational configuration;
- `L3-G-153` — Library operational configuration.

The owner-domain semantics remain G03/G04/G05 respectively. A01 supplies the generic lifecycle, owner-scoped Configuration Approval Authority and generic framework lifecycle-event publication.

No G03/G04/G05-specific duplicate configuration lifecycle or generic-event copy is introduced.

## 5. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 33 |
| REVIEW-APPROVAL-WORKFLOW | 28 |
| EXCEPTION-RECOVERY-WORKFLOW | 11 |
| CROSS-WORKSPACE-HANDOFF | 3 |
| SYSTEM-BACKGROUND-OBSERVABLE | 2 |
| **Total** | **77** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 74 |
| NOT-REQUIRED | 1 |
| PROHIBITED | 2 |
| **Total** | **77** |

The `NOT-REQUIRED` row is G150 downstream clearance consumption. The two `PROHIBITED` rows are G136 reservation expiry and G137 fine calculation. They remain visible/inspectable inside their normalized user-addressable workflows.

## 6. Workflow Normalization Method

Scenarios were merged only where they share the same primary workspace, canonical owner, aggregate/lifecycle context and sustained institutional job.

Normalization intentionally:

- merges create/change/suspend/close variants where they are one owned lifecycle;
- merges complaint variants into one owner-controlled complaint journey;
- retains Route and Transport Stop as distinct lifecycle families;
- retains owner-preserving Facilities hand-offs as hand-offs rather than G04 maintenance execution;
- keeps Library clearance ownership in G05 while exposing downstream consumption;
- keeps automatic expiry/calculation steps observable within larger workflows without creating fake actions; and
- preserves G03/G04/G05 configuration semantics while consuming the shared A01 framework.

## 7. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G7-WF-001 | W-G03 | Hostel admission, waitlist, residency activation and closure lifecycle | 4 | L3-G-078; L3-G-079; L3-G-080; L3-G-081 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-002 | W-G03 | Resident leave and verified-return lifecycle | 1 | L3-G-082 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-003 | W-G03 | Curfew exception and authorised-late-entry decision | 1 | L3-G-083 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-004 | W-G03 | Hostel visitor request, visit and closure | 1 | L3-G-084 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-005 | W-G03 | Hostel discipline and curfew-violation resolution | 2 | L3-G-085; L3-G-086 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G7-WF-006 | W-G03 | Room transfer and reallocation | 1 | L3-G-087 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-007 | W-G03 | Hostel complaint review, resolution and closure | 1 | L3-G-088 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-008 | W-G03 | Hostel room inspection and damage resolution | 1 | L3-G-089 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G7-WF-009 | W-G03 | Mess allocation and reallocation lifecycle | 2 | L3-G-090; L3-G-091 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-010 | W-G03 | Hostel operational-configuration governance | 1 | L3-G-092 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-011 | W-G04 | Transport Route creation, modification, sequencing, suspension and permanent revision | 5 | L3-G-093; L3-G-094; L3-G-095; L3-G-098; L3-G-119 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-012 | W-G04 | Transport Stop creation and suspension lifecycle | 2 | L3-G-096; L3-G-097 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-013 | W-G04 | Transport allocation assignment, change and withdrawal | 3 | L3-G-099; L3-G-100; L3-G-101 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-014 | W-G04 | Driver assignment and replacement | 2 | L3-G-102; L3-G-103 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-015 | W-G04 | Trip planning, readiness and pre-execution cancellation | 2 | L3-G-104; L3-G-105 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-016 | W-G04 | Trip execution, route deviation and completion | 2 | L3-G-106; L3-G-107 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-017 | W-G04 | Vehicle operational assignment and breakdown replacement | 2 | L3-G-108; L3-G-109 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-018 | W-G04 | Vehicle operational issue to G01 maintenance recommendation | 1 | L3-G-110 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G7-WF-019 | W-G04 | Pre-trip and post-trip operational checks | 2 | L3-G-111; L3-G-112 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-020 | W-G04 | Transport Pass issue, renewal, replacement and suspension lifecycle | 4 | L3-G-113; L3-G-114; L3-G-115; L3-G-116 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-021 | W-G04 | Emergency route diversion and temporary-stop adjustment | 2 | L3-G-117; L3-G-118 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G7-WF-022 | W-G04 | Transport Complaint review and corrective-resolution lifecycle | 3 | L3-G-120; L3-G-121; L3-G-122 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-023 | W-G04 | Transport operational-configuration governance | 1 | L3-G-123 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-024 | W-G05 | Library Membership activation, suspension and closure lifecycle | 3 | L3-G-124; L3-G-125; L3-G-126 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-025 | W-G05 | Catalogue creation, review, publication and update lifecycle | 2 | L3-G-127; L3-G-128 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-026 | W-G05 | Library Copy registration and condition restriction lifecycle | 2 | L3-G-129; L3-G-130 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-027 | W-G05 | Circulation issue, return and renewal lifecycle | 3 | L3-G-131; L3-G-132; L3-G-133 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-028 | W-G05 | Reservation, availability/collection and expiry lifecycle | 3 | L3-G-134; L3-G-135; L3-G-136 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-029 | W-G05 | Overdue fine and waiver governance | 2 | L3-G-137; L3-G-138 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-030 | W-G05 | Lost or damaged library-resource recovery | 2 | L3-G-139; L3-G-140 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G7-WF-031 | W-G05 | Library acquisition approval, receipt and catalogued-copy establishment | 2 | L3-G-141; L3-G-142 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-032 | W-G05 | Reading Room opening, usage and operational closure | 3 | L3-G-143; L3-G-144; L3-G-145 | INTERACTIVE-WORKFLOW | REQUIRED |
| G7-WF-033 | W-G05 | Stock verification, discrepancy investigation and administrative review | 3 | L3-G-146; L3-G-147; L3-G-148 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-034 | W-G05 | Library clearance approval and institutional-exit hand-off | 2 | L3-G-149; L3-G-150 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G7-WF-035 | W-G05 | Resource-availability conflict and alternative resolution | 1 | L3-G-151 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G7-WF-036 | W-G05 | Borrowing restriction administrative resolution | 1 | L3-G-152 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-037 | W-G05 | Library operational-configuration governance | 1 | L3-G-153 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G7-WF-038 | W-G05 | Library operational reporting and governance review | 1 | L3-G-154 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |

## 8. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 17 |
| REVIEW-APPROVAL-WORKFLOW | 14 |
| EXCEPTION-RECOVERY-WORKFLOW | 5 |
| CROSS-WORKSPACE-HANDOFF | 2 |
| **Total** | **38** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 38 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 0 |
| **Total** | **38** |

The workflow-level mock count differs from scenario-row mock counts because G136/G137 are automatic sub-outcomes inside user-addressable reservation/fine workflows, and G150 is an automatic owner-preserving hand-off inside a clearance workflow that also contains a human approval.

## 9. Cross-Domain / Workspace Observations

- Hostel complaint and room-damage flows may hand maintenance requirements to G01; maintenance execution remains G01-owned.
- Transport vehicle/complaint maintenance recommendations may hand work to G01; G04 never receives maintenance-work-order ownership.
- Student/personnel/Finance facts consumed by G03/G04/G05 remain source-owned.
- Library clearance remains G05-owned even when C/E consumes the approved/completed outcome.
- Library acquisition remains a G05 acquisition decision; procurement execution remains outside G05 and no J-owned procurement controls are fabricated inside this workflow.
- Notification delivery remains H-owned and is not represented as a G-domain action.
- The generic A01 configuration framework is reused by G092/G123/G153 without changing domain-policy ownership.

## 10. Canonical Identity Check

Deterministic range reconciliation:

```text
Expected first ID        L3-G-078
Expected last ID         L3-G-154
Expected count           77
Observed scenario rows   77
Unique primary IDs       77
Missing IDs              0
Duplicate primary IDs    0
Primary workflow count   38
Scenario assignments     77 / 77 exactly once
Workspace set            W-G03, W-G04, W-G05
```

## 11. Group Exit Gate

| Gate | Result |
|---|---|
| G2 canonical scenarios = 77 | PASS |
| Reconciled realization rows = 77 | PASS |
| Unique IDs = 77 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Every scenario assigned to one normalized workflow | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Distinct workflows discovered | PASS — 38 |
| User-addressable workflows discovered | PASS — 38 |
| Mock-required workflow population discovered | PASS — 38 |
| Material upstream blockers = 0 | PASS |
| A01 configuration governance consumed without ownership transfer | PASS |
| No invented actor/lifecycle/event/publisher | PASS |
| No implementation leakage | PASS |

**GROUP 7 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

Stage 2 may continue to Group 8. The canonical Workflow Realization Register remains intentionally unpopulated until Groups 1–14 and master cross-group reconciliation complete.
