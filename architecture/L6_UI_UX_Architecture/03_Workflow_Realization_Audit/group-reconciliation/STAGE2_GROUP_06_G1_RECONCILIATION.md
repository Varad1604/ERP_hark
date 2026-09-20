# SutraERP L6 Stage 2 — Group 6 Section G1 Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section G — Campus Operations G1 (`L3-G-001 … L3-G-077`)  
**Scenario population:** 77  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G6-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Reconciliation Result

```text
Assigned canonical scenarios       77
Reconciled scenario rows           77 / 77
Unique scenario IDs                77
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      39
User-addressable workflows         37
Tracking / non-mock workflows      2
Upstream-blocked workflows         0
```

The G1 population is fully reconciled and Group 6 now passes its group exit gate after canonical G00 exception-recovery governance and shared A01 configuration governance were resolved upstream and re-derived through L3/L4.

## 2. Former Upstream Blockers — Resolved

Group 6 originally identified two material upstream blockers:

1. `L3/L4-G-013` lacked authoritative recovery-selection roles/guards and a canonical non-escalation recovery event.
2. `L3/L4-G-014` lacked the owner-scoped approval authority and generic lifecycle-event semantics required by the shared A01 configuration framework.

They are now resolved through:

- ADR-G-EXCEPTION-001;
- locked `L2-G00_Exception_Recovery_Governance_Amendment_2026-09-20.md`;
- ADR-PLAT-CONFIG-001;
- locked `L2-A01_Configuration_Governance_Completion_Amendment_2026-09-20.md`;
- targeted L3 revalidation; and
- re-derived L4 behavior / parent-readiness reconciliation.

G013 now uses Campus Operations Manager for ordinary configured recovery, Executive Reviewer for Administrative Override/escalated recovery, and `GovernanceExceptionRecoveryRecorded` for authorized non-escalation recovery.

G014 now uses the shared A01 lifecycle with Configuration Approval Authority acting on behalf of Campus Operations Governance for G00-owned domain configuration. A01 remains sole publisher of generic configuration lifecycle events. G00 retains semantic ownership of its configuration values.

## 3. Semantic Reconciliation Findings

Worker G1 was mechanically complete but substantially under-classified operational Campus work. **56 scenario rows** required a disposition and/or mock-interaction correction.

Evidence-backed corrections include:

- G00 Operational Risk, Multi-Domain Action Plan, SLA deviation governance and Executive Governance Resolution all expose named governance actors and decisions; they are not background-only.
- G00 accreditation evidence preparation is an owner-preserving G00→H09 handoff rather than a hidden background process.
- Facilities Complaint, Work Order, reopening, reassignment, preventive/corrective/emergency maintenance, vendor, AMC, inspection, SLA and configuration flows all expose Maintenance Supervisor / Facilities Manager / Technician or vendor actions.
- Preventive-maintenance deferral explicitly requires a Facilities Manager authorization and reason.
- Work Order reopening explicitly requires supervisor authorization and successor verification.
- Utility reading itself remains trackable/non-mock, but outage/anomaly recovery is user-addressable.
- Laboratory readiness, practical-session preparation, scheduling/batch allocation, equipment restriction, safety, incident, inspection, compliance and recovery paths expose Laboratory Coordinator / Laboratory In-Charge / Laboratory Assistant / safety/compliance responsibilities.
- Laboratory incident → maintenance recommendation is an owner-preserving G02→G01 handoff; G02 recommends and G01 owns execution.
- Equipment and consumable usage history remain tracking-oriented where no user decision is required beyond recording/observation.
- G013 and G014 are not given artificial controls because their missing upstream authority/event semantics are material.

No G00/G01/G02 owner, actor authority, lifecycle, state, event, rule, cross-domain ownership or implementation mechanism was invented.

## 4. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 18 |
| REVIEW-APPROVAL-WORKFLOW | 27 |
| TRACKING-WORKFLOW | 5 |
| EXCEPTION-RECOVERY-WORKFLOW | 24 |
| CROSS-WORKSPACE-HANDOFF | 3 |
| **Total** | **77** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 72 |
| NOT-REQUIRED | 5 |
| PROHIBITED | 0 |
| **Total** | **77** |

`L3-G-013` is now an exception/recovery workflow with governed mock interaction. `L3-G-014` is now a review/approval workflow using the shared A01 configuration-governance contract.

## 5. Workflow Normalization Method

Scenarios were merged only when they share the same primary workspace, canonical owner, sustained institutional job and lifecycle context.

Normalization specifically avoided:

- merging G00 governance with G01/G02 operational ownership;
- converting operational alerts into invented approvals;
- treating source-domain maintenance execution as G02-owned;
- treating G00 governance review as permission to mutate source-owned operational aggregates;
- hiding real supervisor/coordinator/technician actions behind "background" labels; and
- manufacturing UI actions for the two blocked upstream authority gaps.

No normalized Group-6 workflow crosses a workspace boundary.

