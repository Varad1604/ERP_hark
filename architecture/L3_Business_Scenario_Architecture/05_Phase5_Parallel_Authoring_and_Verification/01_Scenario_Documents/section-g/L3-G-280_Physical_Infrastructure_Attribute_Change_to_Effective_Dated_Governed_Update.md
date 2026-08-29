# Physical Infrastructure Attribute Change to Effective-Dated Governed Update

# 1. Scenario Metadata

**Scenario ID:** L3-G-280  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A01 — Platform & Tenant Foundation; L2-D03 — Timetable Workflows where Academic Delivery consumes physical infrastructure facts  
**Referenced By:** Authorized consumers of current Physical Campus, Building, Floor and Space facts

---

# 2. Scenario Purpose

Govern an authorized change to a G10-owned physical infrastructure attribute while preserving immutable aggregate identity, source ownership, historical traceability and downstream consumption boundaries.

---

# 3. Business Outcome

The new attribute value becomes effective according to policy, consuming domains resolve the current authoritative infrastructure fact, and prior values remain historically traceable.

The locked G10 source does not define one universal effective-dated attribute-change lifecycle or contract across all attribute classes covered by the frozen trigger, and its event catalogue varies by attribute and aggregate. Because the frozen outcome requires these semantics across the full scenario scope, this Draft is **BLOCKED**.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised attribute of a physical infrastructure aggregate changes, including campus, building, floor or space metadata, capacity, capability, sustainability characteristics or security classification.

## 4.2 End Boundary

The scenario ends only when the changed G10 fact is effective according to an upstream-defined policy for that attribute class, the prior value remains historically traceable, the current authoritative fact is available to consumers, and the exact required completed business event is unambiguous.

## 4.3 In Scope

- Identification of the affected Physical Campus, Building, Floor or Space aggregate.
- Authorization of the attribute change within G10 scope.
- Validation under the applicable G10 business rule and lifecycle constraints.
- Preservation of aggregate identity and prior historical evidence.
- Effective dating where the authoritative source explicitly defines it.
- Publication of the exact applicable G10 attribute-change event where unambiguous.
- Availability of the resulting authoritative physical fact to authorized consumers.

## 4.4 Out of Scope

- Academic scheduling changes or other consumer-domain reactions.
- Treating Platform configuration lifecycle as ownership of G10 physical attributes.
- Inventing a universal effective-dated attribute version model for all G10 attributes.
- Inventing or selecting an event where locked G10 sources do not establish one unambiguously.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment | Establishes a Physical Campus whose physical attributes may later change. |
| Related | L3-G-282 — Building Registration to Physical Building Establishment | Establishes a Building whose attributes may later change. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Primary owner of physical infrastructure identity and the physical attributes covered by the scenario. |
| Platform Foundation / A | Owns the common configuration framework and resolution lifecycle; does not own G10 physical attribute semantics. |
| Academic Delivery / D | Consumes authoritative G10 physical facts for its own scheduling decisions; does not modify G10 aggregates. |

The frozen participating-domain value remains exactly `G, A, D`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Infrastructure Administrator | Initiates permitted physical-attribute changes. | May act only within authorized infrastructure scope and G10 business rules. |
| Campus Administrator / Building Administrator | Manages permitted attributes within assigned physical scope. | Cannot alter facts outside assigned scope or bypass G10 validation. |
| Auditor | Reviews immutable change history. | Read-only. |

No universal attribute-change approver is inferred.

---

# 7. Preconditions and Entry Criteria

- The affected G10 aggregate exists and is not Archived.
- The requested attribute is owned by G10 and is editable under the current business condition.
- The proposed value and business reason are available.
- Applicable policy, configuration and validation rules can be resolved.
- The requester is authorized for the affected infrastructure scope.

---

# 8. Trigger

