# Exceptional Requirement to Off-Cycle Payroll Completion

**Scenario ID:** L3-E-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payroll  
**Participating Domains:** E, F  
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

Provide controlled payroll execution outside the standard payroll cycle when an exceptional business requirement requires an emergency payment, special incentive or approved correction.

## 3. Business Outcome

The exceptional payroll requirement is authorized, processed through an off-cycle payroll run, finalized as an authoritative payroll outcome and handed to Finance for its downstream financial responsibility.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A governed exceptional payroll requirement is identified outside the normal payroll cycle.

### 4.2 End Boundary
The authorized off-cycle payroll run is completed and its payroll outcome is finalized and provided to Finance.

### 4.3 In Scope
Off-cycle requirement assessment; authorization; off-cycle payroll run establishment; applicable calculation and validation; approval; finalization; Finance hand-off.

### 4.4 Out of Scope
Normal-cycle payroll processing; defining the underlying business policy that created the requirement; direct modification of finalized payroll; payment execution and accounting.

### 4.5 Related Scenarios
E053 — Eligible Payroll Period to Finalized and Published Payroll Outcome is the general payroll execution scenario; E054 covers controlled post-payroll adjustment; this scenario is specifically the exceptional off-cycle execution path.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Payroll / E | Owns off-cycle payroll run, calculation, validation, approval, finalization and payroll outcome. |
| Finance / F | Owns payment execution and accounting after receiving the finalized payroll outcome. |
| Upstream source domains | Own their originating business facts where the exceptional requirement depends on them; Payroll consumes those facts without taking ownership. |

Participation does not transfer ownership of upstream facts or downstream responsibilities.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Payroll Administrator | Initiates and coordinates the exceptional off-cycle payroll requirement. | Business processing participant. |
| Payroll Approver | Authorizes the off-cycle payroll before finalization. | Approval authority. |
| Finance | Receives the finalized financial outcome for downstream financial processing. | Downstream financial owner. |

## 7. Preconditions and Entry Criteria

- An exceptional payroll requirement exists outside the normal payroll cycle.
- The business basis for the requirement is established.
- Applicable payroll configuration/rules are active.
- Required authority for off-cycle execution is available.
- Required source facts for calculation are available.

## 8. Trigger

An exceptional business requirement requires payroll execution outside the normal cycle.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the exceptional requirement and its business basis. | Payroll | Off-cycle requirement is defined for controlled processing. |
| 2 | Confirm that the requirement is authorized for exceptional payroll execution. | Payroll / Payroll Approver | Off-cycle processing is authorized. |
| 3 | Establish the off-cycle payroll run using the supported Off-Cycle Payroll run type. | Payroll | Controlled off-cycle payroll run is created. |
| 4 | Collect the applicable governed facts and calculate the exceptional payroll result. | Payroll | Off-cycle payroll result is generated. |
| 5 | Validate the result and resolve any business-level validation finding. | Payroll | Off-cycle result is validated. |
| 6 | Obtain required payroll approval before finalization. | Payroll / Payroll Approver | Off-cycle payroll is approved. |
| 7 | Finalize the off-cycle payroll result as an immutable payroll record. | Payroll | Off-cycle payroll outcome is authoritative. |
| 8 | Provide the finalized financial outcome to Finance for payment/accounting processing. | Payroll / Finance | Finance receives the off-cycle financial outcome. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Exceptional requirement is an approved correction | The requirement arises from an already identified payroll correction. | Process the correction through the governed off-cycle payroll path rather than directly editing finalized payroll. | Corrected exceptional payroll outcome is established. | Payroll |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Exceptional requirement lacks required authorization | Off-cycle processing cannot proceed. | Obtain the required business authorization or terminate the request according to the governed decision. | Payroll / Payroll Approver |
| Required source facts are unavailable | Off-cycle result cannot be correctly calculated. | Resolve the authoritative source facts before calculation. | Payroll / source domain |
| Validation finding remains unresolved | Off-cycle payroll cannot be finalized. | Resolve the business finding and re-enter the governed validation path. | Payroll |
| Approval is rejected | Off-cycle payroll remains unfinalized. | Follow the governed payroll review/correction path. | Payroll / Payroll Approver |
| Finance receives a downstream processing issue | Payroll result remains authoritative; downstream financial handling is Finance-owned. | Finance applies its governed settlement/accounting resolution. | Finance |

