# Infrastructure Operational Responsibility Assignment to Governance Record

# 1. Scenario Metadata

**Scenario ID:** L3-G-296  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, E, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B02 — Organisation Workflows; L2-E02 — Position & Assignment Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Record a source-supported responsible organizational or operational party against a physical infrastructure resource while preserving B02 organizational ownership, E02 workforce assignment/responsibility ownership, A04 authorization ownership, and G10 physical-infrastructure ownership.

# 3. Business Outcome

The responsible organisational or operational party is recorded against the infrastructure resource.

G10 requires significant successful infrastructure workflows to publish canonical business events but defines no exact event name for this responsibility-assignment outcome. The Draft is **BLOCKED**.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when responsibility for an infrastructure resource must be formally assigned.

## 4.2 End Boundary

The scenario ends when a source-supported G10 responsible-party association is effective against the physical resource, upstream B/E ownership is preserved, and the required canonical assignment event can be published.

## 4.3 In Scope

- Selection of a G10-supported infrastructure responsibility.
- Reference to authoritative organization/Position Definition facts from B02.
- Reference to authoritative workforce Position/Responsibility Assignment context from E02 where applicable.
- Effective-dated G10 responsible-party association.
- Auditability and required event publication.

## 4.4 Out of Scope

- Creation or change of Organization, Organization Unit or Position Definition.
- Creation or change of E02 Position Assignment or Responsibility Assignment.
- Authorization Role/Role Assignment lifecycle changes.
- Transfer of physical-infrastructure ownership.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns the physical resource and its infrastructure-specific responsible-party association. |
| Institution & Structure / B | Owns Organization/Organization Unit and Position Definition facts referenced by G10. |
| People & Workforce / E | Owns Position Assignment, Responsibility Assignment and organizational authority facts used as workforce context. |
| Platform Foundation / A | Owns authorization decisions, event governance and audit evidence without owning the infrastructure responsibility outcome. |

The G10 responsible-party association is not the E02 `Responsibility Assignment` aggregate.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the infrastructure-specific responsibility association. | Performs the G10 action only where authorized and upstream references are valid. |

G10 supports responsibility types including Infrastructure Custodian, Operational Manager, Administrative Owner, Maintenance Coordinator, Compliance Officer and Safety Officer. No universal named approver is defined.

# 7. Preconditions and Entry Criteria

- The physical resource exists under G10 ownership.
- The requested responsibility type is source-supported by G10.
- Any referenced organization/Position Definition fact is authoritative under B02.
- Any referenced workforce assignment/responsibility context is authoritative under E02.
- The G10 assignment action is authorized.

# 8. Trigger

