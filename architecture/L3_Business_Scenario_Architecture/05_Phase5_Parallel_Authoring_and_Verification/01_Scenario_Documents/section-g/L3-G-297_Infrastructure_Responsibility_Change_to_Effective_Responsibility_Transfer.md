# Infrastructure Responsibility Change to Effective Responsibility Transfer

# 1. Scenario Metadata

**Scenario ID:** L3-G-297  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, E, A  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B02 — Organisation Workflows; L2-E02 — Position & Assignment Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized Infrastructure consumers requiring current operational-responsibility context

# 2. Scenario Purpose

Govern a change in the responsible organizational or operational party recorded against physical Infrastructure so that the successor responsibility becomes effective without erasing the prior G10 responsibility history and without transferring ownership of organizational structure, workforce assignments, authorization, or physical Infrastructure.

# 3. Business Outcome

The new responsibility becomes effective while historical responsibility remains traceable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when the authorised responsibility for infrastructure changes.

## 4.2 End Boundary

The scenario ends when the successor Infrastructure-specific responsible-party record is effective against the physical resource and the previous responsibility remains historically traceable under G10 governance.

## 4.3 In Scope

- Identification of the current Infrastructure responsibility association and proposed successor responsibility.
- Reference to authoritative organization facts from B02 and workforce assignment facts from E02 where applicable.
- Validation of the source-supported Infrastructure responsibility type and authorization for the change.
- Effective-dated change of the G10 Infrastructure-specific responsibility association.
- Preservation of prior responsibility history.

## 4.4 Out of Scope

- Changing Organization, Position Definition, Position Assignment, E02 Responsibility Assignment, or Workforce Engagement lifecycles.
- Changing authorization-layer roles or Authorization Delegation.
- Reassigning ownership of the physical Infrastructure resource.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-296 — Infrastructure Operational Responsibility Assignment to Governance Record | Establishes the Infrastructure-specific responsibility record that this scenario may later change. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical resource and the effective-dated Infrastructure-specific responsibility association and its history. |
| Institution & Structure / Section B | Owns organization structure and Position Definition facts referenced in the transfer. |
| People & Workforce / Section E | Owns Position Assignment, Responsibility Assignment, occupancy and organizational authority facts referenced for workforce responsibility context. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence; it does not own the Infrastructure responsibility transfer outcome. |

The scenario changes only the G10 responsibility association against Infrastructure. It does not transfer or recreate B02/E02 canonical ownership.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the governed change to the Infrastructure-specific responsibility record. | Performs Infrastructure management within authorized scope. |
| Institution Administrator | Provides institution-wide Infrastructure oversight where the responsibility change falls within that governance scope. | Participates in oversight without owning G10 physical facts. |
| Infrastructure Custodian | May be the incumbent or successor where Custodian is the applicable G10 responsibility type. | Holds administrative Infrastructure oversight for its effective period. |
| Operational Manager | May be the incumbent or successor where Operational Manager is the applicable G10 responsibility type. | Holds day-to-day Infrastructure operational responsibility for its effective period. |
| Auditor | Reviews historical responsibility evidence. | Read-only historical review. |

Other responsibility types supported by L2-G10 remain available where applicable; this scenario does not invent additional responsibility categories.

# 7. Preconditions and Entry Criteria

- A current Infrastructure-specific responsibility association exists or is otherwise authoritatively identifiable under G10.
- The physical Infrastructure resource exists under G10 ownership.
- The successor organizational/workforce context is authoritative under B02/E02 where applicable.
- The responsibility type is supported by L2-G10.
- The responsibility change is authorized before becoming effective.

# 8. Trigger

