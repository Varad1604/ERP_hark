# Scheduled Trip Cancellation to Operational Closure

**Scenario ID:** L3-G-105  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-105 |
| Canonical Title | Scheduled Trip Cancellation to Operational Closure |
| Primary Owner | Transport Management |
| Participating Domains | G, E, H |
| Scenario Type | Cross-domain |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the business cancellation of a scheduled transport trip before execution and the resulting operational closure. G04 identifies Trip as the aggregate owning the Trip Schedule, operational assignments, Trip Execution and Trip Completion, while its workflow catalogue separately includes scheduled-trip cancellation.

## 3. Business Outcome

An authorised scheduled transport trip is cancelled before execution, its operational service is closed for the scheduled occurrence, and the cancellation remains historically traceable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A scheduled transport trip exists and a legitimate business condition requires that scheduled occurrence to be cancelled before execution.

### 4.2 End Boundary

The scheduled trip has been cancelled and its operational closure is established within G04.

### 4.3 In Scope

- Identify the scheduled Trip.
- Establish the cancellation basis.
- Evaluate the cancellation against applicable operational policy.
- Obtain configured approval where required.
- Cancel the scheduled trip.
- Close the affected operational trip occurrence.
- Preserve trip history and audit evidence.
- Make the cancellation business fact available to participating consumers.

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

The frozen participating-domain set is **G, E, H**. Participation does not transfer canonical ownership.

G04 is independent from fleet ownership, maintenance execution and financial accounting. Section E owns personnel/driver identity; G01 owns maintenance execution; Finance owns financial accounting. Cross-domain participation is consumption/collaboration only and does not transfer Trip ownership.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Transport Administrator / Coordinator | Coordinates operational trip cancellation. |
| Transport Manager / configured authority | Performs required operational review or approval. |
| Driver / operational workforce | May be affected by the cancellation but does not own the Trip aggregate. |
| Parent Communication / downstream operational consumer | May consume the cancellation outcome where governed by its own workflow. |

These are business responsibilities only. Technical authorization mechanisms are outside L3.

## 7. Preconditions and Entry Criteria

- A scheduled Trip exists.
- The trip has an authoritative route and schedule.
- A legitimate business basis for cancellation exists.
- Applicable operational cancellation policy can be evaluated.

## 8. Trigger

An authorised request or operational decision is made to cancel a scheduled trip before execution.

## 9. Main Business Flow

1. Identify the scheduled Trip and its affected operational context.
2. Establish the business reason for cancellation.
3. Validate the cancellation against configured transport policy and the trip's current lifecycle position.
4. Complete any required operational approval.
5. Cancel the scheduled trip before execution begins.
6. Release the scheduled operational commitment as a business outcome, without describing technical resource handling.
7. Preserve the cancellation reason, decision and trip history.
8. Establish the operational closure of the scheduled trip occurrence.
9. Publish/use the authoritative business outcome through the G04 event/collaboration model where defined.

## 10. Alternate Business Flows

- Cancellation is rejected: the scheduled trip remains available for its planned operational execution.
- The trip has already entered execution: the scenario boundary is no longer Scheduled Trip Cancellation; the Trip Execution workflow governs the outcome.
- The cancellation affects passenger communication: H may consume the transport cancellation fact, but notification delivery is outside this scenario.
- A driver or vehicle reassignment is required because the trip remains active: use the applicable assignment workflow rather than treating it as cancellation.

## 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Business Recovery |
|---|---|---|
| Cancellation basis is not accepted | Scheduled trip remains operationally scheduled | Resolve the business decision or retain the trip |
| Required approval is rejected | Cancellation does not take effect | Retain the scheduled trip or pursue an authorised revised decision |
| Trip is already In Progress | Scheduled cancellation is no longer applicable | Use the authoritative Trip Execution workflow |
| Related downstream consequence exists | G04 remains owner of cancellation fact | Allow the consuming domain to perform its own business outcome |

Recovery is expressed only at business level. No technical retry, queue, rollback, server-failure or infrastructure mechanics are introduced.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Scenario effect |
|---|---|
| Trip owns Trip Schedule, Vehicle Assignment, Driver Assignment, Trip Execution and Trip Completion. | Cancellation is a Trip-owned operational outcome. |
| Trip lifecycle includes Planned, Scheduled, Ready, In Progress and Completed. | Cancellation must respect the trip's actual lifecycle position. |
| Every lifecycle transition is audited. | Cancellation remains permanently traceable. |
| Historical trip information shall never be deleted. | Cancellation does not erase the trip history. |
| Configuration over customisation. | Cancellation policy remains institutionally configurable. |
| Driver ownership remains in Section E. | Cancellation does not alter workforce master ownership. |

## 13. State and Lifecycle Impact

G04 defines the Trip lifecycle as `Planned → Scheduled → Ready → In Progress → Completed`. This scenario applies to the scheduled/pre-execution position and terminates the affected scheduled occurrence through the authoritative cancellation workflow. It does not invent a new `Cancelled` lifecycle state unless that state is explicitly confirmed in the authoritative G04 state matrix.

Only authoritative G04 lifecycle terminology is used. No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The retrieved authoritative G04 material establishes event-driven transport architecture but did not expose a canonical event name specifically for Scheduled Trip Cancellation in the material available for this authoring pass. No `TripCancelled` or `ScheduledTripCancelled` event is fabricated. The canonical event name/publisher should be confirmed against the complete G04 event catalogue before Architecture Approval/Lock.

### 14.2 Synchronous Business Collaboration / Hand-offs

The frozen participants are G, E and H. G04 owns the Trip cancellation. E may be affected through driver operational responsibility; H may consume the business cancellation outcome for governed communication. Neither relationship transfers Trip ownership.

Business collaboration is distinct from event publication and does not transfer Trip ownership.

## 15. Business Authorization, Approval and Audit Obligations

Cancellation authority and approval remain subject to configured G04 operational policy. The cancellation reason, decision, affected trip, effective cancellation and audit history must remain traceable.

G04 requires immutable operational audit history and every lifecycle transition is audited.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Scheduled Trip identified.
- Cancellation basis validated.
- Required review/approval completed.
- Trip cancellation becomes effective before execution.
- Operational closure of the scheduled occurrence is established.
- Cancellation history remains auditable.

### 16.2 Postconditions

- The named Trip outcome is authoritative within G04.
- Trip history remains traceable.
- Driver/personnel ownership remains with Section E.
- No unrelated lifecycle is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The core Trip ownership, lifecycle and scheduled-trip cancellation workflow are source-supported. The exact canonical cancellation state/event vocabulary was not exposed in the retrieved G04 material, so it is explicitly deferred rather than invented.

### 17.2 Assumptions

- Transport policies remain configuration driven.
- Business approval remains governed by authoritative G04 policy.
- No actor, rule, state, event, publisher, owner or dependency is invented.
- Where an exact canonical event name was not exposed by the retrieved source, the event is deliberately not fabricated.

### 17.3 Deferred Questions

Any material unresolved lifecycle/event vocabulary must be confirmed through architecture governance before Architecture Approval/Lock.
