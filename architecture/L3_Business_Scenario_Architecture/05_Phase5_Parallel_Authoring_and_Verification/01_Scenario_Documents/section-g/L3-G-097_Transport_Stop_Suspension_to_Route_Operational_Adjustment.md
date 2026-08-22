# Transport Stop Suspension to Route Operational Adjustment

**Scenario ID:** L3-G-097  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
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
| Final L3 ID | L3-G-097 |
| Canonical Title | Transport Stop Suspension to Route Operational Adjustment |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs suspension of an operational Transport Stop and the resulting route-level operational adjustment required when that stop can no longer be used. The stop suspension remains a Stop lifecycle outcome; the route consequence remains within G04 Route Management.

## 3. Business Outcome

The affected Transport Stop is suspended and the affected Transport Route is operationally adjusted as required so the route no longer relies on the unavailable stop for the affected operation.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Active Transport Stop becomes unavailable for operational use and Transport Management determines that the affected route requires an operational adjustment.

### 4.2 End Boundary
The stop has reached Suspended and the required route operational adjustment has been completed.

### 4.3 In Scope
- Stop suspension.
- Review of route implications.
- Route operational adjustment where required.
- Preservation of stop and route history.

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
| Transport Administrator | Manages stop configuration and suspension. |
| Transport Manager | Provides route-change approval responsibility where required. |
| Downstream transport consumers | Consume the resulting operational state without modifying it. |

technical access control/IAM is outside L3.

## 7. Preconditions and Entry Criteria

- The stop exists and is within G04 scope.
- The stop is in an operational state from which suspension is applicable.
- A business basis for suspension exists.
- The affected route context is known.

## 8. Trigger

An operational requirement arises to suspend an existing Transport Stop.

## 9. Main Business Flow

1. **Identify the stop and route context.**
2. **Establish the suspension basis.** The business reason is established without inventing a new reason catalogue.
3. **Review route impact.** Transport Management determines whether the route can continue without the stop.
4. **Suspend the stop.** The Stop lifecycle moves to Suspended.
5. **Adjust the affected route where necessary.** The route remains within G04 Route Management.
6. **Preserve evidence.** Stop suspension and any route consequence remain historically traceable.

## 10. Alternate Business Flows

- If the route can continue without adjustment, stop suspension may complete without an unnecessary route modification.
- If the consequence is specifically a sequencing change, the Route Sequencing Change scenario remains authoritative for that distinct outcome.
- If the requirement is a temporary diversion, the Route Diversion scenario remains authoritative.
- If suspension is not approved, the stop remains in its current operational condition.

## 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Business Recovery |
|---|---|---|
| Suspension basis unresolved | Stop cannot safely be suspended | Resolve the business basis |
| Suspension not authorised | Current stop condition remains | Continue current operation or pursue an authorised alternative |
| Route cannot operate with stop unavailable | Route requires adjustment | Apply the appropriate G04 route outcome |
| Request is actually diversion/sequencing | Boundary would be exceeded | Use the corresponding canonical scenario |

Recovery is business-level only. No technical retries, queues, rollback or infrastructure failure handling is introduced.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Scenario Effect |
|---|---|
| Transport Stop owns operational status. | Suspension is a Stop lifecycle outcome. |
| Stop availability may be affected by road closure, construction, administrative restriction or emergency diversion. | These are source-supported operational circumstances. |
| Transport Route owns route sequencing and route availability. | Route consequence remains G04-owned. |
| Stop history is preserved. | Suspension remains auditable. |
| Every lifecycle transition is audited. | Suspension and route consequences remain traceable. |

## 13. State and Lifecycle Impact

Stop lifecycle:

`Created → Assigned → Active → Suspended`

This scenario covers:

`Active → Suspended`

The route consequence is an operational adjustment, not a newly invented lifecycle state. If it becomes a distinct sequencing/diversion outcome, the corresponding canonical scenario governs it.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G04 explicitly defines `StopSuspended` — stop suspended.

For a route update, G04 defines `RouteModified` — route updated. Where the route consequence constitutes a route modification, that is the authoritative route event.

**Canonical publisher:** Transport Management (G04).

### 14.2 Synchronous Business Contracts / Hand-offs

The frozen participating-domain set is G only. No additional cross-domain synchronous hand-off is required to complete the scenario. Downstream consumers do not acquire ownership or directly manipulate G04 transport state.

## 15. Business Authorization, Approval and Audit Obligations

Transport Administrator is identified for stop configuration and Transport Manager for route-change approval. These are business responsibilities only.

The suspension basis, decision, route consequence and operational history are auditable.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Stop identified.
- Suspension basis reviewed.
- Stop reaches **Suspended**.
- Required route adjustment is completed through the authoritative G04 route process.
- Stop/route history remains traceable.

### 16.2 Postconditions
- The named business outcome is authoritative within G04.
- Relevant transport history remains traceable.
- No unrelated domain lifecycle is implicitly changed.
- Later allocation, trip and notification activity remains governed by its own canonical scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
The exact route consequence must not be collapsed into a new local state. If it is a distinct sequencing change, diversion or other canonical route outcome, that scenario remains authoritative.

### 17.2 Assumptions
- Institution-specific operational policies remain configuration driven as stated by G04.
- No actor, rule, state or event absent from authoritative architecture is invented.

### 17.3 Deferred Questions
Any material unresolved upstream naming or semantic issue must be resolved at the authoritative architecture level before Architecture Approval/Lock.
