# Infrastructure Capacity Definition to Capacity Profile Activation

# 1. Scenario Metadata

**Scenario ID:** L3-G-288  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows where institutional ownership is referenced; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized infrastructure consumers across academic, examination, facilities, safety, workforce, analytics and institutional operations

---

# 2. Scenario Purpose

Create or update the effective canonical Capacity Profile of a valid infrastructure aggregate under configuration, policy and lifecycle rules.

---

# 3. Business Outcome

A Capacity Profile becomes authoritative for the applicable infrastructure

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when capacity information is established for a physical infrastructure aggregate.

## 4.2 End Boundary

The scenario ends when the exact frozen outcome is established under G10 ownership, the current authoritative physical fact is available through governed contracts, required events have been published only after successful persistence, and immutable audit history is preserved.

## 4.3 In Scope

- Resolution of Tenant, institution, resource and physical hierarchy context.
- Scoped authorization before business validation.
- Validation of G10 identity, hierarchy, lifecycle, capacity, availability, configuration and relationship rules as applicable.
- The `Applicable Physical Campus, Building, Floor or Space` business responsibility.
- Preservation of immutable identifiers and prior historical values.
- Publication of only source-defined completed business facts.
- Governed read contracts for authorized consumers.
- Complete audit evidence.

## 4.4 Out of Scope

- Institutional identity or InstitutionalCampus ownership retained by B03.
- Academic scheduling, examination conduct, facilities-maintenance execution or consumer-specific decisions.
- Authentication, technical RBAC, database, API, UI, queue or implementation design.
- Physical deletion of infrastructure or historical records.
- Inventing lifecycle states, commands, events, publishers, approval actors or cross-domain contracts absent from L2.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment | Establishes the canonical Physical Campus root. |
| Related | L3-G-281 — Infrastructure Hierarchy Creation to Canonical Physical Structure | Governs the Campus–Building–Floor–Space hierarchy. |
| Related | L3-G-289 — Infrastructure Availability Definition to Availability Profile Activation | Establishes authoritative resource availability. |
| Related | L3-G-291 — Infrastructure Reservation Request to Controlled Reservation | Consumes availability and capacity without owning either fact. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Primary owner of physical identity, hierarchy, lifecycle, capacity, availability, classification, relationships and infrastructure master facts. |
| Institution & Structure / Section B, where frozen | Owns Institution and InstitutionalCampus identity; supplies references without owning physical aggregates. |
| Platform Foundation / Section A, where frozen | Supplies Tenant isolation, configuration, authorization, event and audit capabilities without owning infrastructure facts. |
| Academic Delivery / Section D, where frozen | Consumes authoritative capacity, availability or reservation outcomes and retains its academic decisions. |

