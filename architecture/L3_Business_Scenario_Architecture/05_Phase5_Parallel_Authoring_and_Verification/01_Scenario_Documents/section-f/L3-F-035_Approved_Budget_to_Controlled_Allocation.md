# Approved Budget to Controlled Allocation

## 1. Scenario Metadata

**Scenario ID:** L3-F-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Pallav  
**Last Updated:** 2026-09-23  
**Depends On:** L2-F00; L2-F06; `L2-F06_Budget_Allocation_Authority_Amendment_2026-09-23.md`; L2-B02; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
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

`Authorized Budget Administrator` is the canonical abstract preparation actor and `Budget Governance Authority` is the canonical allocation-decision authority under the Locked L2-F06 amendment. Budget & Cost Center Management retains aggregate ownership.

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

`Budget Governance Authority` makes the governed allocation decision. Institutional policy maps that abstract authority to the appropriate office, committee, administrator or governance body.

### 15.2 Separation of Duties, Overrides and Exceptions

Preparation and allocation approval are distinct canonical responsibilities. No universal requirement is imposed that they be different persons; institutional policy may impose stronger separation of duties.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Approved Budget and Cost Center references | Budget & Cost Center Management |
| Preserve scenario decision traceability | Balanced allocation schedule and Central Reserve treatment | Budget & Cost Center Management |
| Preserve scenario decision traceability | Governed allocation decision | Budget & Cost Center Management |

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

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| RESOLVED — L2-F06 Budget Allocation Authority Amendment | Canonical preparation actor, decision authority and approval provenance are now explicit without naming a universal institutional job title. | Closed by Locked L2-F06 amendment approved 2026-09-23. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** PASS  
**Validation Note:** Business semantics remain grounded in L2-F06. Actor/decision authority and approval provenance are supplied by the Locked L2-F06 Budget Allocation Authority Amendment; no unsupported ACR or universal job title is used.


## 2026-09-23 Upstream Authority Reconciliation

L3-F-035 is reconciled to `L2-F06_Budget_Allocation_Authority_Amendment_2026-09-23.md`. Authorized Budget Administrator prepares proposed allocations; Budget Governance Authority makes the governed allocation decision; L2-F06 remains canonical owner and publisher of `BudgetAllocationCreated`.
