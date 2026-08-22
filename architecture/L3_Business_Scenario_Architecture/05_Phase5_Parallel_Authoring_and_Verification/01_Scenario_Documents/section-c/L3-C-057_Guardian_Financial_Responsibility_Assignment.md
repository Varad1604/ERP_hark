# Guardian Financial Responsibility Assignment

**Scenario ID:** L3-C-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Guardian  
**Participating Domains:** A, C, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C06 — Guardian Workflows; L2-A02 — Person & Identity; L2-C01 — Student Lifecycle; additional participating-domain L2 sources where applicable  
**Referenced By:** Related canonical Section C scenarios as identified below

## 1. Scenario Metadata

The frozen Section C identity is preserved exactly. Guardian Workflows / Guardian Relationship remains the canonical owner of the Guardian business outcome. Participating domains retain ownership of their own facts and operational outcomes.

## 2. Scenario Purpose

Establish or change which Guardian is institutionally designated as financially responsible for a Student while preserving the boundary between Guardian responsibility assignment and Finance-owned financial obligations and transactions.

## 3. Business Outcome

The Guardian Relationship Aggregate contains an authoritative financial-responsibility assignment, the assignment is auditable, and Finance can consume the business fact without Guardian assuming ownership of financial transactions.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authorized request proposes or changes financial responsibility for an existing Guardian Relationship.

### 4.2 End Boundary
The applicable Guardian financial-responsibility assignment is recorded and the canonical Guardian business fact is published for Finance consumption.

### 4.3 In Scope
- Identify the authoritative Student, Person and Guardian Relationship.
- Validate that the Guardian Relationship is eligible for financial responsibility.
- Validate applicable institutional policy and authority.
- Record or change the financial-responsibility assignment.
- Preserve prior assignment history.
- Publish the canonical Guardian financial-responsibility business fact.
- Allow Finance to use the assignment within its own financial workflows.

### 4.4 Out of Scope
- Creating invoices or financial obligations.
- Fee assessment.
- Payment processing.
- Refunds, concessions or settlements.
- Changing Person identity.
- Changing Student lifecycle.
- Financial authorization credentials.
- Direct modification of Finance-owned state.

### 4.5 Related Scenarios
C052 — Guardian Relationship Modification; C059 — Guardian Relationship Suspension; C071 — Scholarship Eligibility to Financial Aid Processing.

## 5. Ownership and Participating Domains

The **Guardian Relationship Aggregate** is the authoritative owner of Guardian relationship state and the business decision described by this scenario.

Participating domains provide or consume authoritative information within their own boundaries. Participation does not transfer ownership to Finance, Transport, Person & Identity, Student Lifecycle, or any other domain.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns the designation of a Guardian as financially responsible.
- Finance owns financial obligations, invoices, payments and settlements.
- Person & Identity owns the underlying Person.
- Student Lifecycle owns Student existence/lifecycle.
- Finance consumes the Guardian assignment as an authoritative business fact.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- Student exists.
- Guardian Relationship exists and is in an eligible state.
- Guardian Person identity is authoritative.
- Required institutional authority is available.
- Applicable financial-responsibility policy is available.

## 8. Trigger

An authorized financial-responsibility assignment or change is requested.

## 9. Main Business Flow

1. Identify the Student and Guardian Relationship.
2. Validate the Person and Student references.
3. Validate the Guardian Relationship's eligibility.
4. Evaluate applicable institutional financial-responsibility policy.
5. Record the financial-responsibility assignment or change.
6. Preserve prior assignment history.
7. Publish the canonical Guardian financial-responsibility business fact.
8. Finance consumes the resulting assignment within its own canonical financial workflow.

The flow stops when the singular institutional outcome is achieved; downstream operational execution remains outside the scenario unless explicitly owned by Guardian.

## 10. Alternate Business Flows

- A proposed responsible Guardian is not eligible: do not apply the assignment.
- A conflicting assignment is detected: preserve the existing authoritative position until the conflict is resolved through governed business procedures.
- A Guardian remains financially responsible while Finance independently manages the resulting financial obligation.
- Changing responsibility does not rewrite historical financial transactions.

## 11. Failure, Interruption and Business Recovery

An invalid assignment is rejected without altering the existing authoritative relationship. Historical assignment evidence remains intact. Financial disputes or payment recovery remain Finance-owned business matters and are not resolved inside Guardian Workflows.

Recovery remains business-level. No technical retries, queues, rollback, infrastructure, API or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- BR-020: financial responsibility is assigned to an eligible Guardian Relationship.
- Financial responsibility assignment does not transfer ownership of Finance transactions.
- Guardian relationship history remains immutable.
- Guardian business decisions are auditable.
- Cross-domain ownership remains bounded: Guardian assigns responsibility; Finance manages financial consequences.

Only source-supported Guardian rules are applied. No local replacement rule is invented.

## 13. State and Lifecycle Impact

Financial-responsibility changes are Guardian Relationship modifications. The relationship remains within the Guardian lifecycle; no separate Financial Responsibility lifecycle is invented.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

`GuardianFinancialResponsibilityUpdated` — Guardian Relationship Aggregate. Finance is a consumer of the business fact and remains the owner of financial obligations and transactions.

Canonical Guardian events remain published by the Guardian Relationship Aggregate. Events are business facts, not technical commands.

## 15. Business Authorization, Approval and Audit Obligations

Financial-responsibility assignment requires appropriate institutional authority and complete auditability. The exact approving authority is institution-configurable and is not invented here.

## 16. Completion Criteria and Postconditions

- Eligible Guardian Relationship identified.
- Policy/authority validated.
- Financial responsibility recorded.
- Historical assignment preserved.
- Canonical Guardian event published.
- Finance can consume the authoritative assignment without ownership transfer.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C057-01:** The locked C06 source establishes Guardian ownership of financial-responsibility assignment and Finance ownership of financial transactions, but does not define a universal responsible-guardian monetary obligation taxonomy. None is invented.

**OI-C057-02:** Exact institutional approval authority is configurable.

**Classification:** PASS WITH ISSUES — ownership and dependency boundaries are clear; institution-specific financial-responsibility policy remains upstream.

## Production Verification Record

- [x] Frozen ID/title/owner/domains/type/priority preserved.
- [x] Guardian Relationship ownership preserved.
- [x] Cross-domain ownership boundaries preserved.
- [x] Canonical Guardian lifecycle/event vocabulary used.
- [x] Finance/Transport responsibilities are not absorbed into Guardian.
- [x] No implementation leakage.
- [x] All 17 mandatory sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status intentionally remains Draft under Phase 5 governance.
