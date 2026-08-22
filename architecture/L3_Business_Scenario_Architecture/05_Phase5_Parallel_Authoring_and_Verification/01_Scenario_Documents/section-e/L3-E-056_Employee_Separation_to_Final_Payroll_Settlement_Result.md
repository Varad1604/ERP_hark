# Employee Separation to Final Payroll Settlement Result

**Scenario ID:** L3-E-056  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payroll  
**Participating Domains:** E, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E06 — Payroll Workflows; locked Phase 5 scenario allocation and frozen Section E canonical scenario semantics  
**Referenced By:** None yet  

## 1. Scenario Metadata

This scenario applies the locked Payroll business architecture and preserves canonical ownership, lifecycle, rules, events, and downstream boundaries established by L2-E06.

## 2. Scenario Purpose

Process the payroll consequences of an employee separation through the dedicated final-settlement payroll lifecycle and establish the employee's final payroll settlement outcome.

## 3. Business Outcome

The employee separation is reflected in a governed final-settlement payroll result covering applicable pending salary, deductions, recoveries and settlements, and the resulting financial outcome is finalized for Finance.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authoritative employee separation outcome is available and requires final payroll settlement.

### 4.2 End Boundary
The final-settlement payroll result is calculated, approved and finalized as the authoritative payroll outcome for the separated employee.

### 4.3 In Scope
Establish final-settlement payroll scope; apply applicable salary, deduction, recovery and settlement rules; calculate and validate final settlement; obtain approval; finalize the final-settlement payroll outcome; provide the finalized outcome to Finance.

### 4.4 Out of Scope
Creating or approving the employee separation itself; modifying employee master facts; defining compensation or leave policy; payment execution; accounting posting.

### 4.5 Related Scenarios
E053 — Eligible Payroll Period to Finalized and Published Payroll Outcome provides the general payroll lifecycle; this scenario specifically covers the final-settlement payroll run type.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Payroll / E | Owns final-settlement payroll execution, calculation, result, approval and finalization. |
| Finance / F | Owns payment execution and accounting after receiving the finalized payroll financial outcome. |
| Workforce / upstream source | Owns the employee separation fact; it is consumed by Payroll and does not transfer ownership. |

Participation does not transfer ownership of upstream facts or downstream responsibilities.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Payroll Administrator | Initiates and coordinates final-settlement payroll processing. | Business processing participant. |
| Payroll Approver | Reviews and authorizes the final-settlement payroll before finalization. | Approval authority supported by L2-E06. |
| Finance | Receives the finalized financial outcome for downstream settlement/accounting. | Downstream financial owner. |

## 7. Preconditions and Entry Criteria

- An authoritative employee separation outcome exists.
- The employee's effective employment dates are available to Payroll.
- Applicable final-settlement payroll rules and configuration are active.
- Required salary, deduction, recovery and leave-settlement facts are available to Payroll.

## 8. Trigger

An authoritative employee separation outcome requires final payroll settlement.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the final-settlement payroll run for the separated employee using the dedicated final-settlement payroll type. | Payroll | Final-settlement payroll execution is established. |
| 2 | Determine the applicable pending salary, deductions, recoveries and settlements using authoritative business facts and applicable rules. | Payroll | Final-settlement calculation context is established. |
| 3 | Calculate the employee's final payroll settlement result. | Payroll | Final settlement payroll result is generated. |
| 4 | Validate the final-settlement result and resolve any business-level validation finding before approval. | Payroll | Final-settlement result is validated. |
| 5 | Obtain required payroll approval. | Payroll / Payroll Approver | Final settlement is authorized for finalization. |
| 6 | Finalize the final-settlement payroll result as an immutable payroll record. | Payroll | Final payroll settlement outcome is authoritative. |
| 7 | Provide the finalized financial outcome to Finance for its separate settlement/accounting responsibility. | Payroll / Finance | Finance receives the payable final-settlement outcome. |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required separation/effective-date information is incomplete | Final settlement cannot be correctly determined. | Resolve the authoritative workforce information before settlement calculation proceeds. | Payroll / upstream source |
| Required salary, deduction, recovery or settlement input is unavailable | Final settlement cannot be validated. | Resolve the authoritative source information and recalculate through the governed payroll workflow. | Payroll / relevant source domain |
| Validation finding remains unresolved | Final settlement cannot be approved/finalized. | Resolve the business finding and complete governed validation. | Payroll |
| Approval is rejected | Final settlement remains unfinalized. | Follow the governed payroll review/correction path. | Payroll / Payroll Approver |
| Finance receives a downstream settlement issue | Payroll outcome remains a payroll record; Finance resolves its payment/accounting responsibility. | Finance applies its governed downstream recovery without editing the Payroll Result. | Finance |

