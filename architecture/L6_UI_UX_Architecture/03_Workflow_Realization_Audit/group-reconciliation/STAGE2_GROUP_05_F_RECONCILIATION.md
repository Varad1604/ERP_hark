# SutraERP L6 Stage 2 — Group 5 Section F Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section F — Finance  
**Scenario population:** 85  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G5-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Reconciliation Result

```text
Assigned canonical scenarios       85
Reconciled scenario rows           85 / 85
Unique scenario IDs                85
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      59
User-addressable workflows         47
Observable non-interactive         12
Upstream-blocked workflows         0
```

The Section-F population is fully reconciled and Group 5 now passes its group exit gate after canonical F13 analytics-definition governance was resolved upstream and re-derived through L3-F-076 and L4-F-076.

## 2. Former Upstream Blocker — Resolved

Stage-2 Group 5 originally identified `L3-F-076 / L4-F-076` as materially blocked because F13 did not define KPI Definition / Analytical Model lifecycle, approval authority, or lifecycle events.

This has now been resolved by:

- `ADR-FIN-ANALYTICS-001`;
- locked `L2-F13_KPI_and_Analytical_Model_Governance_Amendment_2026-09-20.md`;
- targeted L3-F-076 revalidation; and
- re-derived L4-F-076 behavior.

KPI Definition and Analytical Model now consume the canonical Finance configuration lifecycle `Draft → Validation → Approval → Published → Active → Superseded → Archived`. `Financial Analytics Definition Approval Authority` is the canonical business approval authority. Aggregate-specific approval, activation, supersession and archival events each have exactly one canonical publisher.

F085 remains valid under its separate locked 2026-09-01 Financial Rule Governance amendment.

## 3. Semantic Reconciliation Findings

Worker F was mechanically complete but under-classified a substantial amount of legitimate Finance work. **36 scenario rows** required disposition and/or mock-interaction correction.

Evidence-backed corrections include:

- Student-charge correction is an owner-preserving handoff into canonical Financial Adjustment governance; charge disputes are user-addressable exception work.
- Unidentified receipts and payment failures require governed exception handling where the scenario establishes reconciliation/manual-resolution work; purely completed receipt/allocation outcomes remain observable.
- Financial Adjustment reversal and corrected-successor scenarios have explicit Reversal/Correction Authority, Enterprise Approval Authority and publisher responsibilities.
- Posting Batch, Journal reversal and Ledger Reconciliation expose explicit Accountant / Senior Accountant / Finance Manager governance and are not background-only.
- Budget revision/virement and Budget closure require governed maker-checker / closure decisions even where exact institutional role mapping remains upstream-configured.
- AR dunning and reopening expose Collection / Reopening Review Authority.
- AP dispute/hold, suspension/reopening, change incorporation and payable-population reconciliation expose governed AP review work.
- Financial Year/Period establishment, year-end close, period reopening/reclose and archival all expose explicit F09 authorities.
- Bank reconciliation, inter-account transfer and failed/uncertain bank-transaction resolution expose Treasury work; completed settlement/cash facts remain owner-preserving.
- Asset capitalization, revaluation, impairment and financial write-off require governed Finance decisions; periodic depreciation and source-owned disposal accounting consequences remain background.
- Statutory reporting is a cross-workspace F13→H08 handoff with H08 approval/submission governance.
- Financial Report disposition and official reporting use the locked F13 approval authority.
- Procurement Finance commitment, invoice matching, liability recognition and accounting closure all require governed Finance decisions.
- Finance Configuration and Financial Rule changes are review/approval workflows; F085 uses the later locked F00 rule-governance amendment.
- Automatic ledger postings, derived balance/charge consequences, periodic calculations, historical retrieval and projection generation remain non-interactive where the source does not establish a user decision.

No new Finance actor, approval chain, state, event, rule, ownership or implementation mechanism was invented.

