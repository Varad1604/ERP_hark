# Infrastructure Hierarchy Creation to Canonical Physical Structure

# 1. Scenario Metadata

**Scenario ID:** L3-G-281  
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
**Referenced By:** Infrastructure registration and physical-context consumption scenarios

---

# 2. Scenario Purpose

Establish the canonical G10 physical hierarchy from Physical Campus through Building and Floor to Space while preserving each aggregate's independent ownership, validation and registration workflow.

---

# 3. Business Outcome

Physical Campus, Building, Floor, and Space relationships are established under G10 ownership.

The structural rules are clear, but the locked G10 source gives contradictory mandatory event names for the registration of the hierarchy members. Because every successful registration workflow must publish its canonical event, the complete hierarchy outcome cannot be specified without choosing between locked alternatives. This Draft is **BLOCKED**.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid physical infrastructure hierarchy must be established.

## 4.2 End Boundary

The scenario ends when all required Physical Campus, Building, Floor and Space identities and parent-child relationships exist under G10, all structural invariants hold, each aggregate registration has completed under its own workflow, and every required canonical registration event is unambiguous and published.

## 4.3 In Scope

- Validation of the intended Campus → Building → Floor → Space structure.
- Registration or reuse of the required G10 aggregates.
- Parent existence and parent-lifecycle validation.
- Unique codes and non-circular physical relationships.
- Preservation of independent aggregate boundaries.
- Registration audit history and aggregate-specific completed events.

## 4.4 Out of Scope

- InstitutionalCampus governance owned by B03.
- Treating hierarchy creation as one atomic aggregate operation.
- Inventing a `HierarchyCreated` event or a cross-aggregate lifecycle.
- Operational scheduling, reservations, maintenance or consumer-domain activity.
- Choosing locally between conflicting locked registration event names or lifecycle defaults.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment | Establishes the Physical Campus root. |
| Related | L3-G-282 — Building Registration to Physical Building Establishment | Establishes Building under Physical Campus. |
| Related | L3-G-283 — Floor Registration to Building Hierarchy Extension | Establishes Floor under Building. |
| Related | L3-G-284 — Space Registration to Physical Space Establishment | Establishes Space under Floor. |

---

# 5. Ownership and Participating Domains

Infrastructure Management is the sole frozen participating domain and owns every physical aggregate and parent-child relationship in this scenario. No participation outside `G` is introduced.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Infrastructure Administrator | Initiates permitted infrastructure registration and hierarchy establishment. | Must remain within authorized infrastructure scope and source-defined rules. |
| Campus Administrator / Building Administrator | Administers permitted infrastructure within assigned physical scope. | Cannot bypass parent-child, uniqueness or lifecycle rules. |
| Auditor | Reviews immutable registration and hierarchy evidence. | Read-only. |

---

# 7. Preconditions and Entry Criteria

- The owning institutional context is valid.
- Every existing parent aggregate referenced in the intended hierarchy is canonical and valid.
- Proposed Building, Floor and Space codes satisfy their source-defined uniqueness constraints.
- The intended hierarchy is acyclic.
- The requester is authorized for each aggregate registration that must occur.

---

# 8. Trigger

