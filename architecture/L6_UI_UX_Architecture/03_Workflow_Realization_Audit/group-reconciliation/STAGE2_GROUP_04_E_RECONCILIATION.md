# SutraERP L6 Stage 2 — Group 4 Section E Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section E — People & Workforce  
**Scenario population:** 75  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G4-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Completion Result

```text
Assigned canonical scenarios       75
Reconciled scenario rows           75 / 75
Unique scenario IDs                75
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      55
User-addressable workflows         46
Observable non-interactive         9
```

Group 4 is complete for later master cross-group reconciliation. This does not by itself complete Stage 2.

## 2. Semantic Reconciliation Findings

Worker E was mechanically complete but materially under-classified legitimate People & Workforce interaction. **42 scenario rows** required a disposition and/or mock-interaction correction after checking the Locked L3 parents, their authoritative E01–E07 L2 owners, and representative dependent L4 behavior.

Evidence-backed corrections include:

- E01 workforce modification, transfer, deputation, duplicate-profile resolution and bulk transfer expose governed business actions instead of background-only outcomes.
- E01 long-leave availability, employment closure and rejoining remain observable where the source-owned authoritative decision is already complete before E01 applies its consequence; no artificial user action is introduced.
- E02 reporting-relationship establishment is an HR/Registrar-administered workflow; occupancy conflicts, suspension/freeze restoration, administrative correction and rollback require mock interaction where the Locked parents establish governance authority.
- Organisational authority resolution remains background-observable because its primary business behavior is deterministic authority resolution for a consuming action rather than a distinct user mutation.
- E03 Faculty Workforce establishment/activation, capability review, qualification verification, teaching eligibility, availability/workload-capacity planning and closure all include institutional verification, review or approval. Faculty suspension/reactivation is represented as governed exception/recovery.
- E04 onboarding, promotion, transfer, contract-renewal and separation cases remain owner-preserving cross-workspace handoffs, but mock interaction is required because HR intake/review/approval precedes the handoff. Grievance and compliance cases are user/governance workflows rather than background results.
- E05 Leave Policy activation and governed balance adjustment require review/approval interaction. Periodic accrual, approved-leave consumption and year-end carry-forward remain automatic/observable. Encashment eligibility remains a source-owned observable eligibility decision.
- E06 regular payroll, payroll adjustment, retroactive pay, final settlement and payroll configuration all require Payroll Administrator / Payroll Approver interaction. Off-cycle payroll remains an exception/recovery workflow but is mock-interactive because the Locked scenario requires authorized initiation and approval.
- E07 core profile, skill and language maintenance are direct Employee/HR profile interactions. Workforce-triggered profile synchronization and HR-case/separation consequences remain background-observable because their initiating decisions belong upstream. Profile correction, suspension/restoration, duplicate resolution and archived-profile restoration are user-governed recovery workflows.

No L0–L5 owner, actor authority, lifecycle state, event, business rule or dependency was redefined.

## 3. L4 Consistency Check

Representative L4 parents for every material correction pattern were checked:

- `L4-E-018` preserves governed reporting-relationship eligibility/policy validation.
- `L4-E-026` explicitly requires faculty verification and institutional approval before activation.
- `L4-E-036` requires HR grievance intake, assignment, investigation, resolution and closure.
- `L4-E-044` requires Leave Policy validation, approval and attributable decision evidence.
- `L4-E-053` requires Payroll Approver authorization before finalization.
- `L4-E-059` confirms direct canonical Employee Profile maintenance.
- `L4-E-066` confirms profile synchronization acts only on completed authoritative Workforce facts and does not itself decide the source workforce change.

Two upstream uncertainties are retained but are **not material Group-4 blockers**:

1. `L4-E-044` notes that the canonical identifier/name for the Leave Policy activation event is not exposed by the Locked parent. L6 does not invent it.
2. `L4-E-066` preserves the upstream-unconfirmed exact E01→E07 synchronization mapping. Group 4 therefore realizes this path as observable outcome/history rather than inventing a user command or integration contract.

