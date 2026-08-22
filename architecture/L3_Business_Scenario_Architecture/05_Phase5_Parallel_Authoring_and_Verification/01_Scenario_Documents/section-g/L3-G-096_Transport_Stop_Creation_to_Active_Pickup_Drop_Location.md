# Transport Stop Creation to Active Pickup/Drop Location

**Scenario ID:** L3-G-096  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-096 |
| Canonical Title | Transport Stop Creation to Active Pickup/Drop Location |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario establishes a new Transport Stop through the authoritative G04 Stop Management workflow until it is active for operational pickup/drop use. G04 defines a Transport Stop as a designated pickup or drop location and states that stops are reusable across multiple routes.

## 3. Business Outcome

A Transport Stop is established, its location verified, its required route association established, operational validation completed, and the stop reaches the canonical **Active** state.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A legitimate institutional transport requirement exists for a new pickup/drop location and the proposed stop enters Transport Management's Stop Management workflow.

### 4.2 End Boundary
The Transport Stop is Active and available for its authorised pickup/drop purpose.

### 4.3 In Scope
- Stop establishment and business information.
- Location verification.
- Route assignment.
- Operational validation.
- Stop activation.
- Historical/audit preservation.

### 4.4 Out of Scope
- Vehicle asset ownership or maintenance execution.
- Student/workforce lifecycle ownership.
- Financial accounting.
- Notification delivery mechanics.
- Technical APIs, databases, services, queues, UI, deployment or infrastructure.
- Separate canonical allocation, trip, diversion or passenger-reassignment outcomes.

## 5. Ownership and Participating Domains

Transport Management (G04) is the canonical owner of the transport operational outcome. The frozen participating-domain set is **G only**. Participation or downstream consumption does not transfer ownership.

G04 does not own vehicle assets, maintenance execution, student lifecycle, employee ownership or financial accounting. Those boundaries remain unchanged.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Transport Administrator | Establishes and configures the Transport Stop. |
| Transport Manager / configured operational authority | Performs review/decision responsibilities where the transport workflow requires them. |
| Downstream transport consumers | Consume the active stop outcome without acquiring ownership. |

technical access control/IAM is outside L3.

## 7. Preconditions and Entry Criteria

- A valid institutional transport requirement exists.
- The proposed location is within Transport Management scope.
- Required stop information can be established.
- The location can be verified.
- The stop can be associated with the required route context.

## 8. Trigger

A new Transport Stop is proposed for operational pickup/drop use.

## 9. Main Business Flow

1. **Establish the stop.** Transport Management establishes the Transport Stop within G04.
2. **Capture stop information.** Source-supported information includes Stop Code, Stop Name, Pickup Availability, Drop Availability, Campus and Operational Status; additional information remains configurable.
3. **Verify location.** The proposed pickup/drop location is verified.
4. **Assign route context.** The stop is assigned to the required route context.
5. **Perform operational validation.** The stop is evaluated for operational suitability.
6. **Activate the stop.** The stop reaches the canonical Active lifecycle state.
7. **Preserve history.** The stop's operational history remains traceable.

## 10. Alternate Business Flows

- If location verification is incomplete, activation is withheld until resolved.
- If route assignment is incomplete, the stop remains non-active.
- If operational validation fails, the business condition is corrected and the stop is revalidated.
- If the proposed location is no longer required, the stop is not activated; no new closure state is invented.

## 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Business Recovery |
|---|---|---|
| Incomplete stop information | Stop cannot be operationally established | Complete/correct the information |
| Location cannot be verified | Stop cannot become Active | Resolve location verification |
| Route assignment missing | Stop cannot complete validation | Establish the required route relationship |
| Operational validation fails | Stop remains non-active | Correct the business condition and revalidate |

Recovery is business-level only. No technical retries, queues, rollback or infrastructure failure handling is introduced.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Scenario Effect |
|---|---|
| Transport Stop owns stop information, route association, pickup rules, drop rules and operational status. | Creation/activation remain G04-owned. |
| Stops remain reusable across multiple routes. | Creation does not imply exclusive route ownership. |
| Typical stop information includes Stop Code, Stop Name, GPS Reference (future ready), Pickup Availability, Drop Availability, Campus and Operational Status. | Only source-supported information is used. |
| Temporary stop restrictions preserve historical information. | Creation does not erase history. |
| Every lifecycle transition is audited. | Creation and activation remain traceable. |

## 13. State and Lifecycle Impact

Canonical Stop lifecycle:

`Created → Assigned → Active → Suspended`

The authoritative Stop workflow is:

`Stop Created → Location Verified → Route Assignment → Operational Validation → Stop Activated`

This scenario covers establishment through **Active**. Location Verified and Operational Validation are workflow steps, not newly invented lifecycle states.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G04 explicitly defines:
- `StopCreated` — stop registered.
- `StopAssigned` — stop assigned to route.

The retrieved G04 event catalogue does not define a separate `StopActivated` event. No activation event is invented. The Active lifecycle outcome remains the business completion condition.

### 14.2 Synchronous Business Contracts / Hand-offs

The frozen participating-domain set is G only. No additional cross-domain synchronous hand-off is required to complete the scenario. Downstream consumers do not acquire ownership or directly manipulate G04 transport state.

## 15. Business Authorization, Approval and Audit Obligations

G04 identifies Transport Administrator as a typical authority for configuring stops. This is business authority only.

The stop's establishment, assignment, validation, activation and operational history are subject to G04's immutable audit requirement.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Stop established.
- Location verified.
- Route assignment established.
- Operational validation completed.
- Stop reaches **Active**.
- Pickup/drop use is operationally available.
- History remains traceable.

### 16.2 Postconditions
- The named business outcome is authoritative within G04.
- Relevant transport history remains traceable.
- No unrelated domain lifecycle is implicitly changed.
- Later allocation, trip and notification activity remains governed by its own canonical scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
The exact event representing the Active transition is not exposed in the retrieved G04 event catalogue. Confirm the canonical event catalogue before Architecture Approval/Lock if an activation-specific event is required.

### 17.2 Assumptions
- Institution-specific operational policies remain configuration driven as stated by G04.
- No actor, rule, state or event absent from authoritative architecture is invented.

### 17.3 Deferred Questions
Any material unresolved upstream naming or semantic issue must be resolved at the authoritative architecture level before Architecture Approval/Lock.