## 4. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 7 |
| REVIEW-APPROVAL-WORKFLOW | 39 |
| TRACKING-WORKFLOW | 3 |
| EXCEPTION-RECOVERY-WORKFLOW | 15 |
| CROSS-WORKSPACE-HANDOFF | 3 |
| SYSTEM-BACKGROUND-OBSERVABLE | 17 |
| NO-DIRECT-END-USER-INTERACTION | 1 |
| **Total** | **85** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 61 |
| NOT-REQUIRED | 6 |
| PROHIBITED | 18 |
| **Total** | **85** |

`L3-F-076` is now realized as a governed review/approval workflow using the resolved canonical F13 lifecycle and Financial Analytics Definition Approval Authority.

## 5. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G5-WF-001 | W-F01 | Student Charge establishment, schedule and financial closure | 3 | L3-F-001; L3-F-005; L3-F-006 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-002 | W-F01 | Published Student Charge correction handoff | 1 | L3-F-002 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G5-WF-003 | W-F01 | Source-owned Student Charge lifecycle consequence | 1 | L3-F-003 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-004 | W-F01 | Student Charge dispute resolution | 1 | L3-F-004 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-005 | W-F02 | Payment receipt and allocation | 2 | L3-F-007; L3-F-008 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-006 | W-F02 | Unidentified / unallocated receipt reconciliation | 1 | L3-F-009 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-007 | W-F02 | Payment failure, timeout, reversal and chargeback recovery | 2 | L3-F-010; L3-F-011 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-008 | W-F02 | Payment cancellation | 1 | L3-F-012 | INTERACTIVE-WORKFLOW | REQUIRED |
| G5-WF-009 | W-F02 | Bulk collection outcome | 1 | L3-F-013 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-010 | W-F03 | Refund request, approval, execution and cancellation | 3 | L3-F-014; L3-F-015; L3-F-016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-011 | W-F03 | Refund failure, reversal, recovery and history | 3 | L3-F-017; L3-F-018; L3-F-019 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-012 | W-F04 | Credit, debit and write-off Financial Adjustment | 3 | L3-F-020; L3-F-021; L3-F-022 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-013 | W-F04 | Financial Adjustment reversal and corrected successor | 2 | L3-F-023; L3-F-024 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-014 | W-F05 | Automatic financial transaction ledger posting | 1 | L3-F-025 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-015 | W-F05 | Manual Journal governance | 1 | L3-F-026 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-016 | W-F05 | Accounting Posting Batch | 1 | L3-F-027 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-017 | W-F05 | Posted Journal reversal | 1 | L3-F-028 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-018 | W-F05 | Ledger reconciliation | 1 | L3-F-029 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-019 | W-F05 | Trial Balance generation | 1 | L3-F-030 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-020 | W-F05 | Accounting close-readiness handoff | 1 | L3-F-031 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G5-WF-021 | W-F05 | Closed-ledger audit and historical evidence | 1 | L3-F-032 | TRACKING-WORKFLOW | NOT-REQUIRED |
| G5-WF-022 | W-F06 | Budget proposal, revision/virement and closure | 3 | L3-F-033; L3-F-034; L3-F-038 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-023 | W-F06 | Budget allocation | 1 | L3-F-035 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-024 | W-F06 | Budget control decision | 1 | L3-F-036 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-025 | W-F06 | Cost Center lifecycle | 1 | L3-F-037 | INTERACTIVE-WORKFLOW | REQUIRED |
| G5-WF-026 | W-F07 | Receivable recognition and settlement | 2 | L3-F-039; L3-F-041 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-027 | W-F07 | Receivable dunning / collection governance | 1 | L3-F-040 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-028 | W-F07 | Receivable write-off decision | 1 | L3-F-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-029 | W-F07 | Receivable reopening | 1 | L3-F-043 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-030 | W-F08 | Vendor invoice approval and payable settlement | 2 | L3-F-044; L3-F-045 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-031 | W-F08 | Payable dispute / hold resolution | 1 | L3-F-046 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-032 | W-F08 | Payable change incorporation | 1 | L3-F-047 | INTERACTIVE-WORKFLOW | REQUIRED |
| G5-WF-033 | W-F08 | Payable suspension, reopening and write-off referral | 1 | L3-F-048 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-034 | W-F08 | Payable reconciliation and historical closure | 2 | L3-F-049; L3-F-050 | INTERACTIVE-WORKFLOW | REQUIRED |
| G5-WF-035 | W-F09 | Financial Year / Period establishment and opening | 1 | L3-F-051 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-036 | W-F09 | Financial Period close, reopening/reclose and archival | 3 | L3-F-052; L3-F-054; L3-F-055 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-037 | W-F09 | Financial Year end closure | 1 | L3-F-053 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-038 | W-F10 | Treasury bank-account lifecycle | 1 | L3-F-056 | INTERACTIVE-WORKFLOW | REQUIRED |
| G5-WF-039 | W-F10 | Approved payment population bank execution | 1 | L3-F-057 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-040 | W-F10 | Bank / cash reconciliation | 1 | L3-F-058 | INTERACTIVE-WORKFLOW | REQUIRED |
| G5-WF-041 | W-F10 | Treasury liquidity decision | 1 | L3-F-059 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-042 | W-F10 | Inter-account fund transfer | 1 | L3-F-060 | INTERACTIVE-WORKFLOW | REQUIRED |
| G5-WF-043 | W-F10 | Failed / returned / uncertain bank transaction resolution | 1 | L3-F-061 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G5-WF-044 | W-F11 | Financial Aid allocation, renewal and revocation | 3 | L3-F-062; L3-F-064; L3-F-065 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-045 | W-F11 | Financial Aid disbursement / charge consequence | 1 | L3-F-063 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-046 | W-F11 | Aid funding and utilization reconciliation | 1 | L3-F-066 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-047 | W-F12 | Financial Asset capitalization | 1 | L3-F-067 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-048 | W-F12 | Asset depreciation, revaluation and impairment | 3 | L3-F-068; L3-F-069; L3-F-070 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-049 | W-F12 | Asset disposal and financial write-off accounting | 2 | L3-F-071; L3-F-072 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-050 | W-F13 | Official Financial Report lifecycle and disposition | 2 | L3-F-073; L3-F-078 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-051 | W-F13 | Statutory financial-reporting handoff | 1 | L3-F-074 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G5-WF-052 | W-F13 | Management financial reporting | 1 | L3-F-075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-053 | W-F13 | KPI Definition / Analytical Model version governance | 1 | L3-F-076 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-054 | W-F13 | Financial KPI and dashboard projection | 1 | L3-F-077 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G5-WF-055 | W-F13 | Financial report audit and historical reconstruction | 1 | L3-F-079 | TRACKING-WORKFLOW | NOT-REQUIRED |
| G5-WF-056 | W-F14 | Procurement budget commitment | 1 | L3-F-080 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-057 | W-F14 | Supplier invoice match, vendor liability and Finance closure | 3 | L3-F-081; L3-F-082; L3-F-083 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-058 | W-F00 | Finance Configuration lifecycle | 1 | L3-F-084 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G5-WF-059 | W-F00 | Financial Rule governance lifecycle | 1 | L3-F-085 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |

## 6. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 7 |
| REVIEW-APPROVAL-WORKFLOW | 28 |
| TRACKING-WORKFLOW | 2 |
| EXCEPTION-RECOVERY-WORKFLOW | 9 |
| CROSS-WORKSPACE-HANDOFF | 3 |
| SYSTEM-BACKGROUND-OBSERVABLE | 10 |
| **Total** | **59** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 47 |
| NOT-REQUIRED | 2 |
| PROHIBITED | 10 |
| **Total** | **59** |

## 7. Group Exit Gate

| Gate | Result |
|---|---|
| Section-F canonical scenarios = 85 | PASS |
| Reconciled realization rows = 85 | PASS |
| Unique IDs = 85 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Distinct workflows discovered | PASS — 59 provisional |
| User-addressable workflows discovered | PASS — 46 |
| Material upstream blockers = 0 | PASS |
| F076 canonical realization safely derivable | PASS |

**GROUP 5 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

The former F076 upstream blocker has been resolved at L2, synchronized through L3/L4, and revalidated. Stage 2 may continue to Group 6.
