# Operational Allocation Change to Controlled Reallocation

# 1. Scenario Metadata

**Scenario ID:** L3-G-295  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern an authorized change to an existing Infrastructure Allocation so the allocation reflects the revised Academic Delivery requirement while preserving effective-dated G10 history and consumer-process ownership.

# 3. Business Outcome

Infrastructure allocation is updated without changing ownership of the consuming operational process.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised operational requirement changes.

## 4.2 End Boundary

The scenario ends when the authoritative Infrastructure Allocation reflects the changed requirement, prior allocation history remains traceable, and `InfrastructureAllocationUpdated` has been published.

## 4.3 In Scope

- Identification of the affected G10 Infrastructure Allocation.
- Revised Academic Delivery operational requirement.
- Revalidation of resource, organization and allocation policy.
- Effective-dated allocation update and history preservation.
- Canonical allocation-update event publication.

## 4.4 Out of Scope

- Timetable revision or publication.
- Temporary Reservation rescheduling.
- Physical-infrastructure identity/hierarchy change.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns the Infrastructure Allocation and its controlled update/history. |
| Academic Delivery / D | Owns the changed operational/scheduling requirement that motivates reallocation. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the G10 allocation change. | Makes the authoritative allocation change after G10 validation. |
| Infrastructure Coordinator | Represents D03's changed physical-resource requirement. | Validates infrastructure allocation within D03 and retains academic-process ownership. |

# 7. Preconditions and Entry Criteria

- An authoritative Infrastructure Allocation exists.
- The changed Academic Delivery requirement is valid.
- The target physical resource and organizational context can be validated.
- Prior allocation history is available.
- The change is authorized.

# 8. Trigger

An authorised operational requirement changes.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the changed Academic Delivery requirement and identify the affected G10 allocation. | Academic Delivery / Infrastructure Management | The reallocation is bounded to an authoritative existing allocation. |
| 2 | Revalidate the physical resource, organization and applicable allocation policy for the revised requirement. | Infrastructure Management | The proposed reallocation is eligible to proceed. |
| 3 | Determine the effective allocation change while preserving prior effective history. | Infrastructure Management | Prior and current allocation facts remain reconstructable. |
| 4 | Make the updated Infrastructure Allocation authoritative. | Infrastructure Management | The frozen outcome is achieved without changing D03 process ownership. |
| 5 | Publish `InfrastructureAllocationUpdated`. | Infrastructure Management | The completed allocation modification fact is available to authorized consumers. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Revised resource, organization or allocation-policy validation fails. | The requested reallocation is not authoritative. | Retain the existing allocation until a later compliant requirement is authorized. | Infrastructure Management |
| Authorization fails. | Current allocation remains effective. | Proceed only after a later valid authorization decision. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Operational Allocation remains G10-owned and never transfers physical-infrastructure ownership. | Infrastructure Management | Reallocation cannot make Academic Delivery the owner of the resource/allocation. | L2-G10 §27 |
| Operational allocations may be effective-dated and historical allocations remain preserved. | Infrastructure Management | The update preserves prior effective history. | L2-G10 §27; Effective Dating principle |
| `InfrastructureAllocationUpdated` is the canonical allocation-modification event. | Infrastructure Management | L3 uses the exact source event name. | L2-G10 Allocation Events |
| D03 owns scheduling decisions while consuming G10 infrastructure. | Academic Delivery / Infrastructure Management | The changed operational requirement remains a D03 concern. | L2-D03 §§1.10–1.11, 2.8 |

# 13. State and Lifecycle Impact

No named Infrastructure Allocation lifecycle states are defined for reallocation.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure Allocation | Existing authoritative allocation | Establishes the successor effective allocation position and preserves prior history | Updated authoritative allocation; no invented lifecycle state | Infrastructure Management | L2-G10 §27 |
| Academic Delivery process | Existing D03 position | Supplies the revised requirement | Remains under D03 ownership and lifecycle | Academic Delivery | L2-D03 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InfrastructureAllocationUpdated` | Infrastructure Management | Authorized allocation consumers | States that the G10 Infrastructure Allocation changed. | L2-G10 Allocation Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Academic Delivery | Infrastructure Management | Changed authorized operational infrastructure requirement | Supplies the revised D03 need without transferring process ownership. | Frozen semantics; L2-D03 |
| Infrastructure Management | Academic Delivery | Updated authoritative Infrastructure Allocation fact | Makes the governed G10 reallocation available to D03. | L2-G10 §27 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Change Infrastructure Allocation | Infrastructure Administrator | Must satisfy G10 authorization, organization and allocation policy. | Infrastructure Management |
| Supply changed Academic Delivery requirement | Infrastructure Coordinator | Acts within D03 scheduling responsibilities only. | Academic Delivery |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or mandatory approval rule for ordinary reallocation is defined upstream.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve effective-dated allocation history | Prior and current Infrastructure Allocation positions remain historically reconstructable. | Infrastructure Management |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative Infrastructure Allocation reflects the changed operational requirement.
- Prior allocation history remains traceable.
- `InfrastructureAllocationUpdated` is published by Infrastructure Management.
- Academic Delivery retains ownership of its operational process.

## 16.2 Postconditions

- Authorized consumers may rely on the current G10 allocation fact.
- Historical allocation positions remain preserved.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 defines allocation establishment, effective dating and the exact `InfrastructureAllocationUpdated` event but does not provide a separate detailed permanent-reallocation workflow. | The frozen outcome is supportable, but L3 cannot add more granular reallocation decisions beyond the source-defined allocation rules. | Upstream G10 may define a dedicated reallocation workflow if later architecture requires more detailed business sequencing. |

**Source Assessment:** **PASS WITH ISSUES**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
