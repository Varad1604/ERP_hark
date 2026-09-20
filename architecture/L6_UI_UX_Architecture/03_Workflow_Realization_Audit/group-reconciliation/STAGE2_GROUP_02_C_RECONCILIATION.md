# SutraERP L6 Stage 2 — Group 2 Section C Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section C — Student Lifecycle  
**Scenario population:** 75  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G2-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Completion Result

```text
Assigned canonical scenarios       75
Reconciled scenario rows           75 / 75
Unique scenario IDs                75
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      32
User-addressable workflows         27
Observable non-interactive         5
```

## 2. Semantic Reconciliation Findings

The worker baseline materially over-classified Student Lifecycle behavior as background-only.

Evidence-backed corrections were made where Locked L2/L3 establish real requests, approvals, administrative actions, recovery operations or owner-preserving handoffs.

Key examples:

- Admission revocation/readmission became governed user-facing workflows.
- Student restoration and duplicate resolution became mock-interactive recovery workflows where source actors/approvals exist.
- Inter-Institute transfer and Student profile/preferences became interactive.
- Credit transfer, programme migration, academic break/rejoining and progression-policy governance became review/approval workflows.
- Qualification completion/conferment and governed exit variants became review/approval workflows.
- Institutional clearance remains tracking because source domains own their clearance decisions.
- Guardian relationship modification/suspension/reinstatement/termination became user-facing where C06 supports aggregate modification.
- Scholarship definition/policy, assessment, reassessment, renewal and bulk recalculation now follow the explicit C07 business-role matrix.
- External scholarship synchronization and expiry/archive remain non-interactive.
- Scholarship eligibility to Finance is classified as a cross-workspace handoff.
- Alumni transition uses its explicit approval workflow and is therefore user-facing.

No upstream actor, state, event, business rule or authority was invented.

## 3. Scenario-Level Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 27 |
| REVIEW-APPROVAL-WORKFLOW | 23 |
| TRACKING-WORKFLOW | 2 |
| EXCEPTION-RECOVERY-WORKFLOW | 8 |
| CROSS-WORKSPACE-HANDOFF | 1 |
| SYSTEM-BACKGROUND-OBSERVABLE | 14 |
| NO-DIRECT-END-USER-INTERACTION | 0 |
| **Total** | **75** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 58 |
| NOT-REQUIRED | 3 |
| PROHIBITED | 14 |
| **Total** | **75** |

