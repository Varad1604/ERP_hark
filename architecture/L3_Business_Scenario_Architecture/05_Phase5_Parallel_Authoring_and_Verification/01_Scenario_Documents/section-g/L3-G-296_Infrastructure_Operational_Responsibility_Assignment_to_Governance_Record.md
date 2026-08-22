# Infrastructure Operational Responsibility Assignment to Governance Record

# 1. Scenario Metadata

**Scenario ID:** L3-G-296  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, E, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B02 — Organisation Workflows; L2-E02 — Position & Assignment Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized Infrastructure consumers requiring current operational-responsibility context

# 2. Scenario Purpose

Establish the governed Infrastructure-specific record of which organizational or operational party holds a supported responsibility for a physical Infrastructure resource, while preserving Section B ownership of organizational structure, Section E ownership of workforce Position/Responsibility Assignment, Section A ownership of authorization and audit capabilities, and Section G ownership of the physical resource and its Infrastructure-specific responsibility association.

# 3. Business Outcome

The responsible organisational or operational party is recorded against the infrastructure resource

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when responsibility for an infrastructure resource must be formally assigned.

## 4.2 End Boundary

The scenario ends when the source-supported responsible organizational or operational party has been recorded against the applicable physical Infrastructure resource under G10 governance, with upstream organizational/workforce ownership preserved.

## 4.3 In Scope

- Identification of the physical Infrastructure resource whose operational responsibility requires assignment.
- Reference to authoritative organizational context from Section B where organizational placement is material.
- Reference to authoritative workforce/assignment context from Section E where a workforce participant is assigned.
- Selection of a G10-supported Infrastructure responsibility type.
- Effective-dated recording of the Infrastructure-specific responsible party against the physical resource.
- Preservation of canonical ownership boundaries and business auditability.

## 4.4 Out of Scope

- Creation or modification of Organization, Organization Unit, Position Definition, Workforce Engagement, Position Assignment, or E02 Responsibility Assignment.
- Creation of authorization roles or assignment of authorization-layer authority.
- Transfer of physical Infrastructure ownership away from G10.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-G-297 — Infrastructure Responsibility Change to Effective Responsibility Transfer | Governs a later change to the responsible party while preserving history. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical resource and the Infrastructure-specific record of supported operational responsibility against that resource. |
| Institution & Structure / Section B | Owns organization structure and Position Definition facts referenced by the assignment; those facts are not recreated by G10. |
| People & Workforce / Section E | Owns workforce Position Assignment, Responsibility Assignment, occupancy and organizational authority used as authoritative workforce context; those lifecycles remain in E02. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence; it does not own the Infrastructure responsibility outcome. |

The phrase “Infrastructure operational responsibility assignment” in this scenario means the G10 record of a responsible party against a physical resource. It does not re-own the canonical E02 `Responsibility Assignment` aggregate.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the Infrastructure-specific responsibility record. | Performs governed Infrastructure management within authorized scope. |
| Institution Administrator | Provides institution-wide Infrastructure oversight where the assignment falls within that governance scope. | Participates in Infrastructure oversight without taking ownership of the physical resource. |
| Infrastructure Custodian | A source-supported G10 responsibility that may be recorded against Infrastructure. | Holds administrative oversight responsibility where that responsibility type is selected. |
| Operational Manager | A source-supported G10 responsibility that may be recorded against Infrastructure. | Holds day-to-day operational management responsibility where that responsibility type is selected. |
| Auditor | Reviews preserved business evidence. | Read-only historical review. |

L2-G10 also supports Administrative Owner, Maintenance Coordinator, Compliance Officer and Safety Officer responsibilities. This scenario does not require one fixed responsibility type; it records whichever source-supported responsibility is applicable to the business case.

# 7. Preconditions and Entry Criteria

- The physical Infrastructure resource exists under G10 ownership.
- The requested Infrastructure responsibility type is one supported by L2-G10.
- Where an organizational party is referenced, its organizational identity/context is authoritative under Section B.
- Where a workforce participant is referenced, the relevant workforce and assignment context is authoritative under Section E.
- The action is authorized before the Infrastructure-specific responsibility record is established.

# 8. Trigger

