# Trip Execution with Route Deviation to Operational Completion

**Scenario ID:** L3-G-107  
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
| Final L3 ID | L3-G-107 |
| Canonical Title | Trip Execution with Route Deviation to Operational Completion |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs a transport Trip that is already executing but cannot continue exactly along its planned route and therefore requires a business-level route deviation before operational completion. G04 identifies Route Diversion/Route Change as an operational capability and keeps Trip Execution within the Trip aggregate.

## 3. Business Outcome

An executing transport Trip is completed through an authorised operational route deviation, with the deviation and resulting completion preserved in the Trip's operational history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A Trip is In Progress and a business condition requires departure from the planned route.

### 4.2 End Boundary

The affected Trip is operationally completed after the authorised deviation, with the deviation outcome recorded.

### 4.3 In Scope

- Identify the executing Trip and planned route.
- Establish the route-deviation reason.
- Determine an authorised operational route adjustment.
- Apply the business deviation for the affected journey.
- Conduct the remainder of the trip under the revised operational route.
- Complete the Trip.
- Preserve deviation and completion history.

### 4.4 Out of Scope

- Vehicle ownership, procurement, asset lifecycle or maintenance execution.
- Driver/personnel master ownership.
- Student lifecycle ownership.
- Financial accounting.
- Technical APIs, databases, services, queues, deployment, infrastructure, UI or technical IAM/RBAC.
- Independent notification delivery.
- Any downstream business outcome that is not part of the frozen scenario title.

## 5. Ownership and Participating Domains

Transport Management (G04) is the canonical owner of the Trip operational outcome. G04 owns the Trip aggregate, including Trip Schedule, Vehicle Assignment, Driver Assignment, Trip Execution and Trip Completion. Driver ownership remains with Section E; G04 performs operational assignment only.

The frozen participating-domain set is **G**. Participation does not transfer canonical ownership.

G04 is independent from fleet ownership, maintenance execution and financial accounting. Section E owns personnel/driver identity; G01 owns maintenance execution; Finance owns financial accounting. Cross-domain participation is consumption/collaboration only and does not transfer Trip ownership.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Transport Coordinator / Supervisor | Coordinates the operational route deviation. |
| Assigned Driver | Executes the approved operational route adjustment. |
| Transport Manager / configured authority | Provides required operational approval where policy requires it. |
| H downstream consumer | May consume the resulting operational information. |

These are business responsibilities only. Technical authorization mechanisms are outside L3.

## 7. Preconditions and Entry Criteria

- Trip is in the canonical In Progress state.
- A planned route exists.
- A material operational condition requires route deviation.
- The business route-change/diversion process can be applied.

## 8. Trigger

During Trip execution, an operational condition makes the planned route unsuitable or temporarily unavailable.

## 9. Main Business Flow

1. Identify the executing Trip and the affected planned route segment.
2. Establish and record the business reason for deviation.
3. Determine the appropriate authorised route diversion or operational adjustment.
4. Obtain any required operational approval under configured policy.
5. Apply the deviation to the affected trip execution.
6. Continue the journey using the authorised operational route.
7. Record the deviation and material execution outcome in trip history.
8. Complete the Trip when the journey's business completion condition is satisfied.
9. Preserve the completed Trip and deviation history.

## 10. Alternate Business Flows

- No safe/authorised operational route adjustment is available: the trip cannot simply be treated as normally completed; the applicable G04 operational recovery decision governs the outcome.
- The issue is a permanent route redesign rather than a trip-specific deviation: use the Permanent Route Revision scenario.
- The issue affects only a temporary stop: use the Temporary Stop Change scenario where applicable.
- The deviation becomes an emergency operational route: use the authoritative Route Diversion workflow.

## 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Business Recovery |
|---|---|---|
| Planned route becomes unsuitable | Normal route execution is interrupted | Apply authorised route diversion/change workflow |
| Required approval unavailable | Deviation cannot become authorised | Resolve business authority condition |
| No viable alternative route | Trip completion condition may remain unresolved | Apply G04 operational recovery decision |
| Permanent route change required | Trip-specific deviation is insufficient | Use Permanent Route Revision scenario |
| Deviation succeeds and journey completes | Trip reaches Completed | Preserve deviation and completion history |

Recovery is expressed only at business level. No technical retry, queue, rollback, server-failure or infrastructure mechanics are introduced.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Scenario effect |
|---|---|
| Trip owns Trip Execution and Trip Completion. | Deviation remains part of the G04 Trip outcome. |
| Route Diversion is a temporary operational modification of a transport route. | A trip-specific deviation must not silently redefine the permanent route. |
| Every Trip shall reference exactly one Route. | The executing Trip retains its authoritative route relationship. |
| Every lifecycle transition is audited. | Deviation and completion remain traceable. |
| Historical trip information shall never be deleted. | Original and deviation history remain available. |
| Configuration over customisation. | Operational deviation policy remains configurable. |

## 13. State and Lifecycle Impact

Canonical Trip lifecycle: `Planned → Scheduled → Ready → In Progress → Completed`. The scenario operates within `In Progress` and ends at `Completed`. Route deviation is an operational business adjustment, not a new Trip lifecycle state.

Only authoritative G04 lifecycle terminology is used. No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

G04 explicitly identifies Route Diversion and Route Change as transport capabilities and requires significant transport activities to publish immutable domain events. However, the exact canonical event name for a Trip-specific route deviation was not exposed in the retrieved material. No `TripRouteDeviation` or equivalent event is invented; confirm the canonical event catalogue before Architecture Approval/Lock.

### 14.2 Synchronous Business Collaboration / Hand-offs

The frozen participating-domain set is G only. The scenario remains entirely within Transport Management, with route/Trip operational ownership retained by G04. Driver/personnel and maintenance ownership remain outside the scenario's canonical ownership boundary.

Business collaboration is distinct from event publication and does not transfer Trip ownership.

## 15. Business Authorization, Approval and Audit Obligations

Route deviation decisions and any required operational approval remain subject to configured G04 transport policy. The deviation reason, decision, effective operational route, execution outcome and completion evidence must remain auditable.

G04 requires immutable operational audit history and every lifecycle transition is audited.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Executing Trip identified.
- Deviation reason established.
- Authorised operational route adjustment determined.
- Deviation applied to the affected journey.
- Journey completed.
- Trip reaches **Completed**.
- Deviation and completion history preserved.

### 16.2 Postconditions

- The named Trip outcome is authoritative within G04.
- Trip history remains traceable.
- Driver/personnel ownership remains with Section E.
- No unrelated lifecycle is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The Trip lifecycle, Route Diversion capability and ownership boundaries are source-supported. The exact canonical event name for a trip-specific route deviation was not exposed in the retrieved G04 source, so it is explicitly deferred rather than fabricated.

### 17.2 Assumptions

- Transport policies remain configuration driven.
- Business approval remains governed by authoritative G04 policy.
- No actor, rule, state, event, publisher, owner or dependency is invented.
- Where an exact canonical event name was not exposed by the retrieved source, the event is deliberately not fabricated.

### 17.3 Deferred Questions

Any material unresolved lifecycle/event vocabulary must be confirmed through architecture governance before Architecture Approval/Lock.