The authorised responsibility for infrastructure changes

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the physical Infrastructure resource, current responsibility association and the authorized change requirement. | Infrastructure Management | The responsibility transfer is bounded to one authoritative G10 resource and responsibility context. |
| 2 | Resolve the successor organizational/workforce reference from B02/E02 where applicable without changing those upstream facts. | Institution & Structure; People & Workforce; Infrastructure Management | The proposed successor responsible party is grounded in canonical upstream context. |
| 3 | Confirm authorization and determine the effective dating for the successor Infrastructure responsibility. | Infrastructure Management, using the canonical authorization decision owned by Section A | The successor responsibility is eligible to become effective under G10 governance. |
| 4 | Make the successor Infrastructure responsibility effective while retaining the prior responsibility as historical G10 evidence. | Infrastructure Management | The frozen Business Outcome is achieved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Temporary responsibility delegation reaches its end | The current G10 responsibility is a temporary delegation governed by effective dates and automatic restoration. | Restore the source-defined prior responsibility according to the G10 delegation rules while preserving the delegation history. | The restored responsibility becomes current and the temporary delegation remains traceable. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Successor organization/workforce reference is not authoritative | The transfer cannot create a valid Infrastructure responsibility association. | Correct the B02/E02 source fact before a later transfer; retain the existing G10 responsibility meanwhile. | Institution & Structure or People & Workforce; Infrastructure Management |
| Authorization does not permit the responsibility change | The current Infrastructure responsibility remains effective. | Proceed only after a later authorized business action; no successor G10 responsibility is asserted meanwhile. | Platform Foundation — Authorization; Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 supports Infrastructure Custodian, Operational Manager, Administrative Owner, Maintenance Coordinator, Compliance Officer and Safety Officer responsibility types. | Infrastructure Management | The transfer may only concern a source-supported Infrastructure responsibility type. | L2-G10 §28 Operational Responsibility Management |
| Responsibility assignments support effective dating for historical traceability. | Infrastructure Management | The successor can become effective without erasing the prior responsibility record. | L2-G10 Effective Dating principle; §28 |
| Temporary G10 responsibility delegation supports effective dates, delegation reason, automatic restoration and complete history. | Infrastructure Management | A temporary change may restore the prior responsibility according to source-defined delegation semantics. | L2-G10 §28 Operational Responsibility Management |
| B02 owns organization structure and Position Definition. | Institution & Structure | G10 cannot alter organizational structure as part of an Infrastructure responsibility transfer. | L2-B02 ownership model |
| E02 owns Position Assignment and Responsibility Assignment. | People & Workforce | G10 references workforce authority/assignment facts but does not transfer E02 aggregate ownership. | L2-E02 §§1.3–1.4, 2.5 |
| Authorization decisions remain owned by A04. | Platform Foundation — Authorization | G10 applies the authorized business decision without owning the authorization lifecycle. | L2-A04 §§3, 5 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure / Space | Existing source-defined lifecycle position. | Its current G10 responsible-party association changes effective-dated; physical lifecycle is not changed. | Same Infrastructure lifecycle position with a new current responsibility association and preserved prior history. | Infrastructure Management | L2-G10 §28 and Effective Dating principle |
| E02 Position Assignment / Responsibility Assignment | Existing authoritative E02 context where applicable. | Referenced to resolve the successor workforce authority/context. | No E02 lifecycle transition is performed by this G10 scenario. | People & Workforce | L2-E02 canonical ownership |
| B02 organization / Position Definition | Existing authoritative B02 context. | Referenced to resolve organizational structure/position context. | No B02 lifecycle transition is performed by this G10 scenario. | Institution & Structure | L2-B02 canonical ownership |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-G10 requires every significant Infrastructure business transition to generate a domain event, but no exact canonical event name for an Infrastructure responsibility change/transfer is defined in its event catalogue. This Draft therefore does not invent or rename an event. The gap is recorded in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institution & Structure | Infrastructure Management | Authoritative organization / Position Definition reference | Supplies structural context for the successor responsibility without transferring B02 ownership. | L2-B02; L2-G10 §9 Dependencies |
| People & Workforce | Infrastructure Management | Authoritative workforce / Position Assignment / Responsibility Assignment context | Supplies workforce organizational-authority context without transferring E02 lifecycle ownership. | L2-E02; L2-G10 §9 Dependencies |
| Platform Foundation — Authorization | Infrastructure Management | Authorization decision | Determines whether the G10 responsibility change may proceed. | L2-A04 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Change the Infrastructure-specific responsible party | Infrastructure Administrator | Must act within the applicable Infrastructure scope and against valid B02/E02 references. | Infrastructure Management |

L2-G10 does not designate one universal named approver for all Infrastructure responsibility transfers. No such approver is invented here.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or mandatory approval requirement for ordinary Infrastructure responsibility change is defined by the authoritative L2 architecture. Temporary delegation follows the source-defined G10 delegation requirements only.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve current and historical Infrastructure responsibility | The successor responsibility and the prior responsibility history remain effective-dated and reconstructable against the physical resource. | Infrastructure Management |
| Preserve immutable business evidence | Canonical completed business facts are converted into immutable audit evidence without A07 taking ownership of the Infrastructure responsibility fact. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The successor responsible organizational or operational party is effective against the applicable physical Infrastructure resource.
- The prior Infrastructure responsibility remains historically traceable.
- B02/E02 ownership of organizational and workforce assignment facts remains unchanged.

## 16.2 Postconditions

- Authorized consumers may rely on the current G10 Infrastructure responsibility association.
- Historical responsibility remains available for governed reconstruction under G10 ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 requires a domain event for significant Infrastructure transitions but does not define an exact canonical event name for Infrastructure responsibility change/transfer. | L3 cannot state the required canonical event contract without inventing an event name. The business outcome and publisher ownership are otherwise clear. | Upstream L2-G10 / enterprise event governance must define the exact canonical event name if publication is required for this transition. |
| L2-G10 defines responsibility types, effective dating and temporary delegation behaviour but does not provide a separate detailed workflow for permanent responsibility transfer. | The frozen transfer outcome can be expressed without inventing states, but additional step-level governance cannot be asserted at L3. | Upstream L2-G10 may define a dedicated transfer workflow if more detailed authoritative behaviour is required for later approval. |

**Architecture assessment:** `PASS WITH ISSUES`

The Draft is source-grounded and preserves canonical ownership, but the missing exact event name and dedicated transfer workflow remain non-silent upstream gaps.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
