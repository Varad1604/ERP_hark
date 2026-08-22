# Infrastructure Availability Change to Suspension or Restoration Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-G-290  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** D, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern a change in physical-resource availability so Infrastructure Management remains authoritative for current usability while Academic Delivery consumes that fact for its own scheduling decisions.

# 3. Business Outcome

Availability is suspended or restored under policy and consuming domains receive the authoritative current usability state.

G10 requires a canonical event for the availability workflow but does not define one exact event for suspension/restoration. The Draft is **BLOCKED** pending upstream event definition.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a physical resource becomes unavailable, restricted, or eligible to return to service.

## 4.2 End Boundary

The scenario ends when G10 has established the source-defined current availability condition, Academic Delivery can consume the authoritative usability fact, and the required canonical availability event can be published unambiguously.

## 4.3 In Scope

- Availability suspension/restriction or restoration under G10 policy.
- Lifecycle and business validation.
- Scoped or scheduled availability changes.
- Provision of the authoritative current usability fact to Academic Delivery.
- Required G10 event publication.

## 4.4 Out of Scope

- Timetable revision, publication or other Academic Delivery decisions.
- Maintenance execution.
- Treating “suspended” or “restored” as new formal availability states where G10 has not named them.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns the current infrastructure availability fact and its governing rules. |
| Academic Delivery / D | Consumes physical-resource availability for scheduling and retains ownership of timetable decisions. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the governed availability change. | Performs the G10 action only where authorized. |
| Infrastructure Coordinator | Uses authoritative infrastructure availability in D03 scheduling responsibilities. | May verify room availability and validate infrastructure allocation; cannot change G10 ownership. |

# 7. Preconditions and Entry Criteria

- The affected physical resource exists.
- Its current lifecycle and availability are known.
- The source condition for restriction, unavailability or restoration is established.
- The requested availability change is permitted by policy and authorization.

# 8. Trigger

A physical resource becomes unavailable, restricted, or eligible to return to service.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the affected physical resource and the business condition changing its usability. | Infrastructure Administrator / Infrastructure Management | The availability change is bounded to canonical infrastructure. |
| 2 | Validate authorization, lifecycle compatibility and the governing availability condition. | Infrastructure Management | The requested change is eligible to proceed. |
| 3 | Apply the source-defined availability condition to the correct hierarchy scope and effective period. | Infrastructure Management | Current usability is suspended/restricted or restored under policy without changing unrelated infrastructure. |
| 4 | Make the authoritative current usability fact available to Academic Delivery. | Infrastructure Management → Academic Delivery | D03 can make its own scheduling decisions from the G10 fact. |
| 5 | Publish the canonical availability-change business event required by G10. | Infrastructure Management | **Blocked:** no exact canonical event name is established for the suspension/restoration outcome. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Partial restriction | Only one hierarchy level or selected spaces are affected. | Apply the condition only to that governed scope. | Unrelated infrastructure remains governed by its own availability. | Infrastructure Management |
| Scheduled restriction | The unavailability begins or ends at a defined future period. | Apply the source-defined schedule and reason. | Availability changes when the governed period becomes effective. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Availability change conflicts with lifecycle or policy. | The prior authoritative usability fact remains. | Correct the business condition before a later authorized change. | Infrastructure Management |
| Authorization is not granted. | No G10 availability change occurs. | Proceed only after a valid later authorization decision. | Infrastructure Management |
| Required event name is undefined. | Canonical publication cannot be completed. | Define the exact G10 availability event upstream. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Availability is distinct from lifecycle. | Infrastructure Management | Availability change does not create a lifecycle transition. | L2-G10 §25 |
| Availability may be changed by maintenance, failure, closure, emergency, reservation, compliance restriction, disaster recovery or authorized administrative action. | Infrastructure Management | Only source-supported business conditions may govern the change. | L2-G10 §25 |
| Partial closure does not necessarily affect unrelated infrastructure. | Infrastructure Management | Scope must not be expanded beyond the affected resource hierarchy. | L2-G10 §25 |
| D03 consumes G10 physical infrastructure resources and owns timetable/scheduling decisions. | Academic Delivery / Infrastructure Management | Receiving availability does not transfer G10 ownership or timetable ownership. | L2-D03 §§1.10–1.11, 2.8 |
| Every successful G10 workflow publishes a canonical business event. | Infrastructure Management | Missing exact event name blocks source-complete completion. | L2-G10 WDP-02 |

# 13. State and Lifecycle Impact

The frozen words “suspended” and “restored” describe the business outcome. G10's named availability states include `Available`, `Reserved`, `Under Maintenance`, `Under Renovation`, `Temporarily Closed`, `Emergency Closed`, and `Operationally Restricted`; L3 does not invent `Suspended` or `Restored` as new states.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure Availability | Existing source-defined availability condition | Applies a governed restriction/unavailability or returns to a source-defined usable condition | Source-defined current availability state appropriate to the business condition | Infrastructure Management | L2-G10 §25 |
| Timetable / D03 scheduling | Existing D03 position | Consumes current G10 usability | No D03 transition is mandated by this G10 scenario | Academic Delivery | L2-D03 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10's availability workflow requires publication but no exact canonical event for availability suspension/restoration is defined. Generic lifecycle/operational-status events such as `InfrastructureActivated` or `InfrastructureDeactivated` do not resolve this because G10 expressly keeps availability separate from lifecycle. L3 does not substitute them.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Infrastructure Management | Academic Delivery | Authoritative current infrastructure usability fact | Allows D03 to evaluate its own scheduling/resource decisions without owning availability. | L2-G10 §25; L2-D03 dependency on G10 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Change infrastructure availability | Infrastructure Administrator | Must satisfy G10 authorization and availability rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule is defined. Manual override remains subject to authorization.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the governed availability change | Prior/current availability condition plus applicable effective period, reason and responsible authority defined by G10. | Infrastructure Management |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The resource's current authoritative usability reflects the frozen suspension/restoration outcome under policy.
- Academic Delivery can consume the authoritative G10 fact without owning it.
- The exact canonical availability-change event is defined and published by Infrastructure Management.

## 16.2 Postconditions

- G10 remains owner of infrastructure availability.
- D03 remains owner of any scheduling response.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 requires event publication for availability changes but defines no exact canonical event for the frozen suspension/restoration outcome. | L3 cannot name the mandatory event without inventing or substituting a lifecycle event. | Upstream G10/event governance must define the canonical availability-change event. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
