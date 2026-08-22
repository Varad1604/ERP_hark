# Route Diversion Request to Emergency Operational Route

**Scenario ID:** L3-G-117  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, H  
**Scenario Type:** Critical  
**Priority:** P0  
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
| Final L3 ID | L3-G-117 |
| Canonical Title | Route Diversion Request to Emergency Operational Route |
| Primary Owner | Transport Management |
| Participating Domains | G, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional response when an active transport route cannot safely or practically continue on its normal path and an emergency operational diversion is required.

The scenario establishes a governed temporary operational route for the affected transport service, preserves the original route information, and makes the changed transport arrangement available to the operational participants that need to act on the diversion.

## 3. Business Outcome

An authorised emergency route diversion is established for the affected transport operation for the applicable operational period, while the underlying route history remains preserved and affected operational participants can act on the emergency route arrangement.

The scenario does not create a permanent route revision. A permanent change belongs to the separate route-revision outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when an operational need for a temporary emergency diversion is identified for a transport route or trip.

Examples supported by L2-G04 include road closure, traffic incident, weather conditions, public events, or administrative direction.

### 4.2 End Boundary

The scenario ends when the emergency diversion has been operationally reviewed and established as the applicable temporary route arrangement for the affected transport operation.

The end boundary is reached before any later permanent route redesign or post-event review.

### 4.3 In Scope

- Identification of the affected route or transport operation.
- Registration of the emergency diversion request.
- Operational review of the proposed temporary route.
- Assessment of the operational impact.
- Consideration of affected active trips, capacity, vehicle availability, driver assignment and passenger allocation where applicable.
- Required business approval.
- Establishment of the temporary emergency route arrangement.
- Preservation of the original route information.
- Communication of the resulting business fact to relevant operational consumers.
- Recording of the diversion outcome and operational history.

### 4.4 Out of Scope

- Permanent route redesign.
- Creation of a new long-term route policy.
- Vehicle ownership or procurement.
- Vehicle maintenance execution.
- Student identity or employee identity management.
- Financial accounting or transport billing.
- Notification delivery implementation.
- Technical APIs, databases, queues, retry mechanisms, infrastructure or UI design.

## 5. Ownership and Participating Domains

Transport Management is the canonical owner of the transport route and route-diversion business outcome.

Section H participates as an institutional communication/notification consumer where required by the governed notification architecture. Participation does not transfer route ownership.

G04 explicitly states that emergency diversions are temporary operational modifications and that historical route information must remain preserved.

The participating-domain boundary is therefore:

- **G:** owns the route/diversion decision and operational route state.
- **H:** may consume the resulting transport business fact for governed communication.

Student, personnel, finance and maintenance ownership are not transferred into this scenario.

## 6. Business Actors, Roles and Responsibilities

### Transport Administrator / Transport Manager

Responsible for the operational review and decision-making authority assigned by the configured transport policy.

### Transport Supervisor

Coordinates operational assessment and affected transport arrangements where the operational model requires supervisory coordination.

### Transport Coordinator

Coordinates the transport service changes and the resulting operational arrangement.

### Driver / Conductor

Acts on the authorised operational route arrangement during the affected transport service.

These are business-level roles supported by the G04 operational role model. The scenario does not define technical permissions.

### Section H Communication Consumer

May consume the resulting transport business fact where notification or communication is required. H does not decide or own the route diversion.

## 7. Preconditions and Entry Criteria

1. A transport route or transport operation is identifiable.
2. A business condition exists that may require temporary route diversion.
3. The condition is within the scope of transport operational decision-making.
4. The affected operational context can be assessed.
5. The route remains under G04 operational ownership.
6. Any required business approval authority is identifiable through the configured transport policy.
7. The diversion is understood to be temporary; a permanent route revision is not silently substituted.

## 8. Trigger

A road closure, traffic incident, weather condition, public event, administrative direction, or comparable operational condition creates a need to temporarily divert an active or scheduled transport operation.

## 9. Main Business Flow

1. A transport diversion need is identified.
2. The affected route, trip or operational service is identified.
3. The reason for the diversion is established.
4. Transport Management performs an operational review.
5. The review considers the active transport context, including affected trips and operational constraints where relevant.
6. The proposed temporary route arrangement is assessed against operational feasibility.
7. Where configured, the proposed diversion is submitted for the required business approval.
8. The authorised diversion becomes the applicable temporary operational route arrangement.
9. The original route information remains historically preserved.
10. The resulting operational fact is made available to relevant participants and consumers.
11. Drivers and other operational participants act according to the authorised temporary arrangement.
12. The scenario concludes with the emergency diversion established; subsequent permanent revision, if required, is outside this scenario.

## 10. Alternate Business Flows

### 10.1 Diversion Not Required After Review

If operational review determines that the original route can continue safely and appropriately, the diversion is not established. The route remains governed by its existing operational arrangement.

### 10.2 Alternative Temporary Diversion Selected

If the initially considered diversion is not suitable but another temporary route is acceptable, the alternative is assessed and, where authorised, established as the emergency operational route.

### 10.3 Existing Active Trip Is Affected

Where the diversion affects an already planned or active trip, the transport operation is adjusted according to the authorised temporary arrangement rather than treating the diversion as a permanent route change.

### 10.4 Multiple Operational Participants Are Affected