Technical retries, queues, rollback, infrastructure failures and implementation-level recovery are outside L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Off-Cycle Payroll is a supported payroll run type. | Payroll | Establishes the scenario's payroll execution mode. | L2-E06 §2.1 / §8.13 / WF-E06-011 |
| Off-cycle payroll requires authorization and maintains an audit trail. | Payroll | Exceptional execution cannot bypass business approval. | L2-E06 §8.13 |
| Payroll cannot be finalized without authorized approval. | Payroll | Approval precedes finalization. | L2-E06 BR-PAY-017 |
| Finalized payroll is immutable; corrections use adjustment workflow. | Payroll | Off-cycle processing must not directly edit finalized records. | L2-E06 BR-PAY-019 / BR-PAY-020 |
| Finance owns payment execution and accounting. | Finance | Finance receives the finalized outcome but does not own payroll calculation. | L2-E06 §4.2 / ownership matrix |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Payroll Run Aggregate | Created | Establishes an Off-Cycle Payroll run. | Calculating → Calculated → Validating → PendingApproval → Approved → Finalized → Posted → Closed. | Payroll | L2-E06 §7.1 / §13.1 |
| Payroll Result Aggregate | Calculated | Represents the exceptional payroll financial result. | Validated → Approved → Finalized. | Payroll | L2-E06 §7.2 / §14 |

Only authoritative L2 lifecycle states and transitions are used; no local state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PayrollRunCreated | Payroll | Payroll consumers | Records creation of the off-cycle payroll run. | L2-E06 §11.1 |
| PayrollCalculationCompleted | Payroll | Payroll consumers | Records completion of off-cycle calculation. | L2-E06 §11.3 |
| PayrollValidationCompleted | Payroll | Payroll consumers | Records completion of validation. | L2-E06 §11.4 |
| PayrollApproved | Payroll | Payroll consumers | Records authorization before finalization. | L2-E06 §11.5 |
| PayrollFinalized | Payroll | Finance Domain | Records the off-cycle payroll as an immutable financial record. | L2-E06 §11.6 / §13 |
| PayrollClosed | Payroll | Analytics Platform | Records completion of the payroll lifecycle. | L2-E06 §11.9 / §13 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Authoritative source domain(s) | Payroll | Governed business facts required for exceptional calculation | Provides facts; Payroll owns financial calculation. | L2-E06 §3 |
| Payroll | Finance | Finalized payroll financial outcome | Provides the amount/outcome Finance must settle and account for. | L2-E06 §4.2 / §8.13 |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority
| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate off-cycle payroll | Payroll Administrator | Exceptional processing must be authorized and uses the governed off-cycle workflow. | Payroll |
| Approve off-cycle payroll | Payroll Approver | Approval is required before finalization. | Payroll |

### 15.2 Separation of Duties, Overrides and Exceptions
| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Approval before finalization | Payroll Approver | Off-cycle payroll cannot be finalized without authorized approval. |
| Payroll/Finance separation | Payroll and Finance | Payroll calculates/finalizes the outcome; Finance executes payment and accounting. |

### 15.3 Business Audit and Evidence Obligations
| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Exceptional processing traceability | Business basis, authorization, payroll processing and approval history as required by the governed Payroll audit model. | Payroll |
| Finalization traceability | Finalization and resulting payroll history. | Payroll |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Exceptional requirement is established and authorized.
- Off-cycle payroll run is created using the supported Off-Cycle Payroll type.
- Applicable calculation and validation complete.
- Required approval is recorded.
- Off-cycle payroll is finalized as an immutable outcome.
- Finalized financial outcome is handed to Finance.

### 16.2 Postconditions
- An authoritative off-cycle payroll outcome exists.
- The finalized payroll result is immutable.
- Finance has the finalized financial outcome for downstream payment/accounting responsibility.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No open issues.

### 17.2 Assumptions
No assumptions.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring Classification:** PASS — source-grounded Draft.
