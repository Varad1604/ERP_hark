# Space Classification to Controlled Physical Classification

# 1. Scenario Metadata

**Scenario ID:** L3-G-286  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A01 — Platform Tenant Foundation Architecture  
**Referenced By:** Authorized consumers of G10 Space classification facts

---

# 2. Scenario Purpose

Apply an authorized G10-owned Space classification using institutionally configured Space Types while preserving Space identity, G10 ownership and the Platform/Foundation boundary for configuration governance.

---

# 3. Business Outcome

The Space receives its authorised classification and remains available to consuming domains.

The locked source is ambiguous about whether this scenario represents initial classification or later reclassification, and it defines conflicting canonical event names for the classification change. This Draft is **BLOCKED**.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid space requires classification according to institutional configuration.

## 4.2 End Boundary

The scenario ends when the Space has one authorized primary Space Type under G10 ownership, the authoritative classification is available through the G10 Space fact, Space identity and lifecycle are preserved, and the exact canonical classification event has been published.

## 4.3 In Scope

- Resolution of the applicable configured Space Type.
- Validation and assignment/change of the Space's primary classification.
- Preservation of Space identity and physical hierarchy.
- G10 auditability and classification-event publication.
- Platform configuration governance without transfer of domain configuration semantics.

## 4.4 Out of Scope

- Creation of the Space itself.
- Operational capability assignment.
- Scheduling, examination allocation, hostel use or other consumer interpretation.
- Changes to Space lifecycle or operational status solely because classification changes.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-284 — Space Registration to Physical Space Establishment | Space registration already lists Space Type as mandatory information, creating the sequencing issue recorded in Section 17. |
| Related | L3-G-287 — Space Capability Definition to Operational Capability Availability | Operational capability is a separate G10-owned attribute from primary Space classification. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns Space Type semantics, Space classification and the resulting authoritative Space fact. |
| Platform Foundation / A | Owns the common configuration framework, resolution, validation and lifecycle; it does not interpret or own G10 Space Type semantics. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Applies the permitted classification to the Space. | Performs the G10 classification action within authorized scope. |

---

# 7. Preconditions and Entry Criteria

- A canonical Space exists under G10.
- The applicable Space Type is defined through authoritative G10 configuration.
- Platform configuration resolution has supplied a valid, active configuration context.
- The requested classification is permitted for the Space.
- The Infrastructure Administrator is authorized to perform classification changes.

The source is ambiguous about an **unclassified existing Space**, because Space Type is mandatory during Space registration. This ambiguity is not silently resolved here.

---

# 8. Trigger

A valid space requires classification according to institutional configuration.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the canonical Space and the applicable active configuration context. | Infrastructure Management; Platform Foundation | The classification decision is grounded in authoritative Space and configuration facts. |
| 2 | Identify the permitted G10 Space Type required for the Space. | Infrastructure Administrator / Infrastructure Management | A candidate primary classification is selected from G10-owned configuration values. |
| 3 | Validate the requested classification against G10 rules and configuration. | Infrastructure Management | Unsupported classification is prevented. |
| 4 | Apply the authorized primary Space classification without changing Space identity, hierarchy or lifecycle. | Infrastructure Management | The Space carries the controlled G10 classification. |
| 5 | Make the classification available as part of the authoritative Space fact. | Infrastructure Management | Authorized consumers can resolve the Space Type without owning it. |
| 6 | Record the classification change under G10 audit obligations. | Infrastructure Management | The classification decision is auditable. |
| 7 | Publish the exact canonical classification event. | Infrastructure Management | **Blocked:** locked G10 sections define conflicting event names. |

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Requested Space Type is absent from authoritative G10 configuration. | Classification cannot proceed. | Correct or approve the domain configuration through the authoritative configuration lifecycle before a later attempt. | Infrastructure Management; Platform Foundation |
| Configuration context is invalid or inactive. | The classification value cannot participate in business execution. | Resolve a valid active configuration context before a later attempt. | Platform Foundation |
| Classification fails G10 validation or authorization. | Existing Space classification remains authoritative. | Correct the business condition before a later authorized attempt. | Infrastructure Management |
| Event-name conflict remains unresolved. | The successful classification fact cannot be published under one canonical event identity. | Resolve the event conflict through architecture change control. | Infrastructure Management |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Space possesses exactly one primary Space Type. | Infrastructure Management | The scenario establishes or changes one controlled primary classification. | L2-G10 §22 |
| Infrastructure classifications are configurable. | Infrastructure Management | Only configured Space Types may be applied. | L2-G10 Space Classification Workflow; §43 |
| Aggregate identity remains unchanged when classification changes. | Infrastructure Management | Reclassification cannot create a new Space identity. | L2-G10 §22; identity rules |
| Only Active configuration may participate in business execution. | Platform Foundation | G10 consumes a validated active configuration context. | L2-A01 §§6.10-6.14 |
| Platform does not interpret domain-specific configuration. | Platform Foundation | G10 retains classification semantics and ownership. | L2-A01 Domain Configuration; §6.12 |

---

# 13. State and Lifecycle Impact

No Space lifecycle transition is defined as part of a classification change. Space identity and lifecycle remain unchanged.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Space | Existing valid Space | Assigns/changes primary Space Type | Existing Space lifecycle remains unchanged; classification is updated | Infrastructure Management | L2-G10 §§22, 141 |
| Configuration | Active resolved configuration | Supplies permitted Space Type values | Configuration lifecycle is unchanged by this scenario | Platform Foundation / G10 values | L2-A01 §§6.10-6.14; L2-G10 §43 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The exact classification event is contradictory:

- `InfrastructureClassificationChanged` in WF-INF-007 and §44A classification events.
- `SpaceClassificationChanged` in the dedicated Space Classification Workflow and Floor & Space Events Summary.

Infrastructure Management is the publisher. L3 does not rename or select between them.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform Foundation | Infrastructure Management | Resolved active configuration context | Supplies validated configuration without taking ownership of Space Type semantics. | L2-A01 §§6.11-6.14 |
| Infrastructure Management | Authorized consumers | Space Summary | Exposes the authoritative `Space Type` together with the canonical Space identity. | L2-G10 §44A.193 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Classify Space | Infrastructure Administrator | Must have source-defined classification/update authority and apply only permitted G10 configuration. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, approval or override requirement is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Classification changes are auditable. | Source-defined evidence of the prior and resulting classification and the authorized business action. | Infrastructure Management |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- One authorized primary Space Type is established for the canonical Space.
- The classification uses valid G10-owned configuration semantics.
- Space identity, hierarchy and lifecycle remain unchanged.
- The classification is available through the authoritative Space fact.
- The exact canonical classification event is resolved and published by Infrastructure Management.

## 16.2 Postconditions

- Consuming domains may use the authoritative Space Type under their own rules but do not own or redefine it.
- Platform Foundation retains configuration-framework ownership; Infrastructure Management retains Space classification ownership.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 requires `Space Type` during Space registration, yet the frozen scenario separately begins when an existing valid Space “requires classification.” | The scenario boundary is ambiguous between initial classification and reclassification. | Upstream architecture must clarify whether this scenario is limited to reclassification or how a valid unclassified Space can exist. |
| L2-G10 defines classification events as both `InfrastructureClassificationChanged` and `SpaceClassificationChanged`. | L3 cannot preserve one exact canonical event name. | Architecture governance must resolve the event identity while retaining G10 as publisher. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
