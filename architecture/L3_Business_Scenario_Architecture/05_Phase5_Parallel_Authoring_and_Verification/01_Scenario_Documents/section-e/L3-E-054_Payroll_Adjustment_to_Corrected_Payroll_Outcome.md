# Payroll Adjustment to Corrected Payroll Outcome

**Scenario ID:** L3-E-054  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payroll  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E06 — Payroll Workflows  
**Referenced By:** None yet

## 1. Scenario Metadata

This scenario preserves the frozen Stage 3 identity and applies the locked L2 business workflow architecture. It does not redefine canonical ownership, lifecycle authority, business rules, event publishers, or downstream dependencies.

## 2. Scenario Purpose

Execute a controlled payroll correction after an identified payroll discrepancy or retrospective change while preserving finalized payroll immutability and complete audit history.

## 3. Business Outcome

A governed payroll adjustment is approved, applied through controlled recalculation/correction, and a corrected payroll outcome is established without directly editing the finalized payroll record.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A payroll discrepancy or correction requirement is identified and enters the Payroll Adjustment workflow.

### 4.2 End Boundary
The approved adjustment has been applied, the corrected payroll outcome is established, and the adjustment outcome is recorded and published.

### 4.3 In Scope
Adjustment initiation; business reason and authority; impact review; approval; controlled correction/recalculation; corrected payroll result; adjustment audit and event publication.

### 4.4 Out of Scope
Creation of originating workforce, attendance, leave or compensation facts; direct editing of finalized payroll; payment execution; accounting posting.

### 4.5 Related Scenarios
E053 — Eligible Payroll Period to Finalized and Published Payroll Outcome; retrospective change and final settlement scenarios are separate canonical scenarios.

## 5. Ownership and Participating Domains

Payroll is the canonical owner of the Payroll Adjustment Aggregate and controlled payroll correction workflow. Upstream domains remain owners of originating facts; Payroll consumes corrected authoritative facts and recalculates the financial outcome. Finance remains owner of payment/accounting downstream of Payroll.

## 6. Business Actors, Roles and Responsibilities

- Payroll Domain — owns adjustment assessment, approval workflow, controlled correction/recalculation and corrected payroll outcome.
- Authorized payroll approver — approves or rejects the adjustment.
- Upstream authoritative domain, where the adjustment originates from a source fact — owns correction of that source fact.
- Finance — consumes the corrected payroll outcome for downstream financial processing.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- A payroll result requiring correction is identified.
- The reason and business basis for the adjustment are available.
- The applicable source facts/rules are available for controlled recalculation.
- Required approval authority is available.
- The existing finalized payroll result remains immutable.

## 8. Trigger

A controlled payroll correction requirement is identified.

## 9. Main Business Flow

| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Create the payroll adjustment case and record the business reason. | Payroll | Adjustment case is established. |
| 2 | Review the affected payroll result and determine the business impact. | Payroll | Adjustment scope and impact are established. |
| 3 | Confirm required authority and obtain approval. | Payroll / authorized approver | Adjustment decision is established. |
| 4 | Apply the controlled correction/recalculation without editing the finalized payroll record. | Payroll | Corrected payroll calculation is established. |
| 5 | Validate the corrected payroll outcome. | Payroll | Corrected result is validated. |
| 6 | Record the corrected payroll result and adjustment history. | Payroll | Corrected payroll outcome is authoritative within the adjustment workflow. |
| 7 | Publish the payroll adjustment outcome for authorized consumers. | Payroll | Downstream consumers can act on the corrected payroll outcome. |

## 10. Alternate Business Flows

- If the adjustment is rejected, the original finalized payroll remains unchanged.
- If the correction depends on a source-domain fact, the authoritative source domain must correct that fact; Payroll then consumes the corrected fact rather than taking ownership of it.
- If recalculation identifies a further business discrepancy, the adjustment remains within the governed correction workflow until the corrected outcome is validated.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Adjustment authority is insufficient | Correction cannot proceed. | Record the business rejection and preserve the finalized payroll. | Payroll |
| Adjustment is rejected | No corrected payroll outcome is established. | Preserve the original finalized payroll record. | Payroll / Approver |
| Source fact is incorrect | Payroll cannot become owner of the source fact. | Source domain corrects its fact; Payroll recalculates using the corrected authoritative input. | Source domain / Payroll |
| Corrected calculation remains invalid | Corrected result cannot be finalized as an adjustment outcome. | Resolve the business discrepancy and repeat the governed correction/validation process. | Payroll |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- Payroll owns controlled correction workflows.
- Finalized payroll cannot be edited.
- Corrections require the Payroll Adjustment workflow.
- Payroll Adjustment Aggregate handles corrections and retroactive changes.
- Previous payroll versions remain auditable.
- Payroll consumes source facts but does not own those facts.
- Approval is required for controlled payroll adjustment.
- Corrected payroll outcomes must retain calculation references and applicable rule/version evidence.

## 13. State and Lifecycle Impact

The Payroll Adjustment Aggregate progresses through its controlled correction lifecycle. The original finalized Payroll Result remains immutable; the adjustment produces a corrected payroll outcome with preserved historical traceability. No rollback or technical undo mechanism is introduced.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

- **PayrollAdjustmentCreated** — correction initiated.
- **PayrollCalculationCompleted** — applicable corrected calculation completed.
- **PayrollValidationCompleted** — corrected outcome validated.
- Where the correction produces a new finalized payroll outcome, **PayrollFinalized** remains the canonical event for the resulting immutable payroll record.

### 14.2 Synchronous Business Contracts / Hand-offs

The adjustment consumes authoritative source facts and publishes the corrected payroll outcome to authorized downstream consumers. Finance remains responsible for payment/accounting consequences; Payroll remains responsible for the corrected payroll result.

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Adjustment reason, authority, approval decision, affected payroll result, calculation references, rule versions, validation outcome and resulting corrected payroll history must remain traceable. Previous versions remain auditable and finalized records are not directly edited.

## 16. Completion Criteria and Postconditions

- Adjustment case and business reason are recorded.
- Required authority and approval are established.
- Controlled correction/recalculation is completed.
- Corrected payroll outcome is validated.
- Original finalized payroll remains immutable.
- Adjustment and resulting payroll events are published as applicable.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
