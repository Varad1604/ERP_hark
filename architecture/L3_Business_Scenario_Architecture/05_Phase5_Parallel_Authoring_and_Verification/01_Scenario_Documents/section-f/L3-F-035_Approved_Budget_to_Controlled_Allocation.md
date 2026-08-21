# Approved Budget to Controlled Allocation

## 1. Scenario Metadata

**Scenario ID:** L3-F-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** L2-F00; L2-F06; L2-B02; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario distributes an approved active Budget into governed Cost Center allocations while preserving planning, organisational and accounting boundaries.

## 3. Business Outcome

The approved Budget is completely distributed into authorized allocations referencing active Cost Centers, and the allocation outcome is published.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An approved Active Budget is available for controlled allocation.

### 4.2 End Boundary

Authorized allocations equal the Budget total and applicable allocation events are published.

### 4.3 In Scope

- validation of Budget and Cost Center references
- authorization and creation of controlled allocations
- allocation-bound validation and event publication

### 4.4 Out of Scope

- Budget proposal approval
- Cost Center creation
- utilization, procurement, payments or accounting posting

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Finance decision or accounting outcome and its evidence |
| Section B — Institution & Structure | Provides authoritative organisational references without transferring ownership |

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

- The Budget is Active.
- Each target Cost Center is valid and active.
- The approved Budget total and planning scope are established.

## 8. Trigger

The approved Active Budget is ready for controlled distribution.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate the Active Budget and target Cost Center references | Budget & Cost Center Management | Eligible allocation structure is established |
| 2 | Distribute amounts within the approved parent bounds | Budget & Cost Center Management | Proposed allocations are complete |
| 3 | Assign any reserve to the Central Reserve Cost Center | Budget & Cost Center Management | No amount remains unowned |
| 4 | Obtain governed allocation authorization | Budget & Cost Center Management | Allocation is authorized |
| 5 | Create allocations and publish canonical events | Budget & Cost Center Management | Controlled allocation is available |

## 10. Alternate Business Flows

The allocation structure may cover departments, campuses, administrative units, projects or Central Reserve where supported by the approved Budget.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Target Cost Center is inactive or invalid | Allocation cannot be completed | Correct the Section B/Cost Center dependency before resubmission | B / F |
| Allocation totals do not equal Budget TotalAmount | Controlled allocation is invalid | Correct allocation amounts before authorization | Budget & Cost Center Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Allocation total exactly equals Budget TotalAmount | Budget & Cost Center Management | Allocation cannot complete unless balanced | L2-F06 §17.1 |
| Unallocated reserve uses Central Reserve Cost Center | Budget & Cost Center Management | No unowned remainder is permitted | L2-F06 §17.1 |
| Each allocation references one active Cost Center | Budget & Cost Center Management | Invalid Cost Center prevents allocation | L2-F06 §§9.6, 12.1 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Budget Allocation | Not established | Authorized allocation is created under the Active Budget | Controlled allocation exists; no canonical state name is defined | Budget aggregate | L2-F06 §§9.3–9.6, 12 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| BudgetAllocationCreated | Budget & Cost Center Management | Accounting, Procurement and interested Finance domains | Communicates controlled allocation | L2-F06 §14.1 |
| CostCenterFunded | Budget & Cost Center Management | Funded organisational unit and interested domains | Communicates funded Cost Center outcome | L2-F06 §14.2 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section B | Budget & Cost Center Management | Authoritative organisational reference | Validate target organisational scope | L2-B02; L2-F06 §8.3 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Prepare allocation | Finance Department | Within the approved Budget total | Budget & Cost Center Management |
| Authorize allocation | Defined by approved ACR-F-2026-08-21-001 | Required governed authority is unnamed | Budget & Cost Center Management |

### 15.2 Separation of Duties, Overrides and Exceptions

L2-F06 requires governed authorization for structural changes but does not define the allocation approver or separation roles.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Approved Budget and Cost Center references | Budget & Cost Center Management |
| Preserve scenario decision traceability | Balanced allocation schedule and Central Reserve treatment | Budget & Cost Center Management |
| Preserve scenario decision traceability | Authorization decision and canonical event references | Budget & Cost Center Management |

Section A retains ownership of Authorization and Audit infrastructure; Budget & Cost Center Management owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Every allocation references an active Cost Center.
- Allocation total equals the approved Budget total.
- Applicable canonical events are published.

### 16.2 Postconditions

- Controlled allocations are available for Budget validation and monitoring.

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
