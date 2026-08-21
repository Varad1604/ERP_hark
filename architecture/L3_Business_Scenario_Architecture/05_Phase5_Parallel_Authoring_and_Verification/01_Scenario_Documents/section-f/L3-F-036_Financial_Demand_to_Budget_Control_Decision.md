# Financial Demand to Budget Control Decision

## 1. Scenario Metadata

**Scenario ID:** L3-F-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** L2-F00; L2-F06; L2-F14; L2-J03; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario provides the Finance-owned budget-control decision for an authoritative financial demand without taking ownership of the originating procurement decision.

## 3. Business Outcome

A governed Budget-valid or Budget-not-valid decision is recorded and communicated for the presented financial demand.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A financial demand requiring Budget control is presented to Finance.

### 4.2 End Boundary

Finance records and publishes the governed Budget-control decision.

### 4.3 In Scope

- validation of Budget, allocation, Cost Center, fund and period
- availability assessment under governed policy
- decision evidence and communication

### 4.4 Out of Scope

- procurement approval or cancellation
- commitment creation (`L3-F-080`)
- payment or ledger posting

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Finance decision or accounting outcome and its evidence |
| Section J — Enterprise Resource Management | Provides authoritative operational facts and retains its aggregate lifecycles |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Scenario Responsibility |
|---|---|
| Budget Planning Officer | Prepares the governed Budget proposal, allocation, revision, virement or closure request applicable to this scenario. |
| Budget Control Officer | Performs Budget-control and eligibility assessment under approved Finance policy. |
| Finance Committee | Authorizes governed Budget approval, allocation, structural change or closure where applicable. |
| Cost Center Administrator | Prepares Finance-owned Cost Center lifecycle actions where applicable. |
| Financial Controller | Independently authorizes Cost Center activation, deactivation, reactivation or closure where applicable. |

Roles and authority are established by approved `ACR-F-2026-08-21-001` and do not define technical access control.

## 7. Preconditions and Entry Criteria

- The demand has an authoritative source reference.
- Applicable Budget, allocation and organisational references are available.

## 8. Trigger

An authoritative financial demand requires a Budget-control decision.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate the demand and Finance references | Budget & Cost Center Management | Demand is eligible for control assessment |
| 2 | Apply governed Budget availability constraints | Budget & Cost Center Management | Budget-validity result is determined |
| 3 | Record the decision and reason without altering the source demand | Budget & Cost Center Management | Finance outcome is authoritative |
| 4 | Publish `BudgetValidationCompleted` | Budget & Cost Center Management | Originating domain receives Finance outcome |

## 10. Alternate Business Flows

The valid outcomes are Budget-valid and Budget-not-valid; Section J owns any procurement consequence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Budget reference is missing or invalid | Decision cannot be reached | Return the demand for governed correction | F / J |
| Budget is unavailable or insufficient | Demand cannot receive a positive Budget decision | Record Budget-not-valid outcome and preserve reason | Budget & Cost Center Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Finance owns Budget availability decision | Finance | Section J cannot create or alter Finance monetary state | L2-F14 §4 |
| Operational demand remains Section J-owned | Section J | Finance decision does not amend procurement aggregate | L2-F14 §3 |

## 13. State and Lifecycle Impact

The approved owner-domain monetary outcome and F14 collaboration lifecycle apply without altering Section J operational state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| BudgetValidationCompleted | Budget & Cost Center Management | Procurement Finance Consumption and Section J | Communicates the authoritative F06 Budget-control outcome | Approved ACR-F-2026-08-21-001 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Budget & Cost Center Management | Authoritative financial demand and procurement reference | Provide demand for Finance validation | L2-J03; L2-F14 §§5–6 |
| Budget & Cost Center Management | Section J | Budget-control decision | Inform downstream procurement governance | L2-F14 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Make Budget-control decision | Budget Control Officer | Final routine authority under approved Budget Control Policy | Budget & Cost Center Management |

### 15.2 Separation of Duties, Overrides and Exceptions

The Budget Control Officer issues the routine Budget-control decision under approved policy and remains separate from procurement authority.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Source demand reference | Budget & Cost Center Management |
| Preserve scenario decision traceability | Budget, allocation, Cost Center, fund and period references | Budget & Cost Center Management |
| Preserve scenario decision traceability | Decision, reason and authority | Budget & Cost Center Management |

Section A retains ownership of Authorization and Audit infrastructure; Budget & Cost Center Management owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A governed Budget-control decision exists.
- The source demand remains Section J-owned.
- `BudgetValidationCompleted` is published.

### 16.2 Postconditions

- Section J may govern its downstream procurement path using the Finance outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No unresolved material architecture issue remains for this scenario. The former upstream gaps were resolved by approved `ACR-F-2026-08-21-001` and its exact amendment specification.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** PASS  
**Validation Note:** Source-grounded Draft revalidated against approved ACR-F-2026-08-21-001; no unresolved material L3 issue remains.
