# Business Obligation to Published Student Charge

**Scenario ID:** L3-F-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Charge & Fee Management  
**Participating Domains:** C — Student Lifecycle; D — Academic Delivery; F — Finance; G — Campus Operations; I — Examination  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Varad  
**Last Updated:** 2026-09-23  
**Depends On:** L2-F00 — Finance Domain Model & Foundation; L2-F01 — Student Charge & Fee Management; `L2-F01_Student_Charge_Publication_Authority_Amendment_2026-09-23.md`; authoritative source-domain business event architectures  
**Referenced By:** Payment Management, Refund Management, Financial Adjustments, Accounting, Budget Management, Financial Reporting, Analytics and Notification consumers as applicable

## 1. Scenario Metadata

This scenario preserves the frozen Phase 4 identity: canonical title, Primary Owner, participating domain set `C, D, F, G, I`, Scenario Type `Critical`, and Priority `P0`.

Authoritative sources for this scenario are the Phase 4 Section F Canonical Register, `L2-F00 — Finance Domain Model & Foundation`, `L2-F01 — Student Charge & Fee Management`, `DEPENDENCY_MAP.md`, and the applicable source-domain L2 architectures for the originating business obligation.

## 2. Scenario Purpose

Establish a canonical Student Charge from an authoritative business obligation and publish the resulting validated financial obligation for downstream Finance consumption.

The scenario separates the originating business capability from the financial obligation. Participating domains C, D, G and I remain owners of their business facts and events; Student Charge & Fee Management becomes the canonical owner of the resulting Student Charge.

## 3. Business Outcome

A valid business obligation is interpreted, classified, evaluated against applicable charging policy, valued through the applicable Fee Structure and Charge Calculation, validated, established as a canonical Student Charge, and published as an enterprise-recognized financial obligation.

The resulting published Student Charge is traceable to exactly one Charge Source and is ready for downstream settlement and financial processing without transferring ownership of the originating business fact.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authoritative business event from a participating operational domain indicates that a business obligation may require creation of a student financial obligation.

### 4.2 End Boundary

The scenario ends when the Student Charge has passed publication readiness validation and becomes Published, with the canonical financial event available to downstream Finance domains.

### 4.3 In Scope

- Receipt of authoritative business context.
- Resolution of Charge Source.
- Charge Classification.
- Charge Policy evaluation.
- Eligibility evaluation.
- Fee Structure resolution.
- Charge Calculation.
- Financial validation.
- Student Charge establishment.
- Publication readiness.
- Charge Publication.
- Publication business events and traceability.

### 4.4 Out of Scope

- Definition of the originating operational business event.
- Payment collection.
- Payment allocation or settlement.
- Refund processing.
- Accounting journal creation.
- Banking operations.
- Notification delivery.
- Technical event transport, APIs, queues, databases, retries or infrastructure.

### 4.5 Related Scenarios

Downstream settlement, adjustment, refund, accounting and reporting scenarios consume the published Student Charge without redefining Student Charge ownership.

## 5. Ownership and Participating Domains

| Domain | Role in Scenario | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | May originate student-lifecycle business obligations/events. | Student Lifecycle concepts remain C-owned. |
| D — Academic Delivery | May originate academic-delivery business obligations/events. | Academic Delivery concepts remain D-owned. |
| F — Finance / Student Charge & Fee Management | Interprets the obligation and owns Student Charge. | Student Charge, Charge Source, Charge Type, Fee Structure, Charge Policy, Charge Calculation, Installment and Outstanding Balance remain F-owned. |
| G — Campus Operations | May originate campus-service obligations/events such as hostel, transport or library-related obligations where supported by the owning L2 architecture. | Campus Operations concepts remain G-owned. |
| I — Examination | May originate examination-related obligations/events where supported by the owning L2 architecture. | Examination concepts remain I-owned. |

The originating domain publishes the authoritative business fact; Finance creates and owns the resulting financial obligation. Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

Standard event-driven charge generation is domain-authoritative and policy-driven under the Locked L2-F01 publication-authority amendment. No universal human approver is required for standard F001 processing; manual/exceptional charge authority remains separately governed when policy requires it.

