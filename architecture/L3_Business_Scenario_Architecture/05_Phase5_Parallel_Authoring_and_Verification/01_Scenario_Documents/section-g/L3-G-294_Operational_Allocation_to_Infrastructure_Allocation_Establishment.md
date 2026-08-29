# Operational Allocation to Infrastructure Allocation Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-294  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Establish the authoritative long-term Infrastructure Allocation required for an Academic Delivery operational need while preserving G10 ownership of physical infrastructure and D03 ownership of scheduling decisions.

# 3. Business Outcome

The resource allocation is recorded against the applicable physical infrastructure.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a consuming domain requires an infrastructure resource for authorised operational use.

## 4.2 End Boundary

The scenario ends when an effective Infrastructure Allocation is recorded against the applicable G10 physical resource for the authorized operational requirement.

## 4.3 In Scope

- Academic Delivery's operational infrastructure requirement.
- G10 authorization, organization and allocation-policy validation.
- Effective-dated Infrastructure Allocation establishment.
- Preservation of physical-resource and consumer-process ownership boundaries.
- `DepartmentAllocated` publication.

## 4.4 Out of Scope

- Timetable creation, publication or revision.
- Temporary Infrastructure Reservation.
- Physical-infrastructure master-data change.
- Transfer of infrastructure ownership to Academic Delivery.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns the physical resource and authoritative Infrastructure Allocation. |
| Academic Delivery / D | Owns the operational/scheduling requirement that consumes the allocated resource. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the G10 allocation. | Establishes the allocation after source-defined validation. |
| Infrastructure Coordinator | Represents D03's physical-resource allocation concern. | Validates infrastructure allocation within D03 while not owning the G10 resource/allocation. |

# 7. Preconditions and Entry Criteria

- The applicable physical resource exists under G10 ownership.
- The resource is in a source-defined position permitting allocation.
- The Academic Delivery operational requirement is valid.
- The organizational context can be validated.
- The allocation action is authorized.

# 8. Trigger

A consuming domain requires an infrastructure resource for authorised operational use.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Academic Delivery need for physical infrastructure. | Infrastructure Coordinator / Academic Delivery | An authorized operational resource requirement is available without transferring D03 process ownership. |
| 2 | Identify the physical resource and validate its organizational allocation context. | Infrastructure Management | The resource and organizational target are valid under G10 governance. |
| 3 | Validate authorization, allocation policy and applicable effective date. | Infrastructure Management | The long-term allocation is eligible to be established. |
| 4 | Record the Infrastructure Allocation against the physical resource. | Infrastructure Management | The frozen business outcome is achieved and G10 ownership is preserved. |
| 5 | Publish `DepartmentAllocated`. | Infrastructure Management | The completed allocation fact is available to authorized consumers. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Shared infrastructure | The resource is governed for multiple organizational units. | Apply the G10 shared-allocation configuration without transferring ownership. | Multiple permitted allocations can coexist under G10 policy. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Organization, resource or allocation-policy validation fails. | No Infrastructure Allocation is established. | Correct the business requirement/context before a later allocation request. | Infrastructure Management; Academic Delivery retains its requirement |
| Authorization fails. | Allocation does not proceed. | Proceed only after a later valid authorization decision. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Operational Allocation is long-term assignment of Infrastructure to organizational units. | Infrastructure Management | The scenario does not substitute a temporary Reservation. | L2-G10 §27 |
| Allocation does not transfer canonical infrastructure ownership. | Infrastructure Management | Academic Delivery receives authorized use, not ownership. | L2-G10 §27 |
| Allocation may be effective-dated and historical allocations remain preserved. | Infrastructure Management | The allocation may carry a governed effective period without rewriting history. | L2-G10 §27 |
| D03 consumes G10 physical infrastructure and owns timetable/scheduling decisions. | Academic Delivery / Infrastructure Management | G10 allocation does not make G10 owner of the consuming academic process. | L2-D03 §§1.10–1.11, 2.8 |
| `DepartmentAllocated` is the allocation-completion event. | Infrastructure Management | L3 uses the exact source event name. | L2-G10 Allocation Events |

# 13. State and Lifecycle Impact

G10 defines Infrastructure Allocation as an aggregate but does not define named allocation lifecycle states for this establishment outcome.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure Allocation | No allocation exists for this requirement | Establishes the effective-dated allocation fact | Established authoritative allocation; no invented lifecycle state | Infrastructure Management | L2-G10 §27 |
| Physical infrastructure | Existing source-defined position permitting allocation | Referenced by the allocation | Infrastructure lifecycle unchanged | Infrastructure Management | L2-G10 |
| D03 scheduling context | Existing or future D03 state | Supplies operational requirement | No D03 transition is imposed by this G10 scenario | Academic Delivery | L2-D03 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `DepartmentAllocated` | Infrastructure Management | Authorized allocation consumers | States that the organizational Infrastructure Allocation was established. | L2-G10 Allocation Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Academic Delivery | Infrastructure Management | Authorized operational infrastructure requirement | Supplies D03's need without transferring its process ownership. | Frozen semantics; L2-D03 |
| Infrastructure Management | Academic Delivery | Authoritative Infrastructure Allocation fact | Allows D03 to reference the governed physical allocation. | L2-G10 §27 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish Infrastructure Allocation | Infrastructure Administrator | Must satisfy G10 authorization, organization and policy validation. | Infrastructure Management |
| Supply Academic Delivery resource requirement | Infrastructure Coordinator | Acts within D03 scheduling responsibilities and cannot alter G10 ownership. | Academic Delivery |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or mandatory approval rule for ordinary allocation establishment is defined upstream.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve allocation history | The authoritative allocation and applicable effective period remain historically traceable. | Infrastructure Management |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A valid Infrastructure Allocation is recorded against the applicable physical resource.
- Applicable effective dating is preserved.
- `DepartmentAllocated` is published by Infrastructure Management.
- Academic Delivery retains ownership of its operational process.

## 16.2 Postconditions

- Authorized consumers may rely on the current G10 allocation fact.
- Physical infrastructure remains owned by Infrastructure Management.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

**Source Assessment:** **PASS**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