A valid physical infrastructure hierarchy must be established.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Define the intended Physical Campus → Building → Floor → Space structure. | Infrastructure Management | The requested hierarchy scope is explicit. |
| 2 | Validate parent existence, parent lifecycle, unique codes and absence of circular relationships. | Infrastructure Management | The hierarchy is structurally admissible or rejected. |
| 3 | Register or resolve the Physical Campus through its own G10 aggregate workflow. | Infrastructure Management | The Physical Campus identity is canonical. |
| 4 | Register each required Building under exactly one Physical Campus through the Building workflow. | Infrastructure Management | Building parentage is canonical. |
| 5 | Register each required Floor under exactly one Building through the Floor workflow. | Infrastructure Management | Floor parentage is canonical. |
| 6 | Register each required Space under exactly one Floor through the Space workflow. | Infrastructure Management | Space parentage is canonical. |
| 7 | Validate the resulting complete hierarchy against G10 structural invariants. | Infrastructure Management | Physical relationships are established without duplication or cycles. |
| 8 | Publish each required aggregate-specific registration event and preserve audit evidence. | Infrastructure Management | **Blocked:** locked G10 defines conflicting canonical registration event names for the hierarchy members. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Existing parent | One or more required parent aggregates already exist. | Reuse the canonical parent and register only missing children. | No duplicate aggregate is created. |
| Optional Wing | A Building uses a Wing. | Preserve Wing as the optional Building child permitted by G10 without creating a new hierarchy level outside the canonical model. | Campus → Building → optional Wing → Floor → Space remains valid. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Parent does not exist or does not satisfy lifecycle requirements. | The child cannot be established canonically. | Correct or establish the parent under its own G10 workflow before proceeding. | Infrastructure Management |
| Duplicate code or circular relationship is detected. | Structural integrity would be violated. | Reject the affected registration and preserve existing canonical structure. | Infrastructure Management |
| One member registration remains incomplete. | The intended full hierarchy outcome is incomplete. | Preserve successfully existing independent aggregates; do not claim the full hierarchy outcome until all required relationships are valid. | Infrastructure Management |
| Registration event name is contradictory upstream. | Mandatory event integrity cannot be maintained. | Do not choose or invent an event; resolve through architecture change control. | Infrastructure Management |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical hierarchy is Physical Campus → Building → optional Wing → Floor → Space. | Infrastructure Management | Arbitrary hierarchy levels or circular containment are prohibited. | L2-G10 §§8.10, 18 |
| Every Building belongs to exactly one Campus, every Floor to one Building, every Space to one Floor. | Infrastructure Management | Parent-child relationships are exclusive and validated. | L2-G10 §§8, 13, 13A.65 |
| Every workflow modifies exactly one Aggregate Root. | Infrastructure Management | This L3 outcome composes independent registration workflows; it does not create one cross-aggregate transaction or aggregate. | L2-G10 §22A.99 WDP-01 |
| Every successful workflow publishes at least one canonical domain event. | Infrastructure Management | Each created hierarchy member requires an exact canonical event. | L2-G10 §22A.99 WDP-02 |
| Aggregate identity is immutable. | Infrastructure Management | Existing parents are reused by identity and never recreated to satisfy hierarchy establishment. | L2-G10 §§8, 13A.64 |

---

# 13. State and Lifecycle Impact

Each Physical Campus, Building, Floor and Space owns an independent G10 lifecycle. Hierarchy establishment does not create a separate hierarchy lifecycle.

For newly registered members, G10 has an unresolved initial-state inconsistency (`Draft` in the registration summary versus configured defaults / `Proposed` or `Planned` in detailed workflow material). This Draft does not choose an initial state.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

For the same registration completions, L2-G10 §44 lists `CampusRegistered`, `BuildingRegistered`, `FloorRegistered`, and `SpaceRegistered`, while detailed WF-INF workflows and event summaries use `CampusCreated`, `BuildingCreated`, `FloorCreated`, and `SpaceCreated`. Since successful registration must publish canonical events, L3 cannot select either set locally.

No single `HierarchyCreated` event is defined or invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G10 exposes Physical Campus, Building, Floor and Space Summary contracts for the resulting physical facts. Consumers remain outside this scenario and cannot modify the hierarchy.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Every aggregate registration requires applicable Infrastructure Registration authorization.

## 15.2 Approval and Separation of Duty

Any configured approval is applied only where authoritative G10 policy requires it. No universal hierarchy approver is inferred.

## 15.3 Business Audit and Evidence Obligations

The hierarchy evidence must preserve each aggregate identity, parent reference, validation result, actor, business reason where applicable, and the source-defined registration outcome. Each individual registration remains auditable.

---

# 16. Completion Criteria and Postconditions

The frozen outcome is complete only when:

- the Physical Campus, Building, Floor and Space relationships satisfy the canonical G10 hierarchy;
- no duplicate or circular relationship exists;
- every newly required aggregate has completed its own registration workflow;
- every required registration fact has an unambiguous canonical event and publisher; and
- all individual aggregate histories remain auditable.

The current locked source fails the event-name condition, so the scenario remains blocked.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-281-BLOCKER-01 | Blocking | L2-G10 gives conflicting mandatory registration event names for Physical Campus, Building, Floor and Space (`*Registered` in §44 versus `*Created` in detailed WF-INF workflows/event summaries). | Architecture governance must resolve the canonical event names while preserving G10 as publisher. |
| G-281-ISSUE-02 | Non-blocking | No single hierarchy-completion event or cross-aggregate scenario contract is defined. | Downstream specifications must continue treating this as a composition of independently owned G10 workflows unless upstream defines a business-level completion contract. |
| G-281-ISSUE-03 | Non-blocking | Initial lifecycle position for newly registered infrastructure is inconsistent within G10. | Align registration and canonical lifecycle wording upstream. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