Technical retries, queues, rollback, infrastructure failures and implementation-level recovery are outside L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Employees leaving during a payroll period are processed according to effective employment dates, settlement rules and applicable compensation rules. | Payroll applies; source domains own source facts | Determines final-settlement scope and calculation. | L2-E06 BR-PAY-008 |
| Final Settlement Payroll is a supported payroll run type. | Payroll | The scenario uses the dedicated final-settlement lifecycle. | L2-E06 §2.1 / §8.12 / WF-E06-010 |
| Payroll approval is mandatory before finalization. | Payroll | Settlement cannot become authoritative without approval. | L2-E06 BR-PAY-017 |
| Finalized payroll is immutable and corrections use adjustment workflow. | Payroll | Final settlement cannot be directly edited after finalization. | L2-E06 BR-PAY-019 / BR-PAY-020 |
| Finance owns payment execution and accounting. | Finance | Finance receives but does not modify the Payroll Result. | L2-E06 §4.2 / ownership matrix |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Payroll Run Aggregate | Created | Final-settlement run is established. | Calculating → Calculated → Validating → PendingApproval → Approved → Finalized, then downstream Posted/Closed as applicable. | Payroll | L2-E06 §7.1 / §13.1 |
| Payroll Result Aggregate | Calculated | Final settlement result is calculated and validated. | Validated → Approved → Finalized. | Payroll | L2-E06 §7.2 / §14 |

Only authoritative L2 lifecycle states and transitions are used; no local state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PayrollRunCreated | Payroll | Payroll consumers | Records creation of the final-settlement payroll run. | L2-E06 §11.1 |
| PayrollCalculationCompleted | Payroll | Payroll consumers | Records completion of final-settlement calculation. | L2-E06 §11.3 |
| PayrollValidationCompleted | Payroll | Payroll consumers | Records completion of validation. | L2-E06 §11.4 |
| PayrollApproved | Payroll | Payroll consumers | Records authorization before finalization. | L2-E06 §11.5 |
| PayrollFinalized | Payroll | Finance Domain | Records the final settlement as an immutable payroll record. | L2-E06 §11.6 / §13 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Upstream employee/workforce authority | Payroll | Authoritative employee separation outcome and effective employment facts | Establishes the business basis for final settlement. | L2-E06 §3.2 / §10.3 |
| Payroll | Finance | Finalized payroll financial outcome | Provides the financial outcome that Finance settles/accounts. | L2-E06 §4.2 / §8.12 |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority
| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate final-settlement payroll | Payroll Administrator | Payroll run creation requires an authorized payroll user and valid payroll period/configuration. | Payroll |
| Approve final settlement | Payroll Approver | Approval is required before finalization. | Payroll |

### 15.2 Separation of Duties, Overrides and Exceptions
| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Approval before finalization | Payroll Approver | Final settlement cannot be finalized without authorized approval. |
| Payroll/Finance boundary | Payroll and Finance | Payroll calculates and finalizes the financial outcome; Finance owns payment/accounting. |

### 15.3 Business Audit and Evidence Obligations
| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Final-settlement reproducibility | Applicable employee/workforce, attendance, leave, compensation and rule/calculation references required by Payroll. | Payroll with source domains retaining their authoritative facts |
| Approval traceability | Payroll approval decision and approval history. | Payroll |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Final-settlement payroll run is established using the dedicated final-settlement payroll type.
- Applicable salary, deductions, recoveries and settlements are calculated.
- Final-settlement result passes validation and required approval.
- Final settlement is finalized as an immutable Payroll Result.
- Finalized financial outcome is handed to Finance.

### 16.2 Postconditions
- The separated employee has an authoritative final payroll settlement outcome.
- The finalized Payroll Result is immutable.
- Finance has the finalized financial outcome for its separate settlement/accounting responsibilities.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No open issues.

### 17.2 Assumptions
No assumptions.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring Classification:** PASS — source-grounded Draft.
