# Space Registration to Physical Space Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-284  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows  
**Referenced By:** Infrastructure-consuming institutional workflows; L3-G-286 — Space Classification to Controlled Physical Classification; L3-G-287 — Space Capability Definition to Operational Capability Availability

---

# 2. Scenario Purpose

Establish the canonical G10-owned Space under the correct Floor, preserving physical hierarchy, unique identity, mandatory Space information and Infrastructure Management ownership.

---

# 3. Business Outcome

A canonical Space is created under the correct physical hierarchy.

The locked L2 source conflicts on the canonical Space-registration event name and on the applicable initial lifecycle model. This Draft is therefore **BLOCKED**.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid usable physical space must be represented.

## 4.2 End Boundary

The scenario ends when a canonical Space has been established under the correct Floor with required Space information, a consistent source-defined lifecycle position, auditable registration evidence, and an unambiguous canonical registration event.

## 4.3 In Scope

- Space registration under an existing Floor.
- Parent Floor and hierarchy validation.
- Space identity, Space Code, Space Type, capacity and area validation required for registration.
- Creation of the canonical Space aggregate under G10.
- Required registration evidence and business-event publication.

## 4.4 Out of Scope

- Floor or Building creation.
- Subsequent Space reclassification or capability change.
- Capacity, availability or lifecycle changes after registration.
- Consumer-domain scheduling, allocation or operational decisions.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-283 — Floor Registration to Building Hierarchy Extension | The Space requires a valid parent Floor. |
| Related | L3-G-286 — Space Classification to Controlled Physical Classification | Governs a separate classification outcome after the Space exists where applicable. |
| Related | L3-G-287 — Space Capability Definition to Operational Capability Availability | Governs operational capability attributes of an existing Space. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Sole owner of the Space aggregate, its physical hierarchy relationship, identity, classification facts, capacity facts, lifecycle and registration outcome. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Performs the source-defined Space registration. | Initiates and performs registration within authorized infrastructure scope. |

---

# 7. Preconditions and Entry Criteria

- The parent Floor exists.
- The parent Floor is Operational under the detailed Space Registration Workflow.
- The Space Code is unique within the parent Building.
- Space Name, Space Code, Space Type, Capacity, Area and parent Floor information are available as required by L2-G10.
- Capacity and Area satisfy applicable G10 rules.
- The Infrastructure Administrator is authorized to register the Space.

---

# 8. Trigger

A valid usable physical space must be represented.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the parent Floor and the physical Space to be represented. | Infrastructure Administrator / Infrastructure Management | The intended parent relationship is resolved. |
| 2 | Validate the parent Floor and its required operational condition. | Infrastructure Management | The parent is eligible for Space registration. |
| 3 | Validate Space identity information, Space Type, capacity and area against G10 rules. | Infrastructure Management | Invalid or incomplete Space registration is prevented. |
| 4 | Establish the Space under exactly one Floor with its canonical identity. | Infrastructure Management | The canonical Space exists in the correct physical hierarchy. |
| 5 | Record the completed Space registration under G10 audit obligations. | Infrastructure Management | The registration is auditable. |
| 6 | Publish the canonical Space-registration event. | Infrastructure Management | **Blocked:** locked L2 defines contradictory canonical event names. |

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Parent Floor is absent or not in the required operational condition. | Space registration cannot proceed. | Correct the parent condition before a later registration attempt. | Infrastructure Management |
| Space Code is not unique within the Building. | Canonical identity rules would be violated. | Supply a valid unique Space Code before resubmission. | Infrastructure Management |
| Mandatory Space information, capacity or area is invalid. | A valid Space cannot be established. | Correct the business information before resubmission. | Infrastructure Management |
| Authorization fails. | No Space is established. | Obtain source-supported authorization before resubmission. | Infrastructure Management |
| Canonical registration event remains contradictory. | The mandatory completion fact cannot be published canonically. | Resolve the locked event-name conflict through architecture change control. | Infrastructure Management |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Space belongs to exactly one Floor. | Infrastructure Management | Space registration must establish one valid parent Floor. | L2-G10 §8.4; BR-ST-04; FSM-02 |
| Space Code is unique within the parent Building. | Infrastructure Management | Duplicate Space Codes are rejected. | L2-G10 Space Registration Workflow; FSM-03 |
| Capacity cannot be negative and Area must be positive for the detailed registration workflow. | Infrastructure Management | Invalid physical values prevent registration. | L2-G10 WF-INF-005; BR-CP-01; BR-AR-01 |
| Every successful infrastructure workflow publishes a canonical business event. | Infrastructure Management | The contradictory Space-registration event name blocks canonical completion. | L2-G10 WDP-02; §44 |

---

# 13. State and Lifecycle Impact

L2-G10 conflicts on the Space lifecycle. The generic registration workflow states `Draft`; §46 states `Draft → Approved → Operational → Reserved → Available → Closed → Archived`; the canonical §11A lifecycle begins `Proposed`; and the dedicated Space lifecycle omits `Under Construction` while otherwise following the canonical model.

No initial Space lifecycle state is selected locally.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Space | No canonical Space exists for the requested physical location | Establishes the Space under the parent Floor | Exact initial lifecycle position unresolved upstream | Infrastructure Management | L2-G10 §§11A, 16, 46; Space Registration Workflow |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

A canonical registration event is mandatory but contradictory:

- `SpaceRegistered` in L2-G10 §44.
- `SpaceCreated` in WF-INF-005, the detailed Space Registration Workflow, §44A structural events and the Floor & Space Events Summary.

Infrastructure Management remains the sole publisher.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain hand-off is required to complete this G-only registration scenario. The G10 Space Summary contract is available for later authorized consumers after canonical establishment.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register Space | Infrastructure Administrator | Must satisfy Infrastructure Registration authorization and Space business rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override requirement is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Space registration is auditable. | Source-defined evidence that the Space was established under the validated Floor through an authorized registration. | Infrastructure Management |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A canonical Space exists under exactly one correct Floor.
- Required Space identity, Space Type, capacity and area rules have been satisfied.
- The resulting lifecycle position is defined consistently by locked L2.
- The exact canonical Space-registration event is published by Infrastructure Management.

## 16.2 Postconditions

- The Space remains the G10-owned canonical physical unit for later authorized consumers.
- Subsequent classification, capability, capacity, availability and reservation scenarios do not transfer Space ownership.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 names the same registration fact `SpaceRegistered` in §44 and `SpaceCreated` in the detailed registration/event sections. | A mandatory business event cannot retain one exact canonical name. | Architecture governance must resolve the canonical event name while retaining G10 as publisher. |
| L2-G10 defines incompatible Space lifecycle models and initial positions across §§11A, 16, 46 and the Space lifecycle section. | L3 cannot state the new Space's canonical lifecycle position without choosing between locked definitions. | Align the authoritative Space lifecycle and registration state. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