Where the diversion affects drivers, stops, allocations or passenger arrangements, the resulting business change is coordinated with those participants. Their canonical ownership remains unchanged.

## 11. Failure, Interruption and Business Recovery

### 11.1 Diversion Cannot Be Authorised

If the proposed diversion cannot receive the required business approval, it does not become the authorised emergency route. Transport Management must resolve the operational condition through another governed business decision.

### 11.2 Proposed Diversion Is Operationally Unsuitable

If impact assessment determines that the proposed temporary route is unsuitable, the proposal is not established. A different business option may be assessed.

### 11.3 No Acceptable Temporary Route Is Available

If no suitable diversion can be authorised, the affected transport service remains subject to the applicable operational restriction or cancellation decision under the relevant transport workflow. This scenario does not invent a new lifecycle state or emergency rule.

### 11.4 Condition Requires a Permanent Change

If the operational condition is no longer temporary, the matter exits this scenario and is handled by the permanent route revision scenario. The emergency diversion is not silently converted into a permanent route.

## 12. Business Rules and Decision Constraints


- Transport Management (G04) is the canonical owner of transport routes, stops, route sequencing, allocation, trip planning/execution, route diversions and operational transport services.
- Vehicle ownership remains outside G04; maintenance execution remains within G01.
- Student identity remains owned by Section C; employee/driver ownership remains within Section E.
- Financial accounting remains owned by Finance; G04 provides operational transport information rather than executing accounting.
- Transport routes, stop policies, allocation policies, approval workflows and operational transport policies are configuration driven.
- Institutional transport operations support manual operational decisions; route diversion and trip adjustments remain manually overridable.
- Historical transport operations shall not be deleted.

Additional scenario-specific constraints:

- L2-G04 defines emergency diversions as temporary operational modifications.
- Emergency diversion reasons explicitly include road closure, traffic incident, weather conditions, public events and administrative direction.
- Route change validation may consider active trips, route capacity, vehicle availability, driver assignment, student allocation and administrative approval.
- Historical route information must remain preserved.
- Route change validation rules remain configuration driven.
- A diversion must not transfer ownership of the route, driver, vehicle, student, financial or maintenance domains.
- The scenario must remain distinct from permanent route revision.

## 13. State and Lifecycle Impact

The Transport Route lifecycle in G04 is:

`Draft → Configured → Active → Suspended → Retired`

The scenario does not invent an `Emergency` or `Diverted` route lifecycle state.

Emergency diversion is defined by G04 as a temporary operational modification of a route, with its own Route Diversion aggregate. The diversion therefore represents a temporary operational arrangement without redefining the canonical Transport Route lifecycle.

Where the affected operational route is active, the route remains an active route unless an authoritative route-state decision separately requires suspension. The diversion itself does not automatically imply suspension.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The locked G04 event catalogue explicitly defines:

- `RouteCreated`
- `RouteActivated`
- `RouteSuspended`
- `RouteModified`

G04 does **not** define a canonical `RouteDiversionCreated`, `RouteDiverted`, or `EmergencyRouteActivated` event in the supplied event catalogue.

Accordingly, this scenario does not invent one.

Where an established G04 event is materially applicable to an authoritative route state transition, it must retain G04 as publisher. The temporary diversion business fact remains G04-owned.

### 14.2 Synchronous Business Collaboration / Hand-offs

Immediate operational decisions remain within Transport Management.

Section H may consume the resulting transport business fact for notification/communication purposes. The communication concern does not become a second owner of the diversion.

Where passenger or personnel context is needed, G04 consumes authoritative information from the owning domain rather than redefining that ownership.

## 15. Business Authorization, Approval and Audit Obligations

G04 states that route change workflow includes:

`Change Requested → Operational Review → Impact Assessment → Approval → Route Updated → Stakeholders Notified`

The scenario applies that business-level approval structure to an emergency diversion where the applicable transport policy requires approval.

The decision record should preserve the business basis for diversion, affected route/operation, operational assessment, approval decision where applicable, effective temporary arrangement, and historical route context.

Audit infrastructure remains a Platform responsibility; G04 owns the business facts and operational history that must be auditable.

No technical access control or IAM design is specified.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The affected route/operation is identified.
- The emergency condition is established.
- Operational impact has been considered.
- A suitable temporary diversion is selected.
- Required business approval is completed where applicable.
- The temporary emergency route arrangement is authorised.
- Original route history remains preserved.
- Relevant operational participants can act on the authorised arrangement.
- No permanent route revision is implied.

### 16.2 Postconditions

- The emergency operational route arrangement is authoritative for the applicable temporary period.
- The underlying route remains owned by G04.
- The diversion remains distinguishable from permanent route revision.
- Historical route information remains available.
- Any later permanent revision requires its own governed business scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material ownership, lifecycle, rule or boundary conflict was identified in the supplied sources.

A specific canonical event for “route diversion established” is not defined in the G04 event catalogue. This is deliberately recorded rather than invented. The scenario therefore relies only on explicitly defined G04 events where their state-transition semantics actually apply.

### 17.2 Assumptions

- Emergency diversion remains temporary as defined by G04.
- Exact diversion approval authority remains configuration/policy driven.
- Notification delivery remains external to G04.
- The scenario does not assume a new route lifecycle state.

### 17.3 Deferred Questions

If future architecture requires a dedicated diversion event, its canonical name and publisher must be established upstream through architecture governance before being introduced into L3.