Where a source domain has an actor under its own locked architecture, that actor remains owned by the source domain and is not redefined as a Finance actor.

## 7. Preconditions and Entry Criteria

- An authoritative originating business event exists.
- The business context required for financial interpretation is available.
- The Charge Source can be resolved.
- Charge Classification is available.
- Applicable Charge Policy can be evaluated.
- An applicable Fee Structure can be resolved.
- Charge Calculation can determine the monetary value.
- The Student Charge can satisfy structural, context, policy, financial and publication validation.
- The same obligation has not already produced a canonical Student Charge.

These criteria apply the locked L2-F01 charge-generation sequence from source resolution and classification through policy evaluation, Fee Structure resolution, calculation, validation and Student Charge establishment.

## 8. Trigger

An authoritative business event representing a valid business obligation is received by Student Charge & Fee Management.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | The originating domain publishes the authoritative business obligation event. | C / D / G / I as applicable | A business obligation capable of generating a financial consequence exists. |
| 2 | Student Charge & Fee Management resolves the originating Charge Source. | F | The financial obligation is attributed to one canonical source. |
| 3 | The charge is classified using the canonical Finance classification. | F | The obligation has a valid financial classification. |
| 4 | Applicable Charge Policy is evaluated for applicability and eligibility. | F | The generation decision is established as applicable, not applicable, deferred, blocked or requiring review. |
| 5 | Where applicable, the Fee Structure is resolved for the institutional and academic context. | F | A reusable charging definition is selected. |
| 6 | Charge Calculation determines the monetary value of the obligation. | F | A calculated financial amount exists. |
| 7 | The Student Charge passes structural, context, policy and financial validation. | F | The charge is complete and publication-ready. |
| 8 | The canonical Student Charge is established and becomes the authoritative financial obligation. | F | A canonical Student Charge exists. |
| 9 | Publication readiness is confirmed. | F | The charge is eligible for enterprise publication. |
| 10 | The Student Charge becomes Published and the applicable canonical Finance events are published. | F | Downstream Finance domains can consume the published financial obligation. |

The flow ends at Published. Payment, accounting and notification remain downstream responsibilities.

## 10. Alternate Business Flows

### A1 — Charge Not Applicable

Policy evaluation determines that the obligation does not create a charge. No Student Charge is established.

### A2 — Charge Deferred

The business obligation is valid but a required condition is not yet satisfied. The financial obligation is not published until the governed dependency is resolved.

### A3 — Charge Requires Review

The obligation cannot be deterministically completed from available business context and remains outside Published state pending the required business resolution.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Resolution |
|---|---|
| Originating business context is incomplete | Do not create or publish a Student Charge until the source business context is sufficient. |
| Charge Source cannot be resolved | Keep the obligation outside canonical published financial state and resolve the source-domain context. |
| Charge is blocked by policy, context or validation | Keep the obligation outside Published state until the blocking business condition is resolved. |
| Fee Structure cannot be resolved | Do not calculate or publish the charge. |
| Calculation cannot produce a valid value | Do not establish a publication-ready obligation. |
| Validation fails | Correct the governing business or configuration condition before publication. |
| Duplicate obligation is detected | Preserve the existing canonical Student Charge and do not create another. |

These are business-level failure and recovery paths. Technical retries, queues, transport failures and database recovery are outside L3.

## 12. Business Rules and Decision Constraints

The scenario applies these authoritative L2-F01 constraints:

- Every payable obligation shall be represented as a Student Charge.
- Every Student Charge has exactly one Charge Source.
- Every Student Charge belongs to exactly one Charge Type.
- Charge Policies govern applicability.
- Fee Structures define reusable charging definitions.
- Charge Calculation establishes monetary value after policy evaluation and Fee Structure resolution.
- Outstanding Balance is derived and not independently owned.
- External domains shall not modify Student Charges directly.
- Only architecturally complete Student Charges may be published.
- Publication is idempotent.
- Publication does not perform payment collection or accounting posting.
- Published Student Charges remain historically traceable.

