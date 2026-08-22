# Business Obligation to Published Student Charge

**Scenario ID:** L3-F-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Charge & Fee Management  
**Participating Domains:** C — Student Lifecycle; D — Academic & Curriculum; F — Finance; G — Institutional Services; I — Engagement, Communication & Experience  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F00 — Finance Domain Model & Foundation; L2-F01 — Student Charge & Fee Management; authoritative source-domain business event architectures  
**Referenced By:** Payment Management, Refund Management, Financial Adjustments, Accounting, Budget Management, Financial Reporting, Analytics and Notification consumers as applicable  

## 2. Scenario Purpose

Establish a canonical Student Charge from an authoritative business obligation and publish the resulting validated financial obligation for downstream Finance consumption.

The scenario separates the originating business capability from the financial obligation. The participating C, D, G and I domains remain owners of their business facts and events; Student Charge & Fee Management becomes the canonical owner of the resulting Student Charge.

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

## 5. Ownership and Participating Domains

| Domain | Role in Scenario | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Originates student lifecycle business obligations/events. | Student lifecycle concepts remain C-owned. |
| D — Academic & Curriculum | Originates academic business obligations/events. | Academic concepts remain D-owned. |
| F — Student Charge & Fee Management | Interprets the obligation and owns Student Charge. | Student Charge, Charge Source, Charge Type, Fee Structure, Charge Policy, Charge Calculation, Installment and Outstanding Balance remain F-owned. |
| G — Institutional Services | May originate service-related student obligations/events. | Service concepts remain G-owned. |
| I — Engagement, Communication & Experience | Participates where an authoritative business obligation/event is within its canonical responsibility. | I-owned business concepts remain I-owned. |

The originating domain publishes the business event; Finance creates the financial obligation. No cross-domain participation transfers ownership. This follows the Finance foundation principle that Finance validates, records, accounts, controls and reports financial consequences while operational domains own the originating business facts. fileciteturn128file0

## 6. Business Actors, Roles and Responsibilities

The L2-F01 document does not define an authoritative actor catalogue and explicitly records “Actors” as a governance gap. Therefore no additional actor or authority is invented here. The scenario operates on authoritative business events and governed Finance processing rather than introducing scenario-specific roles. fileciteturn113file0

Where a source domain has an actor under its own architecture, that actor remains owned by the source domain and is not redefined as a Finance actor.

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

L2-F01 explicitly defines Charge Generation as the sequence from business event through source resolution, classification, policy evaluation, eligibility, Fee Structure resolution, calculation, validation and Student Charge creation. fileciteturn115file0

## 8. Trigger

An authoritative business event representing a valid business obligation is received by the Student Charge Domain.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | The originating domain publishes the authoritative business obligation event. | C / D / G / I as applicable | A business obligation capable of generating a financial consequence exists. |
| 2 | Student Charge & Fee Management resolves the originating Charge Source. | F | The financial obligation is attributed to one canonical source. |
| 3 | The charge is classified using the canonical Finance classification. | F | The obligation has a valid financial classification. |
| 4 | Applicable Charge Policy is evaluated for applicability and eligibility. | F | The generation decision is established: applicable, not applicable, deferred, blocked or requires review. |
| 5 | Where applicable, the Fee Structure is resolved for the institutional and academic context. | F | A reusable charging definition is selected. |
| 6 | Charge Calculation determines the monetary value of the obligation. | F | A calculated financial amount exists. |
| 7 | The Student Charge passes structural, context, policy and financial validation. | F | The charge is complete and publication-ready. |
| 8 | The canonical Student Charge is established and becomes the authoritative financial obligation. | F | A canonical Student Charge exists. |
| 9 | Publication readiness is confirmed. | F | The charge is eligible for enterprise publication. |
| 10 | The Student Charge is Published and the applicable Finance domain events are emitted. | F | Downstream Finance domains can consume the published financial obligation. |

The flow ends at Published. Payment, accounting and notification are downstream responsibilities. L2-F01 explicitly states that publication does not perform payment collection, accounting posting or notification. fileciteturn125file0

## 10. Alternate Business Flows

### A1 — Charge Not Applicable