The frozen participating-domain value remains exactly `G, A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Infrastructure Administrator | Initiates and administers permitted infrastructure operations. | Must hold fine-grained permission in the relevant Tenant, institution, Campus, Building or Zone scope. |
| Campus or Building Administrator | Manages infrastructure within assigned physical scope. | Cannot act outside assigned hierarchy scope. |
| Facilities Manager / Operational Manager | Supplies operational context and consumes current physical facts. | Does not change canonical identity or bypass G10 rules. |
| Auditor | Reviews immutable history and evidence. | Read-only; cannot mutate infrastructure. |
| Consumer-domain actor | Requests or consumes an authorized infrastructure service. | Retains the consumer decision and cannot modify G10 aggregates directly. |

Roles are configurable access patterns in L2-G10. No universal named approver is inferred.

---

# 7. Preconditions and Entry Criteria

- The infrastructure aggregate exists and permits capacity modification.
- The Capacity Type is configured and the value is non-negative.
- Effective-date and temporary-override information are supplied where applicable.
- The requester holds scoped Update Capacity permission.
- Tenant and institution isolation are enforceable.
- Authentication context is supplied externally and authorization succeeds before mutation.
- Required audit and event infrastructure is available under enterprise policy.

---

# 8. Trigger

Capacity information is established for a physical infrastructure aggregate

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the infrastructure aggregate and current Capacity Profile. | Infrastructure Management | Resolve the infrastructure aggregate and current Capacity Profile. |
| 2 | Authorize capacity maintenance within the resource scope. | Infrastructure Management | Authorize capacity maintenance within the resource scope. |
| 3 | Validate Capacity Type, value, lifecycle and institutional policy. | Infrastructure Management | Validate Capacity Type, value, lifecycle and institutional policy. |
| 4 | Determine the effective date or bounded override period. | Infrastructure Management | Determine the effective date or bounded override period. |
| 5 | Persist the new profile while preserving prior capacity history. | Infrastructure Management | Persist the new profile while preserving prior capacity history. |
| 6 | Expose the current authoritative Capacity Profile to consumers. | Infrastructure Management | Expose the current authoritative Capacity Profile to consumers. |
| 7 | Publish the applicable completed capacity event after persistence. | Infrastructure Management | Publish the applicable completed capacity event after persistence. |
| 8 | Preserve previous value, new value, dates, reason, actor and correlation evidence. | Infrastructure Management | Preserve previous value, new value, dates, reason, actor and correlation evidence. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| 1 | A temporary override is approved | retain its effective and expiry dates and automatically restore configured capacity after expiry. | Existing ownership and history remain intact. |
| 2 | Several Capacity Types apply | maintain them independently under configuration. | Existing ownership and history remain intact. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant, institution or resource scope cannot be resolved | reject the request without creating a cross-scope fact. | Correct the source condition and submit a later authorized request; no partial success is recognized. | Infrastructure Management; applicable participant |
| Authorization fails | terminate before business mutation. | Correct the source condition and submit a later authorized request; no partial success is recognized. | Infrastructure Management; applicable participant |
| Hierarchy, lifecycle, configuration or policy validation fails | retain the prior authoritative state. | Correct the source condition and submit a later authorized request; no partial success is recognized. | Infrastructure Management; applicable participant |
| A uniqueness, capacity, availability or relationship invariant fails | reject the requested result. | Correct the source condition and submit a later authorized request; no partial success is recognized. | Infrastructure Management; applicable participant |
| Audit evidence cannot be recorded | do not complete the business transition. | Correct the source condition and submit a later authorized request; no partial success is recognized. | Infrastructure Management; applicable participant |
| The success-event condition is not satisfied | do not publish the success event. | Correct the source condition and submit a later authorized request; no partial success is recognized. | Infrastructure Management; applicable participant |

No technical retry, rollback, database transaction or messaging implementation is specified at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management is the canonical owner of physical infrastructure facts. | Infrastructure Management | Consumers reference G10 facts and never duplicate or directly modify aggregates. | L2-G10 §§2, 5, 9, 44A |
| Every Building belongs to one Campus; every Floor to one Building; every Space to one Floor. | Infrastructure Management | Invalid or circular physical relationships are rejected. | L2-G10 §§8, 18, 47 |
| Aggregate identity is immutable and infrastructure is never physically deleted. | Infrastructure Management | Changes preserve identifiers and historical references. | L2-G10 §§13, 47, 48A |
| Authorization precedes business validation. | Authorization & Policy; Infrastructure Management | Unauthorized requests terminate before mutation. | L2-G10 §48A.207 |
| Business validation precedes persistence, and events describe completed facts. | Infrastructure Management; Event Infrastructure | Failed work cannot publish a success event. | L2-G10 §§22A.99, 44A.179 |
| Every successful state or significant attribute change is auditable. | Infrastructure Management; Audit Infrastructure | Actor, prior value, new value, reason and correlation are preserved. | L2-G10 §§48A.208–210, 49 |
| Archived infrastructure is immutable. | Infrastructure Management | No scenario may update an Archived aggregate. | L2-G10 §§11A, 47 |
| Capacity cannot be negative and reservation conflicts are prevented unless an authorized override is permitted. | Infrastructure Management | Invalid capacity or conflicting reservations are rejected. | L2-G10 §§24, 26, 47 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry Condition | Scenario Impact | Resulting State / Position | Canonical Owner |
|---|---|---|---|---|
| `Applicable Physical Campus, Building, Floor or Space` | Valid trigger and authorized context | Executes only the scenario-specific creation, association or governed attribute change. | Infrastructure lifecycle remains unchanged; effective Capacity Profile becomes current | Infrastructure Management, except the B03 reference retained by Institution & Structure |
| Parent physical hierarchy | Existing and valid where applicable | Referenced and validated; not silently re-parented. | Unchanged unless the frozen outcome explicitly creates the child relationship. | Infrastructure Management |
| Tenant and Institution | Existing | Supply isolation and institutional reference. | Unchanged. | Platform Foundation; Institution & Structure |
| Consumer-domain records | Existing or absent | Consume the completed authoritative fact. | Unchanged by G10. | Applicable consumer domain |
| Audit history | Existing | Appends immutable evidence. | Historical trace grows; prior records remain unchanged. | Audit Infrastructure |

L2-G10 requires lifecycle and operational availability to remain distinct. “Activation” in a frozen outcome is not treated as a new lifecycle state unless L2-G10 defines it.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Event Treatment

| Event / Required Fact | Publication Condition | Publisher and Consumer Treatment |
|---|---|---|
| `RoomCapacityChanged for detailed workflow or CapacityUpdated in the summary catalogue` | Only after the corresponding aggregate creation or governed update has persisted successfully. | The affected G10 aggregate publishes the source-defined completed fact; consumers react without changing G10 ownership. |

Where Section 44 and the detailed WF-INF catalogue use different names, this Draft records the inconsistency in Section 17 and does not publish both names for one transition.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Physical Campus Summary | Infrastructure Management | Institution & Structure and authorized consumers | Canonical physical Campus identity, lifecycle and operational status. |
| Building, Floor and Space Summary | Infrastructure Management | Operational domains | Canonical hierarchy, classification, capacity, lifecycle and current status. |
| Capacity Profile | Infrastructure Management | Academic, Examination and operational consumers | Canonical capacity facts; consumers own utilization decisions. |
| Retrieve Availability | Infrastructure Management | Authorized consumers | Current operational availability independently from lifecycle. |
| Reservation service | Infrastructure Management | Authorized requestor | Governed reservation establishment, modification or retrieval. |
| Audit input | Infrastructure Management | Audit Infrastructure | Actor, scope, transition, prior/new values, reason and correlation evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Authentication is external to G10.
- Fine-grained permission is evaluated by Tenant, institution and physical-resource scope before validation.
- Consumer access does not grant direct aggregate mutation.
- Authorization success does not waive hierarchy, lifecycle, capacity, availability or policy rules.

## 15.2 Approval and Separation of Duty

- L2-G10 allows configurable approval for some creation, lifecycle, override and administrative operations.
- The scenario uses approval only where active policy requires it.
- No unnamed approver or technical workflow routing is invented.
- Governance approval supplements business validation and never replaces it.

## 15.3 Audit Obligations

The audit record must preserve Tenant, institution, aggregate type and identifier, actor, authorization scope, previous values, new values, effective dates where applicable, business reason, approval reference where required, event facts and correlation identifier. Audit history is immutable and independently retained.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when:

- resolve the infrastructure aggregate and current Capacity Profile.
- authorize capacity maintenance within the resource scope.
- validate Capacity Type, value, lifecycle and institutional policy.
- determine the effective date or bounded override period.
- persist the new profile while preserving prior capacity history.
- expose the current authoritative Capacity Profile to consumers.
- publish the applicable completed capacity event after persistence.
- preserve previous value, new value, dates, reason, actor and correlation evidence.
- no institutional, academic or consumer-domain ownership has transferred to G10;
- no unauthorized or partial result is treated as successful; and
- all required audit evidence is immutable and queryable.

Postconditions:

- Canonical G10 identity and historical integrity are preserved.
- The current authoritative fact is available through governed contracts.
- Consumers may react under their own rules but cannot rewrite the source fact.
- Failed or incomplete transitions publish no success event.

---

# 17. Open Issues, Assumptions and Deferred Questions

**Architecture assessment:** `PASS WITH ISSUES`

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-288-ISSUE-01 | Non-blocking | L2-G10 Section 44 names `CapacityUpdated`, while WF-INF-008 names `RoomCapacityChanged`; the aggregate-neutral canonical event is inconsistent. | Align L2-G10 and the named participating owner before downstream design depends on the missing or inconsistent contract. |
| G-288-ISSUE-02 | Non-blocking | The frozen term “activation” is an outcome meaning; L2-G10 defines no Capacity Profile lifecycle state named Active. | Align L2-G10 and the named participating owner before downstream design depends on the missing or inconsistent contract. |

No unsupported aggregate, lifecycle state, transition, event, publisher, actor, approval authority, cross-domain contract or implementation mechanism is assumed by this Draft.
