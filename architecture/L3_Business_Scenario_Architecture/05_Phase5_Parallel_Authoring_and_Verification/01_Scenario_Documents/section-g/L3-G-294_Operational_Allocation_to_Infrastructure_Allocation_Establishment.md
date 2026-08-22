# Operational Allocation to Infrastructure Allocation Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-294  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows  
**Referenced By:** Academic Delivery and other authorized consumers of authoritative Infrastructure Allocation facts

# 2. Scenario Purpose

Establish the authoritative Infrastructure Allocation needed when Academic Delivery requires a physical Infrastructure resource for authorized operational use, while preserving Infrastructure Management ownership of the physical resource and Academic Delivery ownership of its scheduling and operational decisions.

# 3. Business Outcome

The resource allocation is recorded against the applicable physical infrastructure

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a consuming domain requires an infrastructure resource for authorised operational use.

## 4.2 End Boundary

The scenario ends when an effective Infrastructure Allocation has been established against the applicable physical resource under G10 governance for the authorized operational requirement.

## 4.3 In Scope

- Receipt of the material operational Infrastructure requirement from Academic Delivery.
- Validation of the target Infrastructure and organizational context under G10 allocation governance.
- Application of allocation policy and effective dating.
- Establishment of the authoritative Infrastructure Allocation against the physical resource.
- Preservation of ownership boundaries between physical Infrastructure and the consuming operational process.

## 4.4 Out of Scope

- Creation, publication, revision, or ownership of a timetable.
- Creation or maintenance of Infrastructure master records.
- Transfer of physical Infrastructure ownership to Academic Delivery.
- Temporary Reservation governance, which is distinct from long-term Infrastructure Allocation.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-293 — Infrastructure Reservation Cancellation to Resource Availability Restoration | Reservation is a separate temporary-booking capability and does not replace long-term Infrastructure Allocation. |
| Followed by | L3-G-295 — Operational Allocation Change to Controlled Reallocation | Governs a later change to an established Infrastructure Allocation. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure resource and the authoritative Infrastructure Allocation recorded against it. |
| Academic Delivery / Section D | Supplies the authorized operational requirement for physical Infrastructure and retains ownership of timetable/scheduling decisions and operational purpose. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the Infrastructure Allocation under G10 governance. | Performs governed Infrastructure management within authorized scope. |
| Infrastructure Coordinator | Represents the Academic Delivery scheduling concern for physical-resource availability and allocation. | Verifies room availability and validates infrastructure allocation within D03 responsibilities; does not own the G10 physical resource. |

# 7. Preconditions and Entry Criteria

- The applicable physical Infrastructure resource exists under Infrastructure Management ownership.
- The resource is eligible for allocation under G10 lifecycle and allocation rules.
- The operational requirement is valid within the consuming Academic Delivery context.
- The organizational context required by the G10 allocation workflow can be validated.

# 8. Trigger

