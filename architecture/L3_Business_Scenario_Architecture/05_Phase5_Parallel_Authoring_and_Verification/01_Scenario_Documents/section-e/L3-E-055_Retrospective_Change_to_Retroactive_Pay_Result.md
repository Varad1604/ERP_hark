# Retrospective Change to Retroactive Pay Result

**Scenario ID:** L3-E-055  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payroll  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E06 — Payroll Workflows; locked Phase 5 scenario allocation and frozen Section E canonical scenario semantics  
**Referenced By:** None yet  

## 1. Scenario Metadata

This scenario applies the locked Payroll business architecture and preserves canonical ownership, lifecycle, rules, events, and downstream boundaries established by L2-E06.

## 2. Scenario Purpose

Process an approved compensation change that takes effect retrospectively and establish the resulting retroactive pay outcome for affected prior payroll periods through the controlled Payroll retro pay workflow.

## 3. Business Outcome

The retrospective change has been evaluated across its impacted prior payroll periods, the resulting difference has been calculated and established as an arrear/retroactive pay result, and that result is included in the governed payroll outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authoritative compensation change with an effective date that precedes already processed payroll is available to Payroll for financial impact processing.

### 4.2 End Boundary
The retroactive difference has been calculated, represented through the governed arrear/retro-pay process, and included in the applicable payroll outcome.

### 4.3 In Scope
Identify impacted historical payroll periods; calculate the financial difference; create the governed arrear transaction; include the retroactive pay result in payroll; preserve applicable calculation and audit history.

### 4.4 Out of Scope
Defining or changing compensation policy; editing historical finalized payroll directly; creating the originating compensation change; payment execution; accounting posting.

### 4.5 Related Scenarios
E054 — Payroll Adjustment to Corrected Payroll Outcome is the broader controlled correction boundary; this scenario specifically addresses the retrospective-change-to-retroactive-pay outcome.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Payroll / E | Owns payroll calculation, retro pay processing, arrear transaction, payroll result and resulting financial outcome. |
| Upstream compensation authority | Owns the originating compensation change and its business policy; Payroll consumes the authoritative change and applies its financial impact. |
| Finance | Owns downstream payment and accounting after the payroll outcome is established. |

Participation does not transfer ownership of upstream facts or downstream responsibilities.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Payroll Administrator | Initiates/coordinates the governed payroll processing for the retrospective change. | Business processing participant. |
| Payroll Approver | Provides required payroll approval before the affected payroll outcome is finalized. | Approval authority supported by L2-E06. |

## 7. Preconditions and Entry Criteria

- An authoritative compensation change exists.
- Its effective date establishes an impact on one or more previously processed payroll periods.
- The applicable payroll rules and calculation context are available.
- The affected payroll result(s) are identifiable.

## 8. Trigger

An authoritative compensation change is identified as affecting previous payroll periods.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the previous payroll periods affected by the retrospective compensation change. | Payroll | Impacted periods are established. |
| 2 | Determine the difference between the previously processed payroll outcome and the outcome applicable under the retrospective change. | Payroll | Retroactive financial difference is established. |
| 3 | Create the governed arrear/retro-pay transaction representing the difference. | Payroll | Retroactive pay amount is represented within Payroll. |
| 4 | Include the arrear/retro-pay result in the applicable payroll processing outcome. | Payroll | Retroactive pay becomes part of the governed payroll result. |
| 5 | Apply normal payroll validation and required approval controls to the resulting payroll outcome. | Payroll / Payroll Approver | Corrected payroll outcome is authorized for finalization. |
| 6 | Finalize the applicable payroll outcome and preserve the historical calculation context. | Payroll | Retroactive pay result is established as an auditable payroll outcome. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Historical change affects multiple periods | The retrospective effective date spans more than one already processed payroll period. | Evaluate each impacted period and calculate the applicable difference before establishing the aggregate retro-pay outcome. | All impacted periods are represented in the retroactive result. | Payroll |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Affected period cannot be established | Retroactive impact cannot be correctly calculated. | Resolve the business-period/source information before proceeding. | Payroll |
| Required compensation change information is not available | Payroll cannot establish the authoritative retrospective basis. | Obtain the authoritative compensation outcome; Payroll does not invent or redefine it. | Payroll / upstream compensation authority |
| Retroactive calculation is not validated | The resulting payroll outcome cannot be finalized. | Resolve the business calculation finding and complete governed validation before approval/finalization. | Payroll |
| Required approval is not granted | The resulting payroll outcome remains unfinalized. | Follow the governed payroll approval/review path. | Payroll / Payroll Approver |

