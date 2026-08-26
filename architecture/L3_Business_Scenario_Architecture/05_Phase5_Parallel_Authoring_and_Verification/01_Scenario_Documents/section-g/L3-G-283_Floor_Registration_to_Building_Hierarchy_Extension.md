# Floor Registration to Building Hierarchy Extension

# 1. Scenario Metadata

**Scenario ID:** L3-G-283  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows  
**Referenced By:** L3-G-281 — Infrastructure Hierarchy Creation to Canonical Physical Structure; L3-G-284 — Space Registration to Physical Space Establishment

---

# 2. Scenario Purpose

Establish a canonical Floor under exactly one valid Building while applying G10-owned hierarchy, uniqueness, lifecycle, authorization, event and audit requirements.

---

# 3. Business Outcome

A canonical Floor is created under the correct Building.

The locked L2 source conflicts on the canonical registration event name and on the applicable initial lifecycle model. Because both event identity and lifecycle integrity are mandatory L3 concerns, this Draft is **BLOCKED** pending upstream resolution.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid floor must be established within a Building.

## 4.2 End Boundary

The scenario ends when one canonical Floor has been established under the correct Building, its registration has satisfied G10 rules, its resulting lifecycle position is unambiguous, and the exact canonical Floor-registration event has been published by Infrastructure Management.

## 4.3 In Scope

- Floor registration within an existing Building.
- Parent Building and parent-lifecycle validation.
- Floor-number uniqueness and physical hierarchy integrity.
- Establishment of the Floor aggregate and its immutable identity.
- G10-owned registration evidence and required business-event publication.

## 4.4 Out of Scope

- Building creation or modification.
- Space creation beneath the Floor.
- Floor activation, closure, decommissioning or archival.
- Academic, examination, hostel, facilities or other consumer-domain use of the Floor.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-281 — Infrastructure Hierarchy Creation to Canonical Physical Structure | Places Floor registration inside the canonical Physical Campus → Building → Floor → Space structure. |
| Followed by | L3-G-284 — Space Registration to Physical Space Establishment | A valid Floor is the parent required for Space registration. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Sole owner of the Floor aggregate, its parent relationship, lifecycle, business rules, registration outcome and registration event. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Performs the source-defined Floor registration activity. | Initiates and performs Floor registration within authorized infrastructure scope. |

---

# 7. Preconditions and Entry Criteria

- The parent Building exists.
- The parent Building is Operational under the detailed Floor Registration Workflow.
- The Floor Number is unique within the Building.
- The applicable infrastructure registration and hierarchy rules are satisfied.
- The Infrastructure Administrator is authorized to perform the registration.

---

# 8. Trigger

A valid floor must be established within a Building.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the existing Building under which the Floor is to be established. | Infrastructure Administrator / Infrastructure Management | The intended parent Building is resolved. |
| 2 | Validate the parent Building and its lifecycle against the Floor-registration rules. | Infrastructure Management | The parent is eligible to contain the new Floor. |
| 3 | Validate the proposed Floor Number and Floor information against G10 rules. | Infrastructure Management | Duplicate or invalid Floor registration is prevented. |
| 4 | Establish the Floor under exactly one Building and preserve its G10-owned identity. | Infrastructure Management | The canonical Floor exists in the correct physical hierarchy. |
| 5 | Record the completed registration as required by G10 audit obligations. | Infrastructure Management | The Floor registration is auditable. |
| 6 | Publish the canonical Floor-registration event. | Infrastructure Management | **Blocked:** the locked L2 source defines contradictory event names for this same registration outcome. |

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Parent Building does not exist or fails the required parent-lifecycle condition. | Floor registration cannot proceed. | Correct the parent condition before a later authorized registration attempt. | Infrastructure Management |
| Floor Number is not unique within the Building. | The proposed Floor would violate the canonical hierarchy rules. | Provide a valid unique Floor Number before a later registration attempt. | Infrastructure Management |
| Authorization fails. | No Floor may be established through this scenario. | Obtain valid source-supported authorization before resubmission. | Infrastructure Management |
| The canonical event-name conflict remains unresolved. | The required completed business fact cannot be published canonically. | Resolve the conflicting locked event definition through architecture change control. | Infrastructure Management |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Floor belongs to exactly one Building. | Infrastructure Management | The Floor cannot exist outside, or under multiple Buildings in, the canonical hierarchy. | L2-G10 §8.3; BR-ST-03; FSM-01 |
| Floor Number must be unique within the Building. | Infrastructure Management | Duplicate Floor Numbers are rejected. | L2-G10 WF-INF-004; Floor Registration Workflow |
| Floor lifecycle cannot exceed the parent Building lifecycle. | Infrastructure Management | Parent lifecycle is validated before the Floor can progress. | L2-G10 §8.3; LR-03 |
| Every successful infrastructure workflow publishes a canonical business event. | Infrastructure Management | A contradictory registration event name prevents canonical completion. | L2-G10 WDP-02; §44 |

---

# 13. State and Lifecycle Impact

The locked L2 source is internally inconsistent. The general registration workflow states that newly registered infrastructure begins in `Draft`; the canonical lifecycle in §11A defines `Proposed → Planned → Approved → Under Construction → Operational...`; and §46 gives a different Floor lifecycle beginning `Draft → Approved → Operational...`.

No initial Floor state is selected locally.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Floor | No canonical Floor exists for the requested Floor Number under the Building | Establishes the new Floor under its parent | Exact initial lifecycle position unresolved upstream | Infrastructure Management | L2-G10 §§11A, 16, 46; Floor Registration Workflow |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

A canonical event is mandatory but its exact name is contradictory:

- `FloorRegistered` in L2-G10 §44.
- `FloorCreated` in WF-INF-004, the Floor Registration Workflow, §44A structural events and the Floor & Space Events Summary.

Infrastructure Management is the publisher in either case. L3 does not choose or rename the event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain hand-off is required to complete this G-only scenario. G10 exposes a Floor Summary contract for later authorized consumers, but consumer activity occurs after this scenario outcome.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register Floor | Infrastructure Administrator | Must satisfy the source-defined Infrastructure Registration authorization and Floor rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override requirement is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Floor registration is auditable. | Source-defined evidence that the Floor was created under the validated Building through an authorized registration. | Infrastructure Management |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A unique canonical Floor exists under exactly one correct Building.
- The applicable parent and hierarchy rules have been satisfied.
- The resulting Floor lifecycle position is defined consistently by locked L2.
- The exact canonical Floor-registration event is published by Infrastructure Management.

## 16.2 Postconditions

- The Floor remains owned by Infrastructure Management.
- Subsequent Space registration may reference the canonical Floor only according to G10 rules.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 defines the same Floor registration fact as `FloorRegistered` in §44 and `FloorCreated` in the detailed registration/event sections. | A mandatory event cannot retain one exact canonical name at L3. | Architecture governance must select the canonical event name while retaining Infrastructure Management as publisher. |
| L2-G10 conflicts on the Floor's initial lifecycle model (`Draft` in registration/§46 versus the §11A canonical lifecycle beginning `Proposed`). | L3 cannot state the resulting lifecycle position without silently changing locked architecture. | Align the locked registration and canonical lifecycle definitions. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