Responsibility for an infrastructure resource must be formally assigned

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the physical Infrastructure resource and the G10-supported responsibility that requires formal assignment. | Infrastructure Management | The responsibility requirement is bounded to a canonical physical resource and supported responsibility type. |
| 2 | Obtain the authoritative organizational context from Section B and, where applicable, workforce assignment context from Section E without recreating either source fact. | Institution & Structure; People & Workforce; Infrastructure Management | The proposed responsible party is anchored to authoritative upstream organizational/workforce context. |
| 3 | Confirm that the proposed Infrastructure responsibility assignment is authorized and determine the applicable effective dating required for historical traceability. | Infrastructure Management, using the canonical authorization decision owned by Section A | The assignment is eligible to become effective under G10 governance. |
| 4 | Record the responsible organizational or operational party against the Infrastructure resource. | Infrastructure Management | The frozen Business Outcome is achieved while B02/E02 ownership remains unchanged. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Temporary delegation of an Infrastructure responsibility | A source-supported G10 responsibility is delegated for a limited period. | Apply the L2-G10 delegation requirements for effective dates, delegation reason, automatic restoration and complete history without redefining E02 Assignment Delegation or A04 Authorization Delegation. | The Infrastructure responsibility remains governed and historically traceable for the delegated period. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authoritative organization or workforce reference is not valid | G10 cannot establish a reliable responsibility association to the proposed party. | Correct the upstream B02/E02 business fact before a later Infrastructure responsibility assignment; G10 does not create a substitute organizational/workforce fact. | Institution & Structure or People & Workforce; Infrastructure Management retains the physical resource. |
| Authorization does not permit the assignment | The Infrastructure responsibility record is not established. | The action may proceed only through a later authorized business decision; no G10 responsibility outcome is asserted meanwhile. | Platform Foundation — Authorization; Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 may record supported operational responsibilities including Infrastructure Custodian, Operational Manager, Administrative Owner, Maintenance Coordinator, Compliance Officer and Safety Officer. | Infrastructure Management | The scenario may use only source-supported Infrastructure responsibility types. | L2-G10 §28 Operational Responsibility Management |
| Responsibility assignments support effective dating where historical traceability is required. | Infrastructure Management | The current responsible party can be established without erasing prior responsibility history. | L2-G10 Effective Dating principle; §28 |
| Section B is authoritative for organizational structure and Position Definition. | Institution & Structure | G10 references organizational facts but does not create or change them. | L2-B02 Purpose, Ownership and approved Position Definition ownership clarification |
| E02 canonically owns Position Assignment and Responsibility Assignment. | People & Workforce | G10 must not reinterpret the Infrastructure-specific responsible-party record as ownership of the E02 Responsibility Assignment aggregate. | L2-E02 §§1.3–1.4, 2.5 |
| Authorization decisions are owned by A04 and business domains determine when authorization is required. | Platform Foundation — Authorization; Infrastructure Management | G10 applies the authorization outcome without redefining A04 authorization concepts. | L2-A04 §§3, 5 and AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure / Space | Existing source-defined lifecycle position. | Gains or updates the G10-owned Infrastructure-specific responsible-party association. | Physical Infrastructure lifecycle itself remains unchanged by this assignment. | Infrastructure Management | L2-G10 §28; Space aggregate responsibilities |
| E02 Position Assignment / Responsibility Assignment | Existing authoritative E02 position/responsibility context where applicable. | Referenced for workforce/authority context only. | No E02 lifecycle transition is performed by this G10 scenario. | People & Workforce | L2-E02 ownership and scope |
| B02 organization / Position Definition | Existing authoritative organization/structural context. | Referenced to identify the organizational party/structure. | No B02 lifecycle transition is performed by this G10 scenario. | Institution & Structure | L2-B02 canonical ownership |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-G10 requires every significant Infrastructure business transition to generate a domain event, but no exact canonical event name for Infrastructure operational-responsibility assignment is defined in its event catalogue. This Draft therefore does not invent or rename an event. The gap is recorded in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institution & Structure | Infrastructure Management | Authoritative organization / Position Definition reference | Supplies organizational structure without transferring B02 ownership. | L2-B02; L2-G10 §9 Dependencies |
| People & Workforce | Infrastructure Management | Authoritative workforce / Position Assignment / Responsibility Assignment context | Supplies workforce organizational-authority context used to identify the responsible party without transferring E02 lifecycle ownership. | L2-E02; L2-G10 §9 Dependencies |
| Platform Foundation — Authorization | Infrastructure Management | Authorization decision | Determines whether the Infrastructure responsibility action may proceed without transferring the business decision to A04. | L2-A04 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish an Infrastructure-specific responsibility record | Infrastructure Administrator | Must act within the applicable Infrastructure scope and against valid B02/E02 references. | Infrastructure Management |

L2-G10 does not designate one universal named approver for all operational-responsibility assignments. No such approver is invented here.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or mandatory approval requirement for ordinary Infrastructure responsibility assignment is defined by the authoritative L2 architecture. Temporary G10 delegation follows only the source-defined effective-date, reason, automatic-restoration and historical-traceability requirements.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the effective Infrastructure responsibility record | The physical resource, supported responsibility, responsible party reference and applicable effective history remain traceable as the G10 business fact. | Infrastructure Management |
| Preserve immutable business evidence | Canonical completed business facts are converted into immutable audit evidence without A07 taking ownership of the underlying Infrastructure fact. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A source-supported responsible organizational or operational party is recorded against the applicable physical Infrastructure resource.
- The organizational/workforce references remain owned by B02/E02 rather than duplicated by G10.
- The Infrastructure-specific responsibility record has the applicable effective dating required for historical traceability.

## 16.2 Postconditions

- Authorized Infrastructure consumers may rely on the current G10 responsibility association for the physical resource.
- B02 organizational facts and E02 workforce assignment facts remain unchanged in ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 requires a domain event for significant Infrastructure transitions but does not define an exact canonical event name for operational-responsibility assignment. | L3 cannot populate a canonical event contract without inventing an event name. The business outcome and publisher ownership are otherwise clear. | Upstream L2-G10 / enterprise event governance must define the exact canonical event name if publication is required for this transition. |

**Architecture assessment:** `PASS WITH ISSUES`

The Draft preserves the frozen owner and outcome and keeps B02/E02/A04 ownership intact. The missing exact G10 event name is recorded rather than synthesized.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