## 6. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G6-WF-001 | W-G00 | Campus readiness assessment and governance decision | 1 | L3-G-001 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-002 | W-G00 | Accreditation evidence preparation and H09 handoff | 2 | L3-G-002; L3-G-015 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G6-WF-003 | W-G00 | Campus Operational Review lifecycle | 1 | L3-G-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-004 | W-G00 | Executive governance review and recommendation | 1 | L3-G-004 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-005 | W-G00 | Operational Risk lifecycle | 1 | L3-G-005 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-006 | W-G00 | Multi-Domain Action Plan lifecycle | 1 | L3-G-006 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-007 | W-G00 | Multi-domain SLA deviation governance | 1 | L3-G-007 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-008 | W-G00 | Campus Operational Health assessment | 1 | L3-G-008 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-009 | W-G00 | Governance KPI review | 1 | L3-G-009 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-010 | W-G00 | Operational Maturity Assessment | 1 | L3-G-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-011 | W-G00 | Governance Finding and corrective-action decision | 1 | L3-G-011 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-012 | W-G00 | Executive Governance Resolution | 1 | L3-G-012 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-013 | W-G00 | Governance evidence/dependency exception recovery | 1 | L3-G-013 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G6-WF-014 | W-G00 | Governance configuration lifecycle | 1 | L3-G-014 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-015 | W-G01 | Facility Service Request and administrative closure | 2 | L3-G-016; L3-G-046 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-016 | W-G01 | Facility Complaint lifecycle, duplicate consolidation and split work | 3 | L3-G-017; L3-G-018; L3-G-019 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-017 | W-G01 | Work Order lifecycle, reopening, reassignment and replanning | 5 | L3-G-020; L3-G-021; L3-G-022; L3-G-047; L3-G-048 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-018 | W-G01 | Preventive Maintenance cycle, deferral and missed-cycle recovery | 3 | L3-G-023; L3-G-024; L3-G-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-019 | W-G01 | Corrective maintenance, root-cause and repeat-failure improvement | 3 | L3-G-026; L3-G-027; L3-G-028 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-020 | W-G01 | Emergency maintenance, incident intervention and post-incident review | 3 | L3-G-029; L3-G-030; L3-G-045 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G6-WF-021 | W-G01 | Vendor assignment, service failure and performance evaluation | 3 | L3-G-031; L3-G-032; L3-G-033 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-022 | W-G01 | AMC service execution, failure and renewal action | 3 | L3-G-034; L3-G-035; L3-G-036 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-023 | W-G01 | Utility monitoring, outage and anomaly recovery | 3 | L3-G-037; L3-G-038; L3-G-039 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G6-WF-024 | W-G01 | Facility Inspection lifecycle | 1 | L3-G-040 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-025 | W-G01 | Facility SLA governance and operational escalation | 2 | L3-G-041; L3-G-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-026 | W-G01 | Maintenance consumable and operational-cost history | 2 | L3-G-043; L3-G-044 | TRACKING-WORKFLOW | NOT-REQUIRED |
| G6-WF-027 | W-G01 | Facilities operational-configuration governance | 1 | L3-G-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-028 | W-G02 | Laboratory readiness, warning and availability recovery | 3 | L3-G-050; L3-G-051; L3-G-052 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-029 | W-G02 | Practical Session operational lifecycle | 3 | L3-G-053; L3-G-054; L3-G-055 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-030 | W-G02 | Laboratory scheduling, batch allocation and reallocation | 5 | L3-G-056; L3-G-057; L3-G-058; L3-G-059; L3-G-074 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-031 | W-G02 | Equipment operational status, restriction and usage | 3 | L3-G-060; L3-G-061; L3-G-062 | INTERACTIVE-WORKFLOW | REQUIRED |
| G6-WF-032 | W-G02 | Laboratory safety verification, warning and emergency stabilization | 3 | L3-G-063; L3-G-064; L3-G-075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-033 | W-G02 | Laboratory Incident resolution | 1 | L3-G-065 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G6-WF-034 | W-G02 | Laboratory incident maintenance-recommendation handoff to G01 | 1 | L3-G-066 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G6-WF-035 | W-G02 | Laboratory Inspection, findings and controlled follow-up | 3 | L3-G-067; L3-G-068; L3-G-076 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-036 | W-G02 | Laboratory Compliance and non-compliance resolution | 2 | L3-G-069; L3-G-070 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G6-WF-037 | W-G02 | Laboratory consumable-usage history | 1 | L3-G-071 | TRACKING-WORKFLOW | NOT-REQUIRED |
| G6-WF-038 | W-G02 | Laboratory operational-restriction / utility-failure recovery | 2 | L3-G-072; L3-G-073 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G6-WF-039 | W-G02 | Laboratory operational-configuration governance | 1 | L3-G-077 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |

## 7. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 12 |
| REVIEW-APPROVAL-WORKFLOW | 18 |
| TRACKING-WORKFLOW | 2 |
| EXCEPTION-RECOVERY-WORKFLOW | 5 |
| CROSS-WORKSPACE-HANDOFF | 2 |
| **Total** | **39** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 37 |
| NOT-REQUIRED | 2 |
| PROHIBITED | 0 |
| **Total** | **39** |

## 8. Cross-Group / Repository Impact

The A01 remediation also resolves the generic Platform/Tenant configuration-governance gaps previously recorded by `L3/L4-A-056`. Group 1's normalized A056 workflow count does not change, but its upstream basis is now fully authoritative.

The final master cross-group reconciliation should continue to check other scenarios that reference generic A01 configuration governance and consume the new locked amendment where relevant. No earlier PASS-group workflow count is changed by this remediation.

## 9. Group Exit Gate

| Gate | Result |
|---|---|
| G1 canonical scenarios = 77 | PASS |
| Reconciled realization rows = 77 | PASS |
| Unique IDs = 77 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Distinct workflows discovered | PASS — 39 |
| User-addressable workflows discovered | PASS — 37 |
| Material upstream blockers = 0 | PASS |
| G013 canonical frozen outcome safely realizable | PASS |
| G014 canonical frozen outcome safely realizable | PASS |

**GROUP 6 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

The former G013/G014 upstream blockers are resolved at L2, synchronized through L3/L4, and revalidated. Stage 2 may continue to Group 7.
