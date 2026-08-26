# Building Registration to Physical Building Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-282  
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
**Referenced By:** Floor, Space and downstream physical-context scenarios

---

# 2. Scenario Purpose

Register one canonical G10 Building beneath exactly one valid Physical Campus while preserving immutable identity, parent-child integrity, audit history and the separation between establishment and later operational lifecycle transitions.

---

# 3. Business Outcome

A canonical Building is established under the applicable Physical Campus.

The Building registration workflow is authoritative, but locked L2-G10 defines two different canonical event names for the same required completed registration fact. Because successful registration must publish its canonical event, this Draft is **BLOCKED** until the event conflict is resolved upstream.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid building must be represented within a Physical Campus.

## 4.2 End Boundary

The scenario ends when the Building exists canonically under one valid Physical Campus, satisfies all Building registration invariants, preserves its immutable identity and registration evidence, and the exact canonical Building registration event has been published unambiguously.

## 4.3 In Scope

- Resolution and validation of the parent Physical Campus.
- Validation of Building code, physical characteristics and required metadata.
- Creation of the G10 Building aggregate and immutable identity.
- Establishment of exactly one parent Physical Campus relationship.
- Registration audit evidence.
- Publication of the required canonical Building registration fact.

## 4.4 Out of Scope

- Building activation, later lifecycle transitions, floor registration or space registration.
- Academic, facilities, security or other consumer-domain operations inside the Building.
- Re-parenting an existing Building.
- Choosing between conflicting locked event names or initial lifecycle descriptions.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Prerequisite | L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment | Provides the parent Physical Campus. |
| Related | L3-G-283 — Floor Registration to Building Hierarchy Extension | Extends the Building hierarchy with Floors. |

---

# 5. Ownership and Participating Domains

Infrastructure Management is the sole frozen participating domain and owns the Building identity, Building physical characteristics, Building lifecycle and parent Physical Campus relationship. No other participating domain is introduced.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Infrastructure Administrator | Initiates Building registration and supplies the permitted Building information. | Must have Infrastructure Administration permission and comply with G10 registration rules. |
| Campus Administrator | Administers the parent-campus scope where applicable. | Does not bypass Building or parent-lifecycle validation. |
| Auditor | Reviews immutable Building registration history. | Read-only. |

These actors are explicitly present in the locked Building Registration workflow.

---

# 7. Preconditions and Entry Criteria

- The parent Physical Campus exists.
- The parent Physical Campus is Operational for the detailed G10 Building Registration workflow.
- The Building code is unique within the Physical Campus.
- The required Building information is available.
- Gross Area is greater than zero where supplied as required physical information.
- Construction Year is not in the future where applicable.
- The requester is authorized to register infrastructure within the parent-campus scope.

---

# 8. Trigger

