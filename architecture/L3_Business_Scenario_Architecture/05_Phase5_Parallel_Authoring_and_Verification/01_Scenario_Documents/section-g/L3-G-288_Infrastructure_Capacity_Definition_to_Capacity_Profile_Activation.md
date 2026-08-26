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
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Establish an authoritative Capacity Profile for physical infrastructure so institutional consumers can rely on governed capacity facts without acquiring ownership of the infrastructure or its capacity.

# 3. Business Outcome

A Capacity Profile becomes authoritative for the applicable infrastructure.

The locked G10 source requires successful capacity workflows to publish a canonical business event but gives conflicting names for the capacity-change fact. The Draft is therefore **BLOCKED** pending upstream event-name resolution.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when capacity information is established for a physical infrastructure aggregate.

## 4.2 End Boundary

The scenario ends when a valid Capacity Profile is authoritative for the applicable infrastructure, its prior capacity history is preserved, and the required canonical capacity event can be identified and published unambiguously.

## 4.3 In Scope

- Capacity Profile definition for G10-owned infrastructure.
- Capacity-type, value, lifecycle and policy validation.
- Effective-dated capacity changes and source-supported temporary overrides.
- Historical capacity preservation and auditability.
- Canonical business-event publication required by G10.

## 4.4 Out of Scope

- Utilisation, timetable, examination or other consumer-domain decisions.
- Changes to physical-infrastructure ownership or hierarchy.
- Creation of a local Capacity Profile lifecycle state named `Active`.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns Capacity Profile, capacity rules, capacity history and the final capacity outcome. |
| Platform Foundation / A | Supplies authorization, event governance and audit capabilities without owning the capacity fact. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the capacity definition for the applicable infrastructure. | Performs the G10 capacity action only where the required authorization decision permits it. |

No separate canonical capacity approver is defined for all cases; approval applies only where the configured G10 rule requires it.

# 7. Preconditions and Entry Criteria

- The applicable infrastructure exists and its lifecycle permits capacity modification.
- The capacity type is supported by institutional configuration.
- The proposed capacity value is valid and non-negative.
- The business action is authorized before the capacity change proceeds.

# 8. Trigger

Capacity information is established for a physical infrastructure aggregate.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the applicable infrastructure and capacity type. | Infrastructure Administrator / Infrastructure Management | The capacity definition is bounded to a canonical G10 resource. |
| 2 | Validate authorization, infrastructure eligibility, capacity type, value and governing policy. | Infrastructure Management | Invalid or unauthorized capacity definitions do not proceed. |
| 3 | Determine the effective capacity definition, including a bounded temporary override where source-supported. | Infrastructure Management | The effective capacity position is established without erasing prior history. |
| 4 | Make the Capacity Profile authoritative and preserve the prior/new capacity history required by G10. | Infrastructure Management | The frozen business outcome is achieved. |
| 5 | Publish the canonical capacity business event required by G10. | Infrastructure Management | **Blocked:** locked G10 defines conflicting event names for this capacity fact. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Temporary capacity override | A bounded override is permitted by institutional policy. | Apply the override for its effective period and restore the configured capacity after expiry while retaining history. | Capacity remains governed and historically traceable. | Infrastructure Management |
| Multiple capacity types | More than one configured capacity category applies. | Maintain each supported capacity definition under the applicable rules. | Each authoritative capacity fact remains independently governed. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Capacity value, type, policy or lifecycle validation fails. | No new authoritative capacity position is established. | Correct the business condition before a later authorized capacity action. | Infrastructure Management |
| Authorization is not granted. | Capacity cannot be changed. | A later action may proceed only after a valid authorization decision. | Platform Foundation / Infrastructure Management |
| Canonical capacity event name remains unresolved. | The required completed business fact cannot be published with one exact canonical name. | Resolve the locked G10 event-name conflict through architecture change control. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns capacity. | Infrastructure Management | Consumers may use capacity facts but do not own or rewrite them. | L2-G10 §24 |
| Capacity shall support configurable profiles, multiple types, effective dating, temporary overrides, history and auditability. | Infrastructure Management | Capacity definition preserves governed temporal history. | L2-G10 §24 Capacity Principles |
| Capacity shall never become negative. | Infrastructure Management | Invalid negative values cannot become authoritative. | L2-G10 §24 |
| Every successful G10 workflow publishes at least one canonical business event. | Infrastructure Management | An unresolved event-name conflict prevents canonical completion. | L2-G10 WDP-02 |
| Events represent completed business facts and the business domain remains their owner. | Platform Foundation — Event Infrastructure; Infrastructure Management | Platform event governance does not transfer capacity ownership. | L2-A06 EIP-01, EIP-02 |

# 13. State and Lifecycle Impact

No Capacity Profile lifecycle with a state named `Active` is defined by locked G10. The frozen title's “Activation” is treated only as the business outcome that the profile becomes authoritative.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Capacity Profile | Existing or absent authoritative capacity definition | Establishes the current effective capacity fact and preserves history | Authoritative Capacity Profile; no invented lifecycle state | Infrastructure Management | L2-G10 §§8, 24 |
| Physical infrastructure | Existing lifecycle position that permits modification | Capacity changes without changing infrastructure lifecycle | Existing lifecycle position remains | Infrastructure Management | L2-G10 §24 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Locked G10 is contradictory for the required capacity-change event:

- `CapacityChanged` appears in WF-INF-008's flow.
- `RoomCapacityChanged` appears in the WF-INF-008 published-event block and Space Capacity workflow.
- `CapacityUpdated` appears in the Capacity Events catalogue.

Infrastructure Management is the business publisher, but L3 cannot choose or rename the event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G10 exposes canonical Capacity Profile facts for authorized consumers. Consumption does not transfer ownership. No additional synchronous business hand-off is required to complete this scenario.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Define or change infrastructure capacity | Infrastructure Administrator | Must satisfy the applicable authorization and G10 capacity rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal separation-of-duty rule is defined for ordinary capacity definition. A temporary override carries only the source-defined approval reference where policy requires one.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve capacity history | Previous capacity, new capacity, effective date, business reason and applicable override history. | Infrastructure Management |
| Preserve immutable audit evidence | Audit evidence derived from the completed canonical business fact. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A valid Capacity Profile is authoritative for the applicable infrastructure.
- Prior capacity history is preserved.
- The exact canonical capacity event is unambiguously defined and published by Infrastructure Management.

## 16.2 Postconditions

- Authorized consumers can rely on the current G10 capacity fact.
- Physical-infrastructure ownership remains with Infrastructure Management.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-G10 uses `CapacityChanged`, `RoomCapacityChanged`, and `CapacityUpdated` for the capacity-change business fact while also requiring canonical event publication. | L3 cannot preserve one exact canonical event name, so the scenario cannot be source-complete. | Architecture governance must establish the canonical event name and retain Infrastructure Management as publisher. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