An authorised attribute of a physical infrastructure aggregate changes, including campus, building, floor or space metadata, capacity, capability, sustainability characteristics or security classification.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the affected G10 aggregate, attribute and current authoritative value. | Infrastructure Management | The change boundary is explicit. |
| 2 | Confirm that G10 owns the requested physical attribute and authorize the change. | Infrastructure Management | No consumer domain acquires mutation authority. |
| 3 | Validate the proposed value against the attribute-specific G10 rules and current lifecycle constraints. | Infrastructure Management | Invalid changes are rejected. |
| 4 | Resolve the applicable effective-date rule for the changed attribute. | Infrastructure Management; Platform configuration framework where applicable | **Blocked for generic scope:** locked G10 does not define universal effective-dating semantics for every attribute named by the frozen trigger. |
| 5 | Preserve the predecessor value and apply the successor value only under the authoritative attribute-specific rule. | Infrastructure Management | Historical traceability is retained only where supported upstream. |
| 6 | Make the current authoritative G10 fact available through the applicable read contract. | Infrastructure Management | Consumers can resolve the current physical fact. |
| 7 | Publish the exact applicable completed business event. | Infrastructure Management | **Blocked for generic scope:** event selection depends on attribute/aggregate and is not uniformly defined. |
| 8 | Preserve immutable change evidence including previous value, new value, applicable effective date, actor and reason. | Infrastructure Management | The governed change remains auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Capacity change | The changed attribute is capacity. | Apply the G10 Capacity Management rules, including its explicit effective-dating support. | Capacity history remains governed by G10. |
| Physical Campus attribute change | The change concerns a source-defined Physical Campus attribute workflow. | Use the exact corresponding G10 workflow and event such as boundary, capacity, accessibility, sustainability, criticality or operational status. | Only that source-defined attribute path is used. |
| Consumer reaction | Academic Delivery observes a changed infrastructure fact. | D03 evaluates the change under its own timetable and scheduling rules. | G10 does not perform the academic decision. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Attribute is not G10-owned. | G10 cannot make the change. | Route the change to the canonical owner and leave G10 unchanged. | Applicable canonical owner |
| Proposed value violates G10 rules. | Authoritative physical integrity would be compromised. | Reject the change and retain the current value. | Infrastructure Management |
| Effective-dating rule is undefined for the attribute. | The frozen governed-update outcome cannot be applied canonically. | Do not invent a version or effective-date lifecycle; require upstream clarification. | Infrastructure Management / architecture governance |
| Exact completed event is ambiguous or absent. | Event integrity cannot be preserved. | Do not invent or rename an event; resolve upstream. | Infrastructure Management / architecture governance |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure is the canonical source of physical facts. | Infrastructure Management | D and other consumers react to published facts but cannot modify them. | L2-G10 §§2, 9A, 44A |
| Aggregate identity is immutable. | Infrastructure Management | Attribute changes never replace the canonical identity. | L2-G10 §§8, 13A.64 |
| Effective dating is explicitly required for operational allocations, responsibility assignments and configurable policies, and Capacity Management supports effective-dated changes. | Infrastructure Management | These source-defined cases may use effective dating; L3 may not generalize the rule to every attribute. | L2-G10 §6, §§24, 27-28, 43 |
| Business-domain configuration values remain owned by their business domain; Platform owns the configuration framework. | G10 / Platform Foundation | A01 cannot be used to redefine G10 physical attribute semantics. | L2-A01 §§6.1-6.16 |
| D03 consumes G10 Physical Infrastructure Resources and owns scheduling. | Academic Delivery | A changed G10 fact does not itself modify a timetable. | L2-D03 §§1-2 |
| Every successful significant G10 change is auditable and publishes a canonical business event. | Infrastructure Management | Completion requires an exact source-defined event. | L2-G10 §§13A.76, 22A.99, 44A |

---

# 13. State and Lifecycle Impact

The frozen scenario is an attribute update, not a lifecycle transition. The affected Physical Campus, Building, Floor or Space therefore remains in its existing canonical lifecycle unless a separately authorized lifecycle transition is triggered.

Prior and current attribute values must remain traceable where G10 defines history/effective dating. No universal attribute-version lifecycle is introduced.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 defines generic and attribute-specific completed facts including `InfrastructureUpdated`, `InfrastructureClassificationChanged`, `RoomCapacityChanged`, `SpaceAccessibilityUpdated`, `SustainabilityProfileUpdated`, and Physical Campus-specific attribute events such as `CampusCapacityUpdated` and `CampusSustainabilityUpdated`.

The frozen scenario spans multiple aggregates and attribute classes. The exact required event must therefore be selected by an upstream source-defined workflow, not by L3 inference. Where no exact source-defined event exists for the requested attribute/aggregate combination, the scenario remains blocked.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| G10 | Authorized consumers including D03 | Applicable Physical Campus, Building, Floor, Space or profile read contract | Provides the current authoritative physical fact. |
| A01 | G10 | Resolved configuration context | Supplies validated configuration framework context without owning G10 business semantics. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Only a source-supported, properly scoped infrastructure role may initiate the change.

## 15.2 Approval and Separation of Duty

Where G10 or applicable configuration requires approval, that source-defined approval must complete. No universal approver or separation-of-duty rule is invented.

## 15.3 Business Audit and Evidence Obligations

The audit history must preserve the affected aggregate identity, attribute, predecessor value, successor value, effective date where defined, actor, business reason, approval reference where applicable, and published completed fact.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when:

- the changed attribute is confirmed as G10-owned;
- its source-defined validation and effective-date policy are available;
- the new value becomes current according to that policy;
- prior values remain traceable under authoritative history rules;
- consuming domains can resolve the current G10 fact without ownership transfer; and
- the exact canonical event for the completed attribute change is defined and published by G10.

The current locked sources do not support these completion conditions uniformly across the full frozen trigger scope.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-280-BLOCKER-01 | Blocking | The frozen trigger spans generic metadata, capacity, capability, sustainability and security classification across Physical Campus, Building, Floor and Space, but L2-G10 does not define one universal effective-dated attribute-version workflow or lifecycle for all of those classes. | L2-G10 must define the applicable effective-dating semantics per attribute class or narrow the scenario contract. |
| G-280-BLOCKER-02 | Blocking | G10 exposes generic and attribute-specific events, but the exact canonical event is not uniformly defined for every aggregate/attribute combination covered by the frozen scenario. | L2-G10 must map each supported governed update to its canonical completed event and publisher. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