Technical retries, queues, rollback, infrastructure failures and implementation-level recovery are outside L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every payroll calculation must execute against a specific rule version. | Payroll / originating rule owners | Historical and current calculation contexts must remain identifiable. | L2-E06 BR-PAY-004 |
| Every payroll result must maintain references to the inputs used during calculation. | Payroll | Retroactive calculation remains reproducible. | L2-E06 BR-PAY-005 |
| Retroactive changes must identify impacted periods, difference amount and correction reason. | Payroll | These determine the retro-pay calculation boundary. | L2-E06 BR-PAY-022 |
| Finalized payroll cannot be directly edited; corrections use controlled adjustment workflows. | Payroll | Historical payroll remains immutable while retroactive pay is established through governed processing. | L2-E06 BR-PAY-019 / BR-PAY-020 |
| Payroll applies approved compensation structures; compensation policy remains upstream-owned. | Compensation / Payroll | Payroll calculates financial impact without taking ownership of the source policy. | L2-E06 BR-PAY-013 / ownership boundary |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Payroll Adjustment Aggregate | Requested / Validated / PendingApproval | Retroactive change is assessed and approved as a controlled payroll correction. | Calculated / Applied / Closed according to the governed adjustment lifecycle. | Payroll | L2-E06 §7.3 / Adjustment Lifecycle |
| Payroll Result Aggregate | Calculated / Validated / Approved | Retroactive pay is incorporated into the resulting payroll outcome. | Finalized when the applicable payroll outcome is finalized. | Payroll | L2-E06 §7.2 / §14 |

Only authoritative L2 lifecycle states and transitions are used; no local state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PayrollAdjustmentCreated | Payroll | Payroll Audit | Records initiation of the controlled correction workflow. | L2-E06 §11.8 / §13 |
| PayrollCalculationCompleted | Payroll | Payroll consumers as applicable | Records completion of the retroactive calculation. | L2-E06 §11.3 |
| PayrollValidationCompleted | Payroll | Payroll consumers as applicable | Records completion of validation before approval. | L2-E06 §11.4 |
| PayrollApproved | Payroll | Payroll consumers as applicable | Records authorized approval of the resulting payroll outcome. | L2-E06 §11.5 |
| PayrollFinalized | Payroll | Finance Domain | Records that the resulting payroll became an immutable financial record. | L2-E06 §11.6 / §13 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Upstream compensation authority | Payroll | Authoritative compensation change | Provides the approved retrospective change that Payroll must financially apply. | L2-E06 §3.5 / §8.11 |
| Payroll | Finance | Finalized payroll financial outcome | Provides the resulting payable financial outcome for downstream Finance processing. | L2-E06 §4.2 / §8.8 |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority
| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate retro-pay processing | Payroll Administrator | Governed payroll processing authority. | Payroll |
| Approve resulting payroll | Payroll Approver | Approval is required before finalization. | Payroll |

### 15.2 Separation of Duties, Overrides and Exceptions
| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Approval before finalization | Payroll Approver | Payroll cannot be finalized without authorized approval. |

### 15.3 Business Audit and Evidence Obligations
| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Reproducibility of retroactive calculation | Applicable input snapshots, rule/version references and calculation context already required by Payroll. | Payroll |
| Approval of resulting payroll | Approval decision and approval history required by Payroll. | Payroll |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Impacted prior payroll periods are identified.
- Retroactive difference is calculated.
- Governed arrear/retro-pay transaction is established.
- Resulting payroll outcome passes required validation and approval.
- Retroactive pay result is included in the governed payroll outcome.

### 16.2 Postconditions
- The retroactive pay result is represented in the applicable payroll outcome.
- The original finalized historical payroll record remains unchanged.
- The resulting payroll outcome retains the applicable calculation and approval history.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No open issues.

### 17.2 Assumptions
No assumptions.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring Classification:** PASS — source-grounded Draft.
