# Student Charge to Installment and Due Schedule

**Scenario ID:** L3-F-005  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Charge & Fee Management  
**Participating Domains:** C — Student Lifecycle; F — Finance  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; authoritative Student Lifecycle context where required for the charge schedule  
**Referenced By:** Payment Management, Accounts Receivable, Reporting and other downstream Finance consumers that consume payable milestones  

## 2. Scenario Purpose

Organize a canonical Student Charge into one or more governed payable milestones when institutional charging policy requires installment-based settlement.

The scenario establishes the installment and due schedule as part of the Student Charge financial architecture without creating a second financial obligation and without initiating payment collection.

## 3. Business Outcome

The Student Charge has an authoritative installment structure and corresponding due schedule that defines when its payable components become contractually due.

The resulting schedule remains subordinate to the Student Charge, historically traceable, and available for downstream settlement and receivable processing.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins after the Student Charge's financial value has been established and the applicable charging policy requires or permits installment organization.

### 4.2 End Boundary

The scenario ends when the canonical installment structure and due schedule have been established against the Student Charge and the charge is ready for publication/consumption with those payable milestones.

### 4.3 In Scope

- Determine whether installment organization applies.
- Establish the installment structure.
- Associate each installment with exactly one Student Charge.
- Establish the due schedule for the installment structure.
- Preserve due-date and installment traceability.
- Make the schedule available for publication and downstream financial processing.

### 4.4 Out of Scope

- Payment collection.
- Payment allocation.
- Settlement execution.
- Refund processing.
- Accounting posting.
- Redefinition of the Student Charge amount.
- Technical scheduling, timers, queues, APIs, databases or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership |
|---|---|---|
| C — Student Lifecycle | Supplies authoritative student context where needed for the financial schedule. | Student lifecycle remains C-owned. |
| F — Student Charge & Fee Management | Owns Student Charge, Installment Architecture and Due Date Architecture. | All canonical financial schedule concepts remain F-owned. |
| Payment Management | Consumes payable milestones for settlement. | Payment remains F02-owned. |
| Accounts Receivable | Consumes due information for receivable monitoring/aging. | Receivable remains F07-owned. |

L2-F01 explicitly states that Installments do not own financial obligations and that every Installment belongs to a Student Charge. fileciteturn138file3turn138file7

## 6. Business Actors, Roles and Responsibilities

L2-F01 does not define an actor catalogue. No schedule-specific role is invented.

The business policy governing installment applicability remains owned by the authoritative Finance policy architecture. Any student-context responsibility remains with Student Lifecycle.

## 7. Preconditions and Entry Criteria

- A canonical Student Charge exists.
- Charge Calculation has established the charge value.
- Applicable institutional charging policy has been evaluated.
- Installment organization is permitted or required.
- The relevant financial and institutional context is available.
- Due-date rules can be established from authoritative configuration/policy.

## 8. Trigger

A valid Student Charge reaches the point at which its payable amount must be organized into an installment and due schedule.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the Student Charge for which installment scheduling is required. | F | The canonical financial obligation is identified. |
| 2 | Determine the applicable installment model from authoritative charging configuration/policy. | F | The required payment organization is established. |
| 3 | Determine the number and financial composition of installments permitted by the applicable model. | F | Scheduled financial components are defined without replacing the Student Charge. |
| 4 | Establish each Installment as a scheduled financial component of the Student Charge. | F | Every installment is subordinate to exactly one Student Charge. |
| 5 | Establish the Due Schedule for the installment structure. | F | Each payable milestone has an authoritative due timing. |
| 6 | Verify that the installment structure remains consistent with the Student Charge and its financial amount. | F | The scheduled structure is financially coherent. |
| 7 | Preserve historical traceability of the Student Charge, installments and due schedule. | F | The schedule can be reconstructed later. |
| 8 | Make the scheduled Student Charge ready for publication and downstream settlement/receivable consumption. | F | Downstream domains can use the payable milestones without owning them. |

L2-F01 positions Installment Architecture after Charge Calculation and before Charge Publication. fileciteturn138file3turn138file5

## 10. Alternate Business Flows

### A1 — Single Immediate Due

The applicable policy does not require multiple installments. The Student Charge receives the applicable single payable milestone/due schedule.