## 4. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 19 |
| REVIEW-APPROVAL-WORKFLOW | 26 |
| TRACKING-WORKFLOW | 0 |
| EXCEPTION-RECOVERY-WORKFLOW | 13 |
| CROSS-WORKSPACE-HANDOFF | 5 |
| SYSTEM-BACKGROUND-OBSERVABLE | 11 |
| NO-DIRECT-END-USER-INTERACTION | 1 |
| **Total** | **75** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 63 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 12 |
| **Total** | **75** |

## 5. Workflow Normalization Method

Scenarios were merged only when they share the same primary workspace, canonical owner, sustained institutional job and business lifecycle.

The normalization deliberately distinguishes:

- upstream-approved/source-owned consequences from real user decisions;
- individual and bulk variants only where they are the same sustained user job;
- case orchestration from the downstream canonical state mutation;
- payroll/leave/profile calculation or synchronization outcomes from the human governance actions surrounding them.

No normalized Group-4 workflow crosses a workspace boundary.

## 6. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G4-WF-001 | W-E01 | Workforce Engagement establishment and activation | 1 | L3-E-001 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-002 | W-E01 | Active Workforce Engagement modification | 1 | L3-E-002 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-003 | W-E01 | Workforce organisational transfer | 2 | L3-E-003; L3-E-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-004 | W-E01 | Workforce suspension and operational restoration | 1 | L3-E-004 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-005 | W-E01 | Workforce deputation lifecycle | 1 | L3-E-005 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-006 | W-E01 | Long-leave operational availability | 1 | L3-E-006 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-007 | W-E01 | Workforce Engagement closure and exit consequence | 1 | L3-E-007 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-008 | W-E01 | Workforce rejoining consequence | 1 | L3-E-008 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-009 | W-E01 | Duplicate Workforce Profile resolution | 1 | L3-E-009 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-010 | W-E02 | Position Assignment lifecycle | 4 | L3-E-011; L3-E-012; L3-E-013; L3-E-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-011 | W-E02 | Supplementary Position occupancy | 1 | L3-E-014 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-012 | W-E02 | Delegated authority and additional charge | 1 | L3-E-015 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-013 | W-E02 | Committee membership appointment | 1 | L3-E-016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-014 | W-E02 | Responsibility Assignment lifecycle | 1 | L3-E-017 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-015 | W-E02 | Reporting Relationship lifecycle | 2 | L3-E-018; L3-E-019 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-016 | W-E02 | Service Jurisdiction lifecycle | 2 | L3-E-020; L3-E-021 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-017 | W-E02 | Organisational authority resolution | 1 | L3-E-022 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-018 | W-E02 | Position occupancy conflict resolution | 1 | L3-E-023 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-019 | W-E02 | Assignment suspension / freeze and restoration | 1 | L3-E-024 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-020 | W-E02 | Position Assignment correction and rollback | 2 | L3-E-071; L3-E-072 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-021 | W-E02 | Bulk Position Assignment operation | 1 | L3-E-073 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-022 | W-E03 | Faculty Workforce lifecycle | 3 | L3-E-026; L3-E-031; L3-E-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-023 | W-E03 | Faculty capability lifecycle | 1 | L3-E-027 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-024 | W-E03 | Faculty qualification verification | 1 | L3-E-028 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-025 | W-E03 | Faculty teaching eligibility | 1 | L3-E-029 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-026 | W-E03 | Faculty availability, workload and capacity planning | 1 | L3-E-030 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-027 | W-E04 | Employee onboarding and accepted owner handoffs | 1 | L3-E-033 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G4-WF-028 | W-E04 | HR leave-case coordination | 1 | L3-E-034 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-029 | W-E04 | HR service and grievance case resolution | 2 | L3-E-035; L3-E-036 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-030 | W-E04 | Employment-change case handoffs | 3 | L3-E-037; L3-E-038; L3-E-040 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G4-WF-031 | W-E04 | Employee certificate request | 1 | L3-E-039 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-032 | W-E04 | Employee separation case and exit handoff | 1 | L3-E-041 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G4-WF-033 | W-E04 | HR compliance case | 1 | L3-E-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-034 | W-E04 | Probation review and confirmation | 1 | L3-E-043 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-035 | W-E05 | Leave Policy lifecycle | 1 | L3-E-044 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-036 | W-E05 | Leave request decision and cancellation | 2 | L3-E-045; L3-E-048 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-037 | W-E05 | Leave entitlement / balance automatic lifecycle | 3 | L3-E-046; L3-E-047; L3-E-050 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-038 | W-E05 | Governed leave balance adjustment | 1 | L3-E-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-039 | W-E05 | Leave encashment eligibility | 1 | L3-E-051 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-040 | W-E05 | Leave exception resolution | 1 | L3-E-052 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-041 | W-E06 | Regular Payroll Run lifecycle | 1 | L3-E-053 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-042 | W-E06 | Payroll adjustment and retroactive pay | 2 | L3-E-054; L3-E-055 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-043 | W-E06 | Final-settlement payroll | 1 | L3-E-056 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-044 | W-E06 | Off-cycle payroll | 1 | L3-E-057 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-045 | W-E06 | Payroll Configuration lifecycle | 1 | L3-E-074 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-046 | W-E07 | Employee Profile establishment and activation | 1 | L3-E-058 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-047 | W-E07 | Employee core-profile maintenance | 1 | L3-E-059 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-048 | W-E07 | Employee qualification and certification verification | 2 | L3-E-060; L3-E-061 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-049 | W-E07 | Employee skills and language profile maintenance | 2 | L3-E-062; L3-E-063 | INTERACTIVE-WORKFLOW | REQUIRED |
| G4-WF-050 | W-E07 | Employee Profile document registration and verification | 1 | L3-E-064 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G4-WF-051 | W-E07 | Employee Profile correction | 1 | L3-E-065 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-052 | W-E07 | Source-owned outcome to Employee Profile synchronization | 2 | L3-E-066; L3-E-067 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-053 | W-E07 | Separation-triggered Employee Profile archival | 1 | L3-E-068 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G4-WF-054 | W-E07 | Employee Profile suspension and restoration | 2 | L3-E-069; L3-E-075 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G4-WF-055 | W-E07 | Duplicate Employee Profile merge | 1 | L3-E-070 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |

## 7. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 12 |
| REVIEW-APPROVAL-WORKFLOW | 21 |
| TRACKING-WORKFLOW | 0 |
| EXCEPTION-RECOVERY-WORKFLOW | 10 |
| CROSS-WORKSPACE-HANDOFF | 3 |
| SYSTEM-BACKGROUND-OBSERVABLE | 9 |
| NO-DIRECT-END-USER-INTERACTION | 0 |
| **Total** | **55** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 46 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 9 |
| **Total** | **55** |

The Group-4 user-addressable population is **46** workflow families. The remaining **9** are deliberately observable/non-interactive because their primary path is a source-owned automatic lifecycle consequence, calculation/resolution, or synchronization rather than a canonical end-user command.

## 8. Canonical Identity Check

The Final Canonical Scenario ID Register confirms Section E contains exactly `L3-E-001 … L3-E-075`.

Group-4 reconciliation preserves all 75 canonical identities and their existing E01–E07 ownership. It does not renumber, split, merge or redefine any upstream scenario.

## 9. Group Exit Gate

| Gate | Result |
|---|---|
| Section-E canonical scenarios = 75 | PASS |
| Reconciled realization rows = 75 | PASS |
| Unique IDs = 75 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Material open issues = 0 | PASS |
| Distinct workflows discovered | PASS — 55 |
| User-addressable workflows discovered | PASS — 46 |
| Mock-required workflows discovered | PASS — 46 |
| Observable non-interactive workflows discovered | PASS — 9 |

**GROUP 4 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

Stage 2 remains in progress because Groups 5–14 and the final master cross-group reconciliation / canonical-register population remain incomplete.
