# Operational Allocation Change to Controlled Reallocation

# 1. Scenario Metadata

**Scenario ID:** L3-G-295  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows  
**Referenced By:** Academic Delivery and other authorized consumers of authoritative Infrastructure Allocation facts

# 2. Scenario Purpose

Govern an authorized change to an existing Infrastructure Allocation so that the physical-resource allocation reflects the revised operational requirement while preserving effective-dated history, G10 ownership of the Infrastructure Allocation, and the consuming domain's ownership of its own operational process.

# 3. Business Outcome

Infrastructure allocation is updated without changing ownership of the consuming operational process

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised operational requirement changes.

## 4.2 End Boundary

The scenario ends when the authoritative Infrastructure Allocation has been updated under G10 governance for the changed operational requirement, with prior allocation history remaining traceable and no transfer of the consuming process's ownership.

## 4.3 In Scope

- Identification of the existing Infrastructure Allocation affected by the changed operational requirement.
- Revalidation of the applicable physical resource, organizational context, allocation policy and effective dating.
- Controlled update of the Infrastructure Allocation.
- Preservation of prior allocation history.
- Preservation of Academic Delivery ownership of its operational and scheduling decisions.

## 4.4 Out of Scope

- Modification of timetable ownership or D03 scheduling lifecycle.
- Modification of Infrastructure master identity or physical hierarchy merely because an allocation changes.
- Reservation rescheduling or reservation cancellation.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-294 — Operational Allocation to Infrastructure Allocation Establishment | Establishes the Infrastructure Allocation that this scenario may later change. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure resource, the Infrastructure Allocation and the governed update to that allocation. |
| Academic Delivery / Section D | Owns the changed operational requirement and any timetable/scheduling decision that motivates the allocation change. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the governed Infrastructure Allocation change. | Performs Infrastructure management within authorized scope. |
| Infrastructure Coordinator | Represents the Academic Delivery concern for physical-resource availability and allocation. | Validates Infrastructure allocation within D03 scheduling responsibilities without owning the G10 allocation. |

# 7. Preconditions and Entry Criteria

- An authoritative Infrastructure Allocation already exists under G10 ownership.
- The changed operational requirement is valid within the consuming Academic Delivery context.
- The target Infrastructure and organizational context can be validated under G10 allocation governance.
- Prior allocation history remains available for effective-dated traceability.

# 8. Trigger

An authorised operational requirement changes

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the changed Academic Delivery operational requirement and identify the existing Infrastructure Allocation it affects. | Academic Delivery; Infrastructure Management | The required allocation change is bounded to an existing authoritative G10 allocation. |
| 2 | Validate the applicable physical resource, organization and allocation policy for the revised requirement. | Infrastructure Management | The revised allocation remains eligible under G10 governance. |
| 3 | Determine the effective-dated allocation change without rewriting the prior allocation history. | Infrastructure Management | The prior and successor allocation positions remain historically traceable. |
| 4 | Update the authoritative Infrastructure Allocation for the changed requirement. | Infrastructure Management | The frozen Business Outcome is achieved without changing ownership of the consuming operational process. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The revised organization or resource context fails G10 validation | The requested reallocation cannot become authoritative. | Retain the current Infrastructure Allocation until a later changed requirement satisfies G10 governance. | Infrastructure Management |
| The revised allocation violates applicable allocation policy | The requested allocation update is not accepted. | Academic Delivery retains its operational decision responsibility and may provide a later compliant requirement; G10 retains the current authoritative allocation meanwhile. | Infrastructure Management; Academic Delivery |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Operational Allocation is a long-term assignment of Infrastructure and does not transfer canonical Infrastructure ownership. | Infrastructure Management | Updating the allocation cannot transfer physical-resource ownership to Academic Delivery. | L2-G10 §27 Operational Allocation Workflows |
| Operational allocations support effective dating where historical traceability is required. | Infrastructure Management | Reallocation preserves the prior effective allocation instead of rewriting history. | L2-G10 Effective Dating principle |
| `InfrastructureAllocationUpdated` is the source-defined event for an allocation modification. | Infrastructure Management | The scenario uses this exact event name and does not create a local event. | L2-G10 §44 Allocation Events |
| D03 consumes G10 physical Infrastructure resources while retaining ownership of timetable and scheduling decisions. | Academic Delivery; Infrastructure Management | The changed operational requirement remains a D03-owned decision even though G10 updates the physical allocation. | L2-D03 §§1.10–1.11, 2.8 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure Allocation | No canonical allocation lifecycle states are defined by L2-G10. | The current effective-dated allocation is superseded by the governed updated allocation position. | Updated authoritative allocation fact with prior effective history preserved; no lifecycle state is invented. | Infrastructure Management | L2-G10 §27; Effective Dating principle; §44 Allocation Events |
| Physical Infrastructure | Existing source-defined lifecycle position compatible with allocation. | Referenced by the updated allocation; identity and ownership are unchanged. | Existing Infrastructure lifecycle remains under G10. | Infrastructure Management | L2-G10 canonical lifecycle and allocation ownership |
| Academic Delivery process | Source-defined D03 lifecycle, where applicable. | Supplies the changed operational requirement. | Remains owned and governed by D03. | Academic Delivery | L2-D03 domain ownership |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InfrastructureAllocationUpdated` | Infrastructure Management | No specific consumer is named for this event in L2-G10. | Represents completion of the source-defined Infrastructure Allocation modification. | L2-G10 §44 Allocation Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Academic Delivery | Infrastructure Management | Changed authorized operational Infrastructure requirement | Provides the revised operational need while D03 retains its own process ownership. | Frozen Stage 3 semantics; L2-D03 dependency on G10 |
| Infrastructure Management | Academic Delivery | Updated authoritative Infrastructure Allocation fact | Makes the governed allocation change available for the consuming process to reference. | L2-G10 §27 and §45 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Change the Infrastructure Allocation | Infrastructure Administrator | The changed allocation must satisfy G10 authorization, organization and policy governance. | Infrastructure Management |
| Supply/validate the changed Academic Delivery requirement | Infrastructure Coordinator | D03 may determine its scheduling requirement but cannot change G10 Infrastructure ownership. | Academic Delivery |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, approval, override, or exception requirement for ordinary Infrastructure reallocation is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve effective-dated allocation history | Prior and current Infrastructure Allocation positions remain historically reconstructable. | Infrastructure Management |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative Infrastructure Allocation reflects the changed operational requirement.
- Prior allocation history remains traceable through G10 effective dating.
- Ownership of the consuming Academic Delivery process has not changed.

## 16.2 Postconditions

- Authorized consumers may reference the current Infrastructure Allocation fact under G10 ownership.
- `InfrastructureAllocationUpdated` represents the completed G10 allocation modification.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 defines the allocation-establishment workflow and the `InfrastructureAllocationUpdated` event, but does not define a separate detailed reallocation workflow. | The frozen reallocation outcome is supportable from the allocation ownership, effective-dating rule and update event, but L3 must not invent additional transition-specific decision steps. | Upstream L2-G10 may add an explicit reallocation workflow if more detailed authoritative behaviour is required for later approval. |

**Architecture assessment:** `PASS WITH ISSUES`

The Draft is usable and preserves the frozen outcome and source-defined allocation semantics. The missing dedicated reallocation workflow is recorded rather than silently synthesized.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