A valid building must be represented within a Physical Campus.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the parent Physical Campus and verify its authoritative G10 identity. | Infrastructure Management | The Building has a canonical prospective parent. |
| 2 | Validate that the parent Campus satisfies the Building Registration preconditions. | Infrastructure Management | Registration proceeds only under a valid parent. |
| 3 | Validate Building code uniqueness and required Building information. | Infrastructure Management | Duplicate or invalid Building proposals are rejected. |
| 4 | Enforce Building invariants, including exactly one parent Campus and applicable area/year constraints. | Infrastructure Management | Building integrity is preserved. |
| 5 | Establish the Building aggregate and assign its immutable identity. | Infrastructure Management | A canonical Building exists beneath the Physical Campus. |
| 6 | Apply only the source-defined initial non-operational lifecycle position. | Infrastructure Management | **Lifecycle wording remains ambiguous upstream; no local state is selected.** |
| 7 | Preserve the Building registration evidence. | Infrastructure Management | The establishment is auditable. |
| 8 | Publish the canonical Building registration event. | Infrastructure Management | **Blocked:** L2-G10 conflicts between `BuildingRegistered` and `BuildingCreated`. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Parent Campus not Operational | The detailed Building workflow precondition is not met. | Reject or defer Building registration until the parent satisfies the authoritative precondition. | No invalid Building is established. |
| Duplicate Building code | The proposed code already exists within the Campus. | Reject the registration and require a valid unique code. | Existing canonical Building records remain unchanged. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Parent Physical Campus is missing or invalid. | The Building cannot exist canonically. | Establish or correct the parent under G10 before a later registration attempt. | Infrastructure Management |
| Building invariant fails. | The proposed Building is invalid. | Reject the registration and preserve existing infrastructure unchanged. | Infrastructure Management |
| Required registration event is ambiguous. | Mandatory event integrity cannot be preserved. | Do not choose or invent an event; require upstream architecture resolution. | Infrastructure Management / architecture governance |
| Initial lifecycle wording is contradictory. | Exact initial state cannot be asserted safely. | Record the Building as established but do not assign a locally inferred lifecycle state in L3. | Infrastructure Management / architecture governance |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Building belongs to exactly one Physical Campus and cannot exist independently. | Infrastructure Management | Parent relationship is mandatory and exclusive. | L2-G10 §§8.2, 13A.65; WF-INF-003 |
| Building Code is unique within Campus. | Infrastructure Management | Duplicate Building registration is rejected. | L2-G10 §8.2; Building Registration workflow |
| Gross Area must be greater than zero and Construction Year cannot be in the future. | Infrastructure Management | Invalid physical metadata prevents establishment. | L2-G10 §8.2 |
| Aggregate identity is immutable. | Infrastructure Management | The Building retains one canonical identity after creation. | L2-G10 §§8, 13A.64 |
| Every successful workflow publishes at least one canonical event. | Infrastructure Management | Building establishment cannot be canonically complete while the event name conflicts. | L2-G10 §22A.99 |

---

# 13. State and Lifecycle Impact

Building owns its lifecycle under G10. Registration establishes a Building but does not make it Operational automatically.

L2-G10 conflicts on the exact initial position: its registration summary says newly registered infrastructure starts in `Draft`, while the detailed Building workflow says initial lifecycle follows configured defaults and the canonical lifecycle begins with `Proposed`. This Draft does not choose among those locked alternatives.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-G10 §44 names the registration event `BuildingRegistered`. WF-INF-003, the workflow-event summary, §44A structural-event catalogue and detailed Building workflow use `BuildingCreated`. Since both describe the completed Building registration fact and successful workflows must publish an exact canonical event, L3 cannot select one locally.

## 14.2 Business Contracts and Cross-Domain Hand-offs

After canonical establishment, G10's Building Summary contract exposes Building identity, code, name, parent Campus, Building type, lifecycle and operational status to authorized consumers. Consumption does not transfer ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

The requester must possess the source-defined Infrastructure Administration permission within the relevant scope.

## 15.2 Approval and Separation of Duty

No universal Building-registration approver or separation-of-duty requirement is defined. Configured approval is applied only where authoritative policy requires it.

## 15.3 Business Audit and Evidence Obligations

Registration evidence must preserve the parent Physical Campus reference, Building identity, Building code, validated metadata, actor, authorization result, approval reference where applicable, and completed registration fact.

---

# 16. Completion Criteria and Postconditions

The frozen outcome is complete only when:

- the parent Physical Campus is valid under the registration preconditions;
- the Building satisfies all registration invariants;
- one canonical Building identity is established beneath exactly one Physical Campus;
- registration evidence is preserved; and
- the exact canonical Building registration event is resolved upstream and published by G10.

The current locked source fails the final condition.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-282-BLOCKER-01 | Blocking | L2-G10 defines the mandatory Building registration event as `BuildingRegistered` in §44 but as `BuildingCreated` in WF-INF-003, workflow event summaries, §44A and the detailed Building workflow. | Architecture governance must define one canonical event name while retaining G10 as publisher. |
| G-282-ISSUE-02 | Non-blocking | L2-G10 conflicts on the initial registration lifecycle position (`Draft`, configured default, or the canonical lifecycle beginning at `Proposed`). | Align registration and lifecycle wording upstream before downstream specification depends on the exact initial state. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
