# Space Capability Definition to Operational Capability Availability

# 1. Scenario Metadata

**Scenario ID:** L3-G-287  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A01 — Platform Tenant Foundation Architecture  
**Referenced By:** Authorized consumer domains requiring Space operational eligibility facts

---

# 2. Scenario Purpose

Define the G10-owned operational capabilities that a canonical Space is permitted to support, using configured capability values without transferring scheduling or operational decision ownership to Infrastructure Management.

---

# 3. Business Outcome

Applicable operational capabilities become available as authoritative infrastructure attributes.

L2-G10 owns Operational Capabilities and requires capability assignment to be configuration-driven and auditable, but it does not define the complete workflow, exact canonical event, or explicit public capability contract required to prove the frozen outcome. This Draft is **BLOCKED**.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a physical space requires defined capabilities.

## 4.2 End Boundary

The scenario ends when the applicable operational capabilities are authoritatively associated with the canonical Space under G10 ownership and are available to authorized consumers through a source-defined business fact/contract, with the required canonical change event published.

## 4.3 In Scope

- Identification of the canonical Space.
- Resolution of configured Operational Capability values.
- Validation and assignment of one or more permitted operational capabilities.
- Preservation of Space identity, classification, hierarchy and lifecycle.
- Auditability of the capability assignment.
- Publication and exposure of the completed authoritative capability fact, once defined upstream.

## 4.4 Out of Scope

- Space registration or Space Type classification.
- Timetable scheduling, examination allocation or other consumer eligibility decisions.
- Consumer-specific interpretation of a capability.
- Creation of a local capability lifecycle, event or read contract.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-284 — Space Registration to Physical Space Establishment | Operational capabilities belong to an existing canonical Space. |
| Related | L3-G-286 — Space Classification to Controlled Physical Classification | Space Type and Operational Capabilities are separate G10-owned concepts. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns Operational Capability semantics as Space attributes and owns the resulting authoritative physical fact. |
| Platform Foundation / A | Owns common configuration resolution, validation and lifecycle; it does not own G10 Operational Capability values or business meaning. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Applies permitted operational capability attributes to the Space. | Acts within source-defined infrastructure update/administration authority. |

No consumer-domain actor is included because the frozen scenario stops when the capability fact becomes authoritative; downstream use is a later consumer decision.

---

# 7. Preconditions and Entry Criteria

- A canonical Space exists under G10.
- Operational Capability values are configured under G10.
- A valid active configuration context is available from the Platform Configuration Framework.
- The Space is eligible for permitted metadata/capability modification under G10 rules.
- The Infrastructure Administrator is authorized for the applicable update.

---

# 8. Trigger