Authoritative source: `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F01_Student_Charge_and_Fee_Management.md`.

## 13. State and Lifecycle Impact

The authoritative L2-F01 lifecycle is:

`Business Obligation → Charge Identification → Charge Generation → Charge Validation → Published → Financially Active → Fully Settled → Archived`

For this scenario, the relevant progression ends at **Published**.

The Student Charge state architecture distinguishes current state from lifecycle. A newly established charge may progress through Draft, Pending Validation, Validated and Published states before becoming operationally Active.

No new state or lifecycle transition is introduced.

Authoritative source: `L2-F01 — Student Charge & Fee Management`, Charge State Architecture and lifecycle sections.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

The originating operational domain publishes the authoritative business obligation event according to its own canonical event architecture.

Student Charge & Fee Management publishes the canonical L2-F01 Student Charge events applicable to the progression, including:

- `ChargeIdentificationInitiated`
- `ChargeGenerationCompleted`
- `ChargeValidationCompleted`
- `ChargePublicationCompleted`
- `ChargeActivationCompleted`
- `ChargeClosureCompleted`
- `ChargeArchivalCompleted`
- `ChargeAmountCalculated`
- `InstallmentStructureGenerated`
- `DueScheduleEstablished`
- `OutstandingBalanceUpdated`

The authoritative publisher of each source-domain event remains its originating domain. Finance does not republish another domain's business fact as its own event.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C / D / G / I as applicable | F — Student Charge & Fee Management | Authoritative business obligation event/context | Provides the business basis for financial obligation creation. |
| F — Student Charge & Fee Management | Payment Management | Published Student Charge | Makes the obligation available for settlement. |
| F — Student Charge & Fee Management | Accounting | Published financial obligation with accounting context | Provides accounting-ready financial information without creating the journal entry in F01. |
| F — Student Charge & Fee Management | Financial reporting / analytics | Published financial obligation facts | Supports downstream financial consumption. |

This follows the L2-F01 publish-own-consume model and its prohibition on external domains directly modifying Student Charge.

## 15. Business Authorization, Approval and Audit Obligations

The Locked L2-F01 publication-authority amendment establishes that standard event-driven Student Charge creation/publication proceeds under L2-F01 policy/validation authority without mandatory human approval. Source domains retain ownership of originating business facts.

The financial obligation must satisfy the authoritative policy and validation architecture before publication. Student Charge & Fee Management preserves business traceability across charge generation, classification, policy evaluation, Fee Structure resolution, calculation, validation, publication, lifecycle progression and state changes as required by L2-F01.

The originating domain remains responsible for the authorization and evidence of the originating business event under its own architecture.

## 16. Completion Criteria and Postconditions

The scenario is complete when:

- the originating business obligation is understood within its canonical business context;
- the Charge Source is established;
- applicable classification, policy and Fee Structure requirements are satisfied;
- Charge Calculation is complete;
- the Student Charge passes required validation;
- the Student Charge is canonical and traceable;
- the Student Charge is Published;
- downstream Finance consumers can consume the published obligation;
- ownership of the originating business concept remains with its source domain; and
- Student Charge ownership remains with Student Charge & Fee Management.

The published obligation does not mean it has been paid or accounted. Settlement and accounting remain downstream.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The prior actor/approval gap affecting standard F001 processing is resolved by `L2-F01_Student_Charge_Publication_Authority_Amendment_2026-09-23.md`. Manual/exceptional charges remain subject to separately governed authority when applicable.

### 17.2 Assumptions

None. No assumption overrides the locked architecture.

### 17.3 Deferred Questions

Confirm any scenario-specific business actor or approval mapping only if an authoritative upstream governance source defines it before Architecture Approval.

**Authoring Classification:** PASS — upstream standard-charge authority resolved by Locked L2-F01 amendment.


## 2026-09-23 Upstream Authority Reconciliation

L3-F-001 consumes the Locked L2-F01 Student Charge publication-authority amendment. Standard event-driven charge generation/publication requires no universal human approval; L2-F01 retains Student Charge ownership and `ChargePublicationCompleted` publication authority. No source-domain ownership is transferred.