Policy evaluation determines that the obligation does not create a charge. No Student Charge is established.

### A2 — Charge Deferred

The business obligation is valid but a required condition is not yet satisfied. The financial obligation is not published until the governed dependency is resolved.

### A3 — Charge Blocked

Policy, context or validation prevents creation. The financial obligation remains uncreated until the governing condition is resolved.

### A4 — Charge Requires Review

The obligation cannot be deterministically completed from available business context and remains outside Published state pending the required business resolution.

### A5 — Duplicate Obligation Detection

A repeated evaluation refers to an obligation for which a canonical Student Charge already exists. A second Student Charge is not created.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Resolution |
|---|---|
| Originating business context is incomplete | Do not create or publish a Student Charge until the source business context is sufficient. |
| Charge Source cannot be resolved | Keep the obligation outside canonical published financial state and resolve the source-domain context. |
| Policy evaluation fails to produce an applicable outcome | Use the governed deferred, blocked or review outcome rather than inventing applicability. |
| Fee Structure cannot be resolved | Do not calculate or publish the charge. |
| Calculation cannot produce a valid value | Do not establish a publication-ready obligation. |
| Validation fails | Correct the business/configuration condition before publication. |
| Duplicate obligation is detected | Preserve the existing canonical Student Charge and do not create another. |

These are business-level failure paths. Technical retries, queues, transport failures and database recovery are outside L3.

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

fileciteturn114file0turn115file0turn125file0

## 13. State and Lifecycle Impact

The authoritative L2-F01 lifecycle is:

`Business Obligation → Charge Identification → Charge Generation → Charge Validation → Published → Financially Active → Fully Settled → Archived`

For this scenario, the relevant progression ends at **Published**. fileciteturn125file0

The Student Charge state architecture distinguishes current state from lifecycle. A newly established charge may progress through Draft, Pending Validation, Validated and Published states before becoming operationally Active. fileciteturn125file0

No new state or lifecycle transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

The originating operational domain publishes the authoritative business obligation event according to its own canonical event architecture.

Student Charge & Fee Management publishes its canonical lifecycle/financial events, including the L2-defined concepts:

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

L2-F01 identifies these as canonical Student Charge domain event categories. fileciteturn126file0

The authoritative publisher of each source-domain event remains its originating domain. Finance does not republish another domain's business fact as its own event.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C / D / G / I as applicable | F — Student Charge | Authoritative business obligation event/context | Provides the business basis for financial obligation creation. |
| F — Student Charge | Payment Management | Published Student Charge | Makes the obligation available for settlement. |
| F — Student Charge | Accounting | Published financial obligation with accounting context | Provides accounting-ready financial information without creating the journal entry in F01. |
| F — Student Charge | Financial reporting/analytics | Published financial obligation facts | Supports downstream financial consumption. |

L2-F01 explicitly defines this Publish–Own–Consume model and prohibits external domains from directly modifying Student Charge. fileciteturn126file0turn127file0

## 15. Business Authorization, Approval and Audit Obligations

L2-F01 does not provide an authoritative actor/approval catalogue for Student Charge creation. Accordingly, this scenario does not invent a Finance-specific role or approval authority.

The financial obligation must nevertheless satisfy the authoritative policy and validation architecture before publication. The Student Charge domain participates in auditing of charge generation, classification, policy evaluation, Fee Structure resolution, calculation, validation, publication, lifecycle progression and state changes. fileciteturn126file0

The originating domain remains responsible for the authorization/evidence of the originating business event under its own architecture.

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
- ownership of the originating business concept remains with its source domain;
- Student Charge ownership remains with Student Charge & Fee Management.

The published obligation does not mean it has been paid or accounted. Settlement and accounting remain downstream.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

L2-F01's governance crosswalk explicitly records **Actors** and **Locked Decisions** as gaps in the L2 document. This L3 therefore does not invent an actor/approval model. fileciteturn113file0

### 17.2 Assumptions

No assumptions override the locked architecture.

### 17.3 Deferred Questions

Confirm any scenario-specific business actor/approval mapping through the applicable upstream governance sources before Architecture Approval.