## 4. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G2-WF-001 | W-C01 | Admission status lifecycle | 5 | L3-C-002; L3-C-003; L3-C-004; L3-C-005; L3-C-006 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-002 | W-C01 | Admission document verification | 1 | L3-C-007 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-003 | W-C02 | Student activation | 1 | L3-C-001 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-004 | W-C02 | Student enrollment and assignment | 4 | L3-C-008; L3-C-009; L3-C-012; L3-C-013 | INTERACTIVE-WORKFLOW | REQUIRED |
| G2-WF-005 | W-C02 | Student suspension and restoration | 2 | L3-C-010; L3-C-011 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G2-WF-006 | W-C02 | Student profile and communication preferences | 2 | L3-C-014; L3-C-015 | INTERACTIVE-WORKFLOW | REQUIRED |
| G2-WF-007 | W-C02 | Student identity and duplicate reconciliation | 2 | L3-C-016; L3-C-017 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G2-WF-008 | W-C02 | Student document and certificate request | 1 | L3-C-018 | INTERACTIVE-WORKFLOW | REQUIRED |
| G2-WF-009 | W-C02 | Student context archival | 1 | L3-C-019 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G2-WF-010 | W-C03 | Academic registration window | 1 | L3-C-020 | INTERACTIVE-WORKFLOW | REQUIRED |
| G2-WF-011 | W-C03 | Academic registration lifecycle | 5 | L3-C-021; L3-C-022; L3-C-023; L3-C-024; L3-C-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| G2-WF-012 | W-C03 | Registration hold lifecycle | 1 | L3-C-026 | INTERACTIVE-WORKFLOW | REQUIRED |
| G2-WF-013 | W-C04 | Academic progression decision lifecycle | 5 | L3-C-027; L3-C-029; L3-C-030; L3-C-031; L3-C-038 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-014 | W-C04 | Academic credit ledger | 2 | L3-C-028; L3-C-037 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G2-WF-015 | W-C04 | Credit transfer and recognition | 1 | L3-C-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-016 | W-C04 | Programme migration | 1 | L3-C-033 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-017 | W-C04 | Academic break and rejoining | 2 | L3-C-034; L3-C-035 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-018 | W-C04 | Qualification eligibility determination | 1 | L3-C-036 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G2-WF-019 | W-C04 | Academic progression policy governance | 1 | L3-C-075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-020 | W-C05 | Qualification completion and conferment | 4 | L3-C-039; L3-C-041; L3-C-049; L3-C-050 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-021 | W-C05 | Student exit lifecycle | 7 | L3-C-040; L3-C-042; L3-C-043; L3-C-044; L3-C-045; L3-C-047; L3-C-048 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-022 | W-C05 | Institutional exit clearance | 1 | L3-C-046 | TRACKING-WORKFLOW | NOT-REQUIRED |
| G2-WF-023 | W-C06 | Guardian relationship lifecycle | 5 | L3-C-051; L3-C-052; L3-C-059; L3-C-060; L3-C-061 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-024 | W-C06 | Guardian custody and consent governance | 2 | L3-C-053; L3-C-054 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-025 | W-C06 | Guardian communication and responsibility governance | 2 | L3-C-055; L3-C-057 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-026 | W-C06 | Emergency and pickup authorization | 2 | L3-C-056; L3-C-058 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-027 | W-C07 | Scholarship definition and policy lifecycle | 2 | L3-C-062; L3-C-063 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G2-WF-028 | W-C07 | Scholarship eligibility assessment | 5 | L3-C-064; L3-C-065; L3-C-066; L3-C-067; L3-C-070 | INTERACTIVE-WORKFLOW | REQUIRED |
| G2-WF-029 | W-C07 | Scholarship eligibility expiry and archive | 1 | L3-C-068 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G2-WF-030 | W-C07 | External scholarship synchronization | 1 | L3-C-069 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G2-WF-031 | W-C07 | Scholarship eligibility to financial aid handoff | 1 | L3-C-071 | CROSS-WORKSPACE-HANDOFF | NOT-REQUIRED |
| G2-WF-032 | W-C08 | Alumni transition lifecycle | 3 | L3-C-072; L3-C-073; L3-C-074 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |

## 5. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 7 |
| REVIEW-APPROVAL-WORKFLOW | 16 |
| TRACKING-WORKFLOW | 1 |
| EXCEPTION-RECOVERY-WORKFLOW | 2 |
| CROSS-WORKSPACE-HANDOFF | 1 |
| SYSTEM-BACKGROUND-OBSERVABLE | 5 |
| NO-DIRECT-END-USER-INTERACTION | 0 |
| **Total** | **32** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 25 |
| NOT-REQUIRED | 2 |
| PROHIBITED | 5 |
| **Total** | **32** |

## 6. Group Exit Gate

| Gate | Result |
|---|---|
| Section-C canonical scenarios = 75 | PASS |
| Reconciled realization rows = 75 | PASS |
| Unique IDs = 75 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Material open issues = 0 | PASS |
| Distinct workflows discovered | PASS — 32 |
| User-addressable workflows discovered | PASS — 27 |
| Mock-required workflows discovered | PASS — 25 |
| Observable non-interactive workflows discovered | PASS — 5 |

**GROUP 2 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

Stage 2 remains in progress because Groups 3–14 and final master cross-group reconciliation/canonical-register population remain incomplete.