### A2 — Multiple Installments

The charge is divided into multiple scheduled components, each remaining subordinate to the same Student Charge.

### A3 — Installment Model Not Applicable

The charge remains a Student Charge without an installment structure beyond the applicable due schedule.

### A4 — Due Schedule Determined by Institutional Calendar

The due schedule follows an authoritative institutional calendar/configuration while preserving the Student Charge as the canonical financial obligation.

### A5 — Schedule Requires Governed Revision

If a published schedule requires later correction, the change follows the applicable governed Student Charge revision/adjustment architecture rather than silently rewriting historical published financial history.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Resolution |
|---|---|
| Installment model is unavailable or not applicable | Do not invent an installment structure; use the applicable governed due arrangement. |
| Due timing cannot be established | Do not publish an incomplete schedule; resolve the governing financial/calendar context first. |
| Installment totals do not reconcile with the Student Charge | Do not treat the schedule as valid until the governing financial inconsistency is resolved. |
| Student Charge is not eligible for installment organization | Preserve the Student Charge and use the applicable non-installment due arrangement. |
| Published schedule requires correction | Use the governed revision/adjustment path and preserve historical traceability. |

Technical scheduling failures are outside L3.

## 12. Business Rules and Decision Constraints

Authoritative L2-F01 constraints include:

- Installments are configuration-driven.
- Installments never own financial obligations.
- Installments remain auditable throughout their lifecycle.
- Every Installment belongs to exactly one parent Student Charge.
- Due schedules are configuration-driven.
- Due schedules do not initiate payment collection.
- Due schedules remain independent of payment execution.
- Published due schedules preserve historical financial integrity.
- Every Installment possesses one Due Schedule.
- Outstanding Balance remains derived rather than independently owned.

fileciteturn125file0turn138file3turn138file7

No installment or due-date rule is redefined locally.

## 13. State and Lifecycle Impact

The Student Charge remains under the L2-F01 canonical lifecycle:

`Business Obligation → Charge Identification → Charge Generation → Charge Validation → Published → Financially Active → Fully Settled → Archived`

The installment and due schedule organize the financial timeline of the Student Charge; they do not replace or create a separate Student Charge lifecycle. fileciteturn125file0

The scenario does not introduce new Student Charge states or lifecycle transitions.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

L2-F01 identifies canonical Student Charge financial events including:

- `ChargeAmountCalculated`
- `InstallmentStructureGenerated`
- `DueScheduleEstablished`
- `OutstandingBalanceUpdated`
- `ChargePublicationCompleted`

The Student Charge Domain remains the publisher of these Student Charge events. fileciteturn126file0

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C — Student Lifecycle | F — Student Charge | Student context required for schedule interpretation | Provides authoritative student business context. |
| F — Student Charge | Payment Management | Published charge with installment/due schedule | Enables downstream settlement against payable milestones. |
| F — Student Charge | Accounts Receivable | Due schedule and outstanding financial position | Supports receivable monitoring and aging. |

Events communicate completed facts; these hand-offs communicate business context or consume authoritative financial information.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

No installment-specific actor or approval authority is defined in L2-F01. Institutional installment policy remains configuration-driven and governed by the applicable upstream policy architecture.

### 15.2 Separation of Duties

No scenario-specific separation-of-duty rule is defined in the accessible L2-F01 material.

### 15.3 Business Audit and Evidence Obligations

The installment structure and due schedule must remain historically traceable to the Student Charge.

L2-F01 explicitly requires installment auditability and historical traceability of due schedules. fileciteturn138file3turn125file0

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The canonical Student Charge is identified.
- Applicable installment policy/model has been established.
- Every created Installment belongs to exactly one Student Charge.
- Each Installment has its required Due Schedule.
- The scheduled structure is financially consistent with the Student Charge.
- The resulting schedule is ready for publication/downstream consumption.
- No payment collection has been initiated by this scenario.

### 16.2 Postconditions

- The Student Charge has an authoritative payable schedule.
- Installments remain subordinate to the Student Charge.
- Due dates define expected settlement milestones.
- Historical schedule information remains traceable.
- Payment and Accounts Receivable can consume the schedule without acquiring ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material scenario-specific upstream issue was identified for the installment/due-schedule boundary in the accessible locked L2-F01 architecture.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