A physical space requires defined capabilities.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the canonical Space and the active configuration context. | Infrastructure Management; Platform Foundation | The capability decision is grounded in authoritative Space and configuration facts. |
| 2 | Identify the applicable configured Operational Capabilities. | Infrastructure Administrator / Infrastructure Management | Candidate capability values are drawn from G10-owned configuration. |
| 3 | Validate that the capability assignment is permitted for the Space. | Infrastructure Management | Unsupported capability values are rejected. |
| 4 | Associate the permitted Operational Capabilities with the canonical Space without changing Space identity or ownership. | Infrastructure Management | The Space carries the intended G10-owned capability facts. |
| 5 | Record the capability assignment as an auditable infrastructure change. | Infrastructure Management | The business change is traceable. |
| 6 | Publish the exact canonical capability-change event. | Infrastructure Management | **Blocked:** no capability-specific canonical event is defined and generic update event definitions are not mapped to this outcome. |
| 7 | Make the capabilities available through the authoritative consumer contract/fact. | Infrastructure Management | **Blocked:** the published Space Summary contract does not include Operational Capabilities and no dedicated capability contract is defined. |

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Requested capability is not configured or configuration is inactive. | Capability assignment cannot proceed. | Correct the authoritative domain configuration and complete its source-defined configuration lifecycle before a later attempt. | Infrastructure Management; Platform Foundation |
| Capability assignment fails G10 validation or authorization. | Existing Space capability facts remain authoritative. | Correct the business condition before a later authorized attempt. | Infrastructure Management |
| Canonical event is undefined for capability assignment. | The successful capability change cannot publish one exact required business event. | Define the canonical capability-change event and publisher upstream. | Infrastructure Management |
| Canonical consumer contract/fact is undefined for Operational Capabilities. | The frozen outcome cannot be proven as available to consumers without inventing a contract. | Define or explicitly extend an authoritative G10 contract upstream. | Infrastructure Management |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Space may support one or more Operational Capabilities. | Infrastructure Management | Multiple applicable capability facts may be associated with one Space. | L2-G10 §§7, 10, 23 |
| Operational Capabilities determine operational eligibility for downstream domains. | Infrastructure Management | G10 publishes physical eligibility facts; consumers retain their own decisions. | L2-G10 §23; FSWP-03 |
| Capability assignment is configuration-driven and fully auditable. | Infrastructure Management | Only configured values may be assigned and the change must be traceable. | L2-G10 §23; §43 |
| Only Active resolved configuration participates in business execution. | Platform Foundation | G10 consumes validated active configuration context. | L2-A01 §§6.10-6.14 |
| Platform does not interpret domain-specific configuration. | Platform Foundation | G10 retains Operational Capability semantics. | L2-A01 Domain Configuration |

---

# 13. State and Lifecycle Impact

No Space lifecycle transition is defined for Operational Capability assignment.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Space | Existing canonical Space | Adds/changes Operational Capability attributes | Space lifecycle remains unchanged | Infrastructure Management | L2-G10 §§10, 21, 23 |
| Configuration | Active resolved configuration | Supplies permitted capability values | Configuration lifecycle unchanged | Platform Foundation / G10 values | L2-A01 §§6.10-6.14; L2-G10 §43 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-G10 requires every successful workflow/significant change to publish a canonical business event, but no event is explicitly defined for Operational Capability assignment.

The source contains generic `InfrastructureUpdated` / `SpaceUpdated` events for broader update behaviour, but it does not state that either is the canonical event for this frozen capability outcome. L3 does not infer that mapping.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform Foundation | Infrastructure Management | Resolved active configuration context | Supplies validated configuration without taking ownership of capability semantics. | L2-A01 §§6.11-6.14 |

G10's published `Space Summary` lists Space Type, parent, capacity, area, operational status and lifecycle but does **not** list Operational Capabilities. No dedicated Operational Capability contract is defined. The required consumer hand-off is therefore unresolved.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Define Space Operational Capabilities | Infrastructure Administrator | Must act within source-defined infrastructure update/administration authority and use permitted G10 configuration. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, approval or override requirement is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Capability assignment is auditable. | Source-defined evidence that permitted Operational Capabilities were associated with the canonical Space through an authorized business change. | Infrastructure Management |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Applicable Operational Capabilities are associated with the canonical Space under G10 ownership.
- Only permitted configured capability values are used.
- Space identity, hierarchy, classification ownership and lifecycle remain unchanged.
- The exact canonical capability-change event is defined and published by Infrastructure Management.
- An authoritative G10 contract/fact exposes Operational Capabilities to authorized consumers.

## 16.2 Postconditions

- Downstream domains may use the authoritative capability fact for their own eligibility and operational decisions.
- Capability consumption never transfers Space or capability ownership away from G10.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 owns Operational Capabilities and requires capability assignment to be auditable, but defines no complete capability-assignment workflow or exact canonical event. | L3 cannot preserve the mandatory event/publisher contract for the frozen outcome without inference. | G10 must define the canonical capability-assignment workflow/event or explicitly map the outcome to an existing canonical update event. |
| The G10 Space Summary contract does not include Operational Capabilities and no dedicated capability read contract is defined. | The frozen outcome requires capabilities to become available as authoritative infrastructure attributes, but the approved consumer hand-off is undefined. | G10 must define or explicitly extend the authoritative consumer contract for Operational Capabilities. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
