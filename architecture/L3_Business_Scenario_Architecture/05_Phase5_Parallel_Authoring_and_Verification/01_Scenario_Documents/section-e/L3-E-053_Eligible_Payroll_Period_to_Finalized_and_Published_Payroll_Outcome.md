# Eligible Payroll Period to Finalized and Published Payroll Outcome

**Scenario ID:** L3-E-053  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payroll  
**Participating Domains:** D, E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E06 — Payroll Workflows; authoritative Workforce, Attendance, Leave, Compensation, Compliance Configuration and Enterprise Calendar dependencies  
**Referenced By:** None yet

## 1. Scenario Metadata

This scenario preserves the frozen Stage 3 identity and applies the locked L2 business workflow architecture. It does not redefine canonical ownership, lifecycle authority, business rules, event publishers, or downstream dependencies.

## 2. Scenario Purpose

Execute an eligible payroll period through controlled payroll calculation, validation, authorization, finalization and publication of the authoritative payroll outcome.

## 3. Business Outcome

An eligible payroll run is calculated, validated, approved, finalized as an immutable payroll record and its resulting payroll outcome is published to authorized downstream business consumers.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An eligible payroll period is established for a controlled payroll run.

### 4.2 End Boundary
Payroll is finalized as an immutable financial record and the authoritative payroll outcome has been published through the defined payroll business events.

### 4.3 In Scope
Payroll run establishment; authoritative upstream fact consumption; calculation; validation; approval; finalization; payslip generation where applicable; publication of payroll outcome events.

### 4.4 Out of Scope
Creation/correction of employee, attendance, leave or compensation facts; payment execution; accounting posting; document storage; notification delivery.

### 4.5 Related Scenarios
Payroll adjustment, retroactive pay, final settlement and off-cycle payroll are separate canonical scenarios.

## 5. Ownership and Participating Domains

| Domain | Business responsibility |
|---|---|
| Payroll / L2-E06 | Owns payroll calculation execution, payroll run lifecycle, payroll results, finalization, payslip generation and payroll business events. |
| Workforce / Attendance / Leave / Compensation / Compliance / Calendar | Own their authoritative upstream facts and rules consumed by Payroll. |
| Finance | Owns payment execution and accounting/ledger posting after payroll outcome publication. |
| Document Platform | Owns document storage. |

Payroll calculates from governed facts; it does not become owner of those upstream facts.

## 6. Business Actors, Roles and Responsibilities

- Payroll Domain — owns payroll execution and outcome.
- Payroll approver / authorized business authority — authorizes payroll before finalization.
- Authoritative upstream domains — provide governed workforce, attendance, leave, compensation, compliance and calendar facts.
- Finance — consumes finalized payroll output for payment/accounting activities.
- Employee-facing/document participants — consume payslip/business outcomes without owning payroll results.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- The payroll period is eligible for processing.
- Required authoritative workforce, attendance, leave, compensation, compliance and calendar facts are available.
- Applicable payroll rules and effective versions are available.
- The payroll run can be established without violating the unique-run constraint.
- Required approval authority is available.

## 8. Trigger

An eligible payroll period is opened for controlled payroll execution.

## 9. Main Business Flow

| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Establish the controlled payroll run for the eligible period and payroll type. | Payroll | Payroll execution instance is created. |
| 2 | Determine payroll scope from authoritative workforce and applicable upstream facts. | Payroll / upstream domains | Eligible payroll population and calculation context are established. |
| 3 | Execute payroll calculation using approved compensation, attendance, leave, compliance and other applicable rules. | Payroll | Employee payroll results are generated. |
| 4 | Validate the calculated payroll results and resolve business-level validation findings within the governed workflow. | Payroll | Payroll validation outcome is established. |
| 5 | Submit the validated payroll for authorization. | Payroll | Payroll reaches approval. |
| 6 | Record the authorized approval decision. | Authorized payroll approver | Payroll approval is established. |
| 7 | Finalize the approved payroll. | Payroll | Payroll becomes an immutable financial record. |
| 8 | Generate the applicable payslip outcome. | Payroll | Payslip generation outcome is established where applicable. |
| 9 | Publish the canonical payroll business events for the finalized and generated outcomes. | Payroll | Downstream business consumers receive authoritative payroll outcomes. |

## 10. Alternate Business Flows

- If validation identifies a business error, payroll remains in the governed validation/correction path and is not finalized until validation is satisfactory.
- If approval is rejected, payroll does not become finalized; the business decision is recorded and the run follows its governed correction/review path.
- A payroll run that conflicts with an existing active/eligible run is not duplicated; the existing run is reviewed under the authoritative payroll governance.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Duplicate/invalid payroll run condition | A second controlled run must not be established for the same governed scope. | Review the existing run and correct the business initiation condition. | Payroll |
| Calculation/validation finding | Payroll cannot be finalized as authoritative. | Resolve the business finding and re-run the governed calculation/validation process. | Payroll |
| Approval rejected | Payroll remains unfinalized. | Record the decision and follow the governed review/correction path. | Payroll / Approver |
| Finalization not yet authorized | Payroll remains non-authoritative. | Complete required approval before finalization. | Payroll / Approver |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- A payroll run must be uniquely identified by its governed payroll period and payroll type/scope.
- Payroll executes approved financial processing rules but does not own originating business facts.
- Payroll results must be reproducible from applicable input snapshots, rule versions and calculation references.
- Finalized payroll cannot be edited.
- Corrections to finalized payroll require the Payroll Adjustment workflow.
- Finalization converts approved payroll into immutable financial records.
- Payment execution and accounting posting are owned by Finance, not Payroll.

## 13. State and Lifecycle Impact

The Payroll Run follows its authoritative execution lifecycle from creation through calculation, validation, approval, finalization and closure. The scenario boundary ends at finalized and published payroll outcome; later financial payment/accounting activities remain with downstream Finance.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

- **PayrollRunCreated** — payroll execution initiated.
- **PayrollCalculationCompleted** — calculation completed.
- **PayrollValidationCompleted** — validation completed.
- **PayrollApproved** — authorization completed.
- **PayrollFinalized** — payroll became an immutable financial record.
- **PayslipGenerated** — employee payroll document became available, where applicable.

### 14.2 Synchronous Business Contracts / Hand-offs

Payroll consumes governed business facts from Workforce, Attendance, Leave, Compensation, Compliance Configuration and Enterprise Calendar. Payroll publishes finalized outcomes to Finance, Employee Portal, Notification, Analytics, Compliance and Document Platform according to the locked dependency model. Finance owns payment/accounting after Payroll finalization.

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Payroll preserves actor identity, timestamp, business reason and state changes for important actions. Calculation references, rule versions, approval history and finalized payroll snapshots support reproducibility and auditability.

## 16. Completion Criteria and Postconditions

- Eligible payroll run is created.
- Required upstream facts and applicable rules are consumed.
- Calculation and validation complete successfully.
- Required approval is recorded.
- Payroll is finalized and immutable.
- Applicable payslip outcome is generated.
- Canonical payroll events are published.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