Responsibility for an infrastructure resource must be formally assigned.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the physical resource and the G10-supported responsibility to assign. | Infrastructure Administrator / Infrastructure Management | The requirement is bounded to canonical infrastructure and a source-supported responsibility type. |
| 2 | Resolve authoritative organizational and workforce references from B02/E02 where applicable. | Institution & Structure / People & Workforce → Infrastructure Management | G10 references rather than recreates upstream business facts. |
| 3 | Validate authorization and any effective dating required for historical traceability. | Infrastructure Management using Platform Authorization | The G10 responsibility association is eligible to become effective. |
| 4 | Record the responsible organizational or operational party against the physical resource. | Infrastructure Management | The frozen business outcome is achieved. |
| 5 | Publish the canonical responsibility-assignment business event required by G10. | Infrastructure Management | **Blocked:** no exact canonical event name is defined upstream. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Temporary G10 responsibility delegation | A supported infrastructure responsibility is delegated for a limited period. | Apply G10 effective dates, delegation reason, automatic restoration and complete history without redefining E02 Assignment Delegation or A04 Authorization Delegation. | The infrastructure responsibility remains governed and traceable. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Organization or workforce reference is not authoritative. | G10 cannot establish a reliable responsible-party association. | Correct the upstream B02/E02 fact before a later G10 assignment. | Institution & Structure / People & Workforce |
| Authorization fails. | No G10 responsibility association is established. | Proceed only after a later valid authorization decision. | Platform Foundation / Infrastructure Management |
| Canonical assignment event remains undefined. | Required event publication cannot be completed. | Define the event through upstream architecture governance. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 supports Infrastructure Custodian, Operational Manager, Administrative Owner, Maintenance Coordinator, Compliance Officer and Safety Officer responsibilities. | Infrastructure Management | Only source-supported infrastructure responsibilities may be recorded. | L2-G10 §28 |
| G10 responsibility assignments may be effective-dated where historical traceability is required. | Infrastructure Management | Current responsibility can be established without erasing history. | L2-G10 Effective Dating principle; §28 |
| B02 owns organization structure and Position Definition. | Institution & Structure | G10 references these facts and cannot recreate or change them. | L2-B02 Purpose/Scope and Position Definition ruling |
| E02 owns Position Assignment and Responsibility Assignment. | People & Workforce | G10's responsible-party association cannot re-own the E02 aggregates. | L2-E02 §§1.3–1.4, 2.5 |
| Authorization decisions remain owned by A04. | Platform Foundation — Authorization | G10 determines when authorization is needed but does not own A04 concepts. | L2-A04 §§3–6 |
| Significant G10 workflows require canonical business-event publication. | Infrastructure Management | Missing event name prevents canonical completion. | L2-G10 design principles / administrative-operation rule |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical infrastructure | Existing source-defined lifecycle position | Gains the G10 responsible-party association | Infrastructure lifecycle unchanged | Infrastructure Management | L2-G10 §28 |
| B02 organization / Position Definition | Existing authoritative facts | Referenced only | No B02 transition | Institution & Structure | L2-B02 |
| E02 Position Assignment / Responsibility Assignment | Existing authoritative facts where applicable | Referenced only | No E02 transition | People & Workforce | L2-E02 |

G10 does not define a separate named lifecycle for its infrastructure-specific responsible-party association.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 requires significant infrastructure workflows/administrative changes to publish a canonical business event, but no exact event name for operational-responsibility assignment is defined. Infrastructure Management is the business publisher; L3 does not invent the missing event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institution & Structure | Infrastructure Management | Authoritative organization / Position Definition reference | Supplies structural context without transferring B02 ownership. | L2-B02; L2-G10 dependencies |
| People & Workforce | Infrastructure Management | Authoritative workforce Position/Responsibility Assignment context | Supplies workforce authority context without transferring E02 ownership. | L2-E02; L2-G10 dependencies |
| Platform Foundation — Authorization | Infrastructure Management | Authorization decision | Determines whether the G10 action may proceed. | L2-A04 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish infrastructure-specific responsible-party association | Infrastructure Administrator | Must act within G10 scope using valid B02/E02 references and a valid authorization decision. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal separation-of-duty or mandatory approval rule is defined for ordinary G10 responsibility assignment. Temporary G10 delegation follows only the source-defined effective-date, reason, restoration and history requirements.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve infrastructure responsibility history | Resource, source-supported responsibility, responsible-party reference and effective history remain traceable. | Infrastructure Management |
| Preserve immutable audit evidence | Audit evidence derived from the completed canonical business fact. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A source-supported responsible organizational or operational party is recorded against the physical resource.
- B02/E02/A04 ownership remains unchanged.
- Applicable effective history is preserved.
- The exact canonical G10 assignment event is defined and published.

## 16.2 Postconditions

- Authorized infrastructure consumers may rely on the current G10 responsible-party association.
- Upstream organization/workforce facts remain under B02/E02 ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 requires business-event publication for significant infrastructure changes but defines no exact canonical event for operational-responsibility assignment. | L3 cannot populate the mandatory event contract without invention. | Upstream G10/event governance must define the exact event and retain Infrastructure Management as publisher. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
