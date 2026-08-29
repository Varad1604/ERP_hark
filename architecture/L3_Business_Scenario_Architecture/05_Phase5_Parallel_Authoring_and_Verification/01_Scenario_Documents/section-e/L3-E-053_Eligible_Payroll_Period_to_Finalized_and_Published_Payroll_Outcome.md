# Eligible Payroll Period to Finalized and Published Payroll Outcome

**Scenario ID:** L3-E-053  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payroll  
**Participating Domains:** D, E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** L2-E06 — Payroll Workflows; authoritative Workforce, Attendance, Leave, Compensation, Compliance Configuration and Enterprise Calendar dependencies  
**Referenced By:** Downstream Finance, employee-facing, compliance, analytics, notification and document consumers of finalized payroll outcomes

## 1. Scenario Metadata

This scenario preserves the frozen Phase 4 identity and applies the locked L2-E06 Payroll workflow architecture. It does not redefine canonical ownership, lifecycle authority, business rules, event publishers, or downstream dependencies.

## 2. Scenario Purpose

Execute an eligible payroll period through controlled payroll calculation, validation, authorization, finalization and publication of the authoritative payroll outcome.

## 3. Business Outcome

An eligible payroll run is calculated, validated, approved by the authorized Payroll Approver, finalized as an immutable payroll record and its resulting payroll outcome is published to authorized downstream business consumers.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An eligible payroll period is established for a controlled payroll run.

### 4.2 End Boundary
Payroll reaches the canonical **Finalized** state as an immutable financial record and the authoritative payroll outcome has been published through the defined Payroll business events.

### 4.3 In Scope
Payroll run establishment; authoritative upstream fact consumption; calculation; validation; approval; finalization; payslip generation where applicable; publication of payroll outcome events.

### 4.4 Out of Scope
Creation/correction of employee, attendance, leave or compensation facts; payment execution; accounting posting; document storage; notification delivery.

### 4.5 Related Scenarios
Payroll adjustment, retroactive pay, final settlement and off-cycle payroll are separate canonical scenarios.

## 5. Ownership and Participating Domains

| Domain | Business responsibility |
|---|---|
| Payroll / L2-E06 | Owns payroll calculation execution, Payroll Run lifecycle, payroll results, finalization, payslip generation and payroll business events. |
| Workforce / Attendance / Leave / Compensation / Compliance / Calendar | Own their authoritative upstream facts and rules consumed by Payroll. |
| Finance | Owns payment execution and accounting/ledger posting after Payroll finalization. |
| Document Platform | Owns document storage. |

Payroll calculates from governed facts; it does not become owner of those upstream facts.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Payroll Administrator / Payroll Operator | Performs governed Payroll Run preparation, calculation and validation activities as permitted by L2-E06. | Does not provide final approval unless also separately authorized by policy. |
| Payroll Approver | Provides the authoritative payroll approval before finalization. | Owns the approval decision for the governed Payroll Run. |
| Finance Reviewer (optional, where configured) | Performs Finance review where the institutional configuration requires it. | Review does not transfer Payroll ownership to Finance. |
| Payroll Domain | Owns Payroll Run execution, result generation, finalization and Payroll events. | Cannot re-own upstream workforce or Finance transactions. |
| Finance | Consumes finalized Payroll output for payment and accounting activities. | Owns payment execution and ledger/accounting, not Payroll calculation. |

No new scenario-specific actor is introduced beyond roles supported by L2-E06.

## 7. Preconditions and Entry Criteria

- The payroll period is eligible for processing.
- Required authoritative workforce, attendance, leave, compensation, compliance and calendar facts are available.
- Applicable payroll rules and effective versions are available.
- The Payroll Run can be established without violating the unique-run constraint.
- An authorized Payroll Approver is available.

## 8. Trigger

An eligible payroll period is opened for controlled Payroll execution.

## 9. Main Business Flow

| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Establish the controlled Payroll Run for the eligible period and payroll type. | Payroll | Payroll Run enters **Created**. |
| 2 | Determine payroll scope from authoritative workforce and applicable upstream facts. | Payroll / upstream domains | Eligible payroll population and calculation context are established. |
| 3 | Execute payroll calculation using approved compensation, attendance, leave, compliance and other applicable rules. | Payroll | Payroll Run progresses through **Calculating** to **Calculated**. |
| 4 | Validate the calculated payroll results. | Payroll | Payroll Run progresses through **Validating** and, when validation passes, reaches **PendingApproval**. |
| 5 | Submit the validated payroll to the authorized Payroll Approver. | Payroll | Payroll remains **PendingApproval** until a governed decision is made. |
| 6 | Record the authorized approval decision. | Payroll Approver | Payroll Run reaches **Approved** when approval is granted. |
| 7 | Finalize the approved Payroll Run. | Payroll | Payroll Run reaches **Finalized** and becomes an immutable payroll record. |
| 8 | Generate the applicable payslip outcome. | Payroll | Payslip generation outcome is established where applicable. |
| 9 | Publish the canonical Payroll business events for the finalized and generated outcomes. | Payroll | Downstream business consumers receive authoritative payroll outcomes. |

## 10. Alternate Business Flows

### A1 — Optional Finance Review

Where institutional configuration requires a Finance Reviewer before approval or finalization, the governed review is performed without transferring Payroll ownership to Finance.

### A2 — Payslip Not Applicable

Where a payroll outcome does not require an employee-facing payslip, the Payroll Run may still complete through the canonical approval and finalization path without generating a payslip.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Duplicate or invalid Payroll Run condition | A second controlled run must not be established for the same governed scope. | Review the existing run and correct the business initiation condition. | Payroll |
| Calculation or validation finding | Payroll cannot progress to authoritative approval/finalization. | Resolve the governed business finding and repeat the applicable calculation/validation steps. | Payroll |
| Validation does not pass | Payroll Run does not progress from validation to **PendingApproval**. | Correct the governed input/rule condition and revalidate. | Payroll |
| Approval rejected | Payroll Run remains unfinalized and follows the L2-E06 governed rejection/review path. | Record the rejection and perform the governed correction/review activity before any later resubmission. | Payroll / Payroll Approver |
| Finalization not authorized | Payroll cannot transition from **Approved** to **Finalized**. | Complete required approval/authorization before finalization. | Payroll / Payroll Approver |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- A Payroll Run must be uniquely identified by its governed payroll period and payroll type/scope.
- Payroll executes approved financial processing rules but does not own originating business facts.
- Payroll results must be reproducible from applicable input snapshots, rule versions and calculation references.
- Finalized payroll cannot be edited.
- Corrections to finalized payroll require the Payroll Adjustment workflow.
- Finalization converts approved payroll into immutable financial records.
- Payment execution and accounting posting are owned by Finance, not Payroll.
- Payroll approval must be performed by the authorized Payroll Approver defined by L2-E06.

## 13. State and Lifecycle Impact

The authoritative Payroll Run progression relevant to this scenario is:

`Created → Calculating → Calculated → Validating → PendingApproval → Approved → Finalized`

L2-E06 also governs rejection and later downstream positions such as Posted and Closed. This scenario's canonical completion boundary is **Finalized and published**; Finance-owned payment/accounting activity remains outside the scenario.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

- **PayrollRunCreated** — Payroll Run established.
- **PayrollCalculationCompleted** — calculation completed.
- **PayrollValidationCompleted** — validation completed.
- **PayrollApproved** — authorization completed.
- **PayrollFinalized** — payroll became an immutable financial record.
- **PayslipGenerated** — employee payroll document outcome became available, where applicable.

All listed Payroll events remain published by Payroll according to L2-E06.

### 14.2 Synchronous Business Contracts / Hand-offs

Payroll consumes governed business facts from Workforce, Attendance, Leave, Compensation, Compliance Configuration and Enterprise Calendar. Payroll publishes finalized outcomes to Finance, employee-facing channels, Notification, Analytics, Compliance and Document Platform according to the locked dependency model. Finance owns payment/accounting after Payroll finalization.

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Payroll preserves actor identity, timestamp, business reason and state changes for important actions. Calculation references, rule versions, approval history and finalized payroll snapshots support reproducibility and auditability.

The **Payroll Approver** is the authoritative approval role for Payroll approval under L2-E06. An optional Finance Reviewer may participate only where configured and does not become the Payroll owner or approver by default.

## 16. Completion Criteria and Postconditions

- Eligible Payroll Run is created.
- Required upstream facts and applicable rules are consumed.
- Calculation and validation complete successfully.
- Authorized Payroll Approver approval is recorded.
- Payroll Run reaches **Finalized** and is immutable.
- Applicable payslip outcome is generated where required.
- Canonical Payroll events are published.
- Payment execution and accounting remain downstream Finance responsibilities.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — directly aligned to locked L2-E06; remains Draft pending formal Phase 5 review.