A consuming domain requires an infrastructure resource for authorised operational use

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Academic Delivery establishes the operational need for an Infrastructure resource without redefining the physical resource. | Academic Delivery / Infrastructure Coordinator | An authorized operational Infrastructure requirement is available to G10. |
| 2 | Identify the applicable physical Infrastructure resource and validate the organizational context for allocation. | Infrastructure Management | The resource and organizational allocation target are valid under G10 governance. |
| 3 | Apply the source-defined authorization, organization and allocation-policy validations, including the applicable effective date. | Infrastructure Management | The requested long-term allocation is eligible to be established. |
| 4 | Establish the Infrastructure Allocation against the applicable physical resource. | Infrastructure Management | The frozen Business Outcome is achieved while canonical Infrastructure ownership remains with G10. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Organization validation fails | The requested allocation cannot be associated with a valid organizational context. | Correct the authoritative organizational requirement/context before a later allocation request; no allocation is established meanwhile. | Infrastructure Management |
| Allocation policy validation fails | The physical resource cannot be allocated under the requested conditions. | The consuming domain retains its operational requirement and may submit a later requirement that satisfies G10 policy; no G10 allocation is created. | Infrastructure Management; Academic Delivery retains its own operational decision. |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Operational Allocation governs long-term assignment of Infrastructure to organisational units and does not transfer canonical Infrastructure ownership. | Infrastructure Management | Academic Delivery receives authorized use context; G10 remains owner of the physical resource and Infrastructure Allocation. | L2-G10 §27 Operational Allocation Workflows |
| The allocation workflow validates authorization, organisation and policy and assigns an effective date before the allocation is established. | Infrastructure Management | The allocation cannot be established without these source-defined business validations. | L2-G10 §27 Allocation Workflow |
| Only Operational Spaces may be allocated by consumer domains. | Infrastructure Management | A non-operational Space cannot satisfy the requested allocation outcome. | L2-G10 FSM-04 |
| D03 consumes physical Infrastructure resources from G10 and does not own Infrastructure master maintenance. | Academic Delivery; Infrastructure Management | Academic scheduling requirements cannot redefine G10 physical Infrastructure. | L2-D03 §§1.10–1.11, 2.8 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure Allocation | No canonical allocation lifecycle states are defined by L2-G10. | A new effective-dated Infrastructure Allocation is established against the physical resource. | Established authoritative allocation fact; no new lifecycle state is invented. | Infrastructure Management | L2-G10 §27; effective-dating principle |
| Space / physical Infrastructure | Operational, where allocation is permitted. | Referenced by the allocation; physical identity and ownership remain unchanged. | Existing G10 lifecycle position remains unchanged by the allocation itself. | Infrastructure Management | L2-G10 FSM-04 and canonical Infrastructure lifecycle |
| Timetable / Academic Delivery scheduling artifacts | Source-defined D03 state, if any participating scheduling context exists. | Supply the operational requirement only. | Remain under D03 ownership; no D03 lifecycle transition is created by this G10 scenario. | Academic Delivery | L2-D03 ownership and scope |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `DepartmentAllocated` | Infrastructure Management | No specific consumer is named for this event in L2-G10. | Represents completion of the source-defined department/organizational Infrastructure Allocation. | L2-G10 §44 Allocation Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Academic Delivery | Infrastructure Management | Authorized operational Infrastructure requirement | Provides the operational need without transferring ownership of the consuming process to G10. | Frozen Stage 3 semantics; L2-D03 dependency on G10 physical resources |
| Infrastructure Management | Academic Delivery | Authoritative Infrastructure Allocation fact | Provides the governed physical-resource allocation that Academic Delivery may reference for its own scheduling decisions. | L2-G10 §27; §45 business capabilities |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish Infrastructure Allocation | Infrastructure Administrator | Allocation must satisfy the authorization, organization and policy validations defined by G10. | Infrastructure Management |
| Supply the Academic Delivery infrastructure requirement | Infrastructure Coordinator | May validate Infrastructure allocation within D03 scheduling responsibilities; cannot change G10 physical ownership. | Academic Delivery |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, approval, override, or exception requirement for ordinary Infrastructure Allocation establishment is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the established allocation and its effective dating | The authoritative Infrastructure Allocation and its effective period remain historically traceable. | Infrastructure Management |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A valid Infrastructure Allocation is recorded against the applicable physical resource.
- The allocation has the applicable effective dating required by G10 governance.
- The consuming Academic Delivery process retains ownership of its own operational decision.

## 16.2 Postconditions

- The authoritative Infrastructure Allocation is available for authorized business use under G10 ownership.
- The physical Infrastructure resource remains canonically owned by Infrastructure Management.
- `DepartmentAllocated` represents the completed allocation fact where the source-defined event applies.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

**Architecture assessment:** `PASS`

The frozen outcome, ownership boundary, governing workflow, allocation rule, event and cross-domain responsibility are sufficiently established by the authoritative sources.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
