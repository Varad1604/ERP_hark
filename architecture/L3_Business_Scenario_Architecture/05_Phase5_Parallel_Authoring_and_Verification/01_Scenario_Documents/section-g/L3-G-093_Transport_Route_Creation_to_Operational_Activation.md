# Transport Route Creation to Operational Activation

**Scenario ID:** L3-G-093  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L1 Business Domain Architecture; L2-G04 Transport Management Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-G-093` |
| Canonical Title | Transport Route Creation to Operational Activation |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |
| Authoritative L2 | `L2-G04` — Transport Management Workflows |

Frozen Phase 5 identity is preserved. Transport Management remains the sole owner of the Transport Route aggregate and its operational lifecycle.

## 2. Scenario Purpose

This scenario governs the complete institutional business outcome in which Transport Management establishes a new Transport Route and brings that route into operational use.

The scenario applies the locked G04 route-management architecture: a Transport Route owns its route definition, route sequencing, operational status, assigned stops and route availability. Route creation is therefore not merely the creation of a route record; it is the institutional process by which a transport path is defined, populated with its required stops, validated for operational sequencing, reviewed and made available for transport operations.

The scenario deliberately stops at route activation. Subsequent transport allocation, trip planning, driver assignment, vehicle assignment and trip execution remain separate canonical outcomes.

## 3. Business Outcome

A valid institutional Transport Route has been established, its required stops have been assigned and sequenced, the route has passed the applicable operational review, and the route is active and available for transport operations.

The completed outcome is authoritative within Transport Management. Downstream consumers may consume the route outcome, but no downstream participant acquires ownership of the Transport Route lifecycle.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when Transport Management receives a business requirement to establish a new operational transport route.

### 4.2 End Boundary

The scenario ends when the authorised route has been activated and is available as an operational transport path.

### 4.3 In Scope

- Establishing the route definition.
- Establishing the intended operational route context.
- Assigning required transport stops.
- Establishing and validating route sequence.
- Reviewing the proposed operational route.
- Authorising route activation.
- Making the route operationally available.
- Preserving route history and audit evidence.

### 4.4 Out of Scope

- Student, staff or guest transport allocation.
- Trip planning or trip scheduling.
- Driver assignment.
- Vehicle assignment.
- Vehicle ownership.
- Vehicle maintenance.
- Student lifecycle.
- Workforce ownership.
- Financial accounting.
- Notification delivery mechanics.
- Technical implementation.

### 4.5 Related Scenarios

The following remain separate canonical outcomes:

- Transport Stop Creation to Active Pickup/Drop Location.
- Route Sequencing Change to Operational Route Update.
- Transport Request to Active Allocation.
- Driver Assignment to Trip Readiness.
- Trip Planning to Ready Transport Trip.
- Vehicle Availability to Operational Vehicle Assignment.

## 5. Ownership and Participating Domains

### 5.1 Canonical Owner

Transport Management is the sole canonical owner of the Transport Route aggregate.

The locked G04 architecture assigns the following to Transport Route:

- Route Definition
- Route Sequencing
- Operational Status
- Assigned Stops
- Route Availability

### 5.2 Participating Domain

The frozen scenario semantics specify **G only**. No additional participating domain is introduced.

Other domains may later consume the activated route as a business fact, but consumption does not transfer ownership.

### 5.3 Ownership Boundaries

Transport Management does not own:

- vehicle assets;
- maintenance execution;
- student identity;
- employee identity;
- financial accounting.

These boundaries remain unchanged when the route becomes active.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Transport Administrator | Establishes and manages the proposed Transport Route and coordinates the route-management process. |
| Transport Supervisor | Provides operational review/decision authority where the configured transport workflow requires supervisory review. |
| Downstream transport consumers | Consume the authoritative route outcome where applicable; they do not modify route state. |

No technical identity profile is defined at L3.

The authoritative G04 source states that transport operations support manual operational decisions and configured operational workflows. Accordingly, this scenario describes business authority rather than technical access-control mechanics.

## 7. Preconditions and Entry Criteria

The following conditions must hold before the route can be activated:

1. There is a legitimate institutional transport requirement for the route.
2. The proposed route is within Transport Management's operational scope.
3. The required route definition can be established.
4. Required Transport Stop information is available.
5. The route can be evaluated against applicable route and sequencing constraints.
6. The route does not attempt to take ownership of an asset or lifecycle belonging to another domain.
7. Any configured operational review required for activation can be performed.

If these conditions cannot be satisfied, the route must not be represented as an active operational route.

## 8. Trigger

A new transport route is proposed for institutional operational use.

The trigger is a business need for a transport path, not a technical creation request.

## 9. Main Business Flow

### 9.1 Establish the Route

Transport Management establishes the proposed Transport Route within the institutional transport context.

The route definition must be sufficiently complete to support the subsequent stop and sequencing decisions.

The route remains a proposed operational object until the activation outcome is reached.

### 9.2 Establish Required Stops

Transport Management identifies and assigns the Transport Stops that form the route.

The route cannot be operationally complete without its required stops.

The Stop aggregate remains independently owned by Transport Management, while the Transport Route owns its assigned-stop relationship.

### 9.3 Establish Route Sequence

Transport Management establishes the intended order in which the route's stops are serviced.

Route sequencing is part of the Transport Route responsibility and must be consistent with the applicable operational constraints.

The scenario does not invent distance, timing, optimisation or geographic rules that are not established by upstream architecture.

### 9.4 Validate Operational Route Definition

Transport Management reviews the route definition, assigned stops and sequence as one operational route.

The purpose is to determine whether the proposed route is sufficiently defined and operationally valid to proceed to activation.

### 9.5 Conduct Operational Review

The route proceeds through the configured operational review.

Where supervisory review is required by the configured transport workflow, the Transport Supervisor performs the business review.

The review is a business decision; no technical approval mechanism is described.

### 9.6 Decide Activation

The authorised Transport Management authority determines whether the route may become operational.

If the route satisfies the applicable business conditions, activation is authorised.

If not, the route remains outside operational use and the identified business issue must be resolved.

### 9.7 Activate Route

Transport Management activates the route for operational use.

Activation makes the Transport Route available as an operational transport path.

### 9.8 Preserve Operational History

The route definition, assigned stops, sequencing, activation decision and resulting operational status remain historically traceable.

The locked G04 architecture requires immutable operational audit history and states that historical transport operations shall never be deleted.

### 9.9 End Boundary

The scenario ends at operational route activation.

It does not continue into allocation, trip planning, driver assignment, vehicle assignment or trip execution.

## 10. Alternate Business Flows

### 10.1 Incomplete Route Definition

If required route information is incomplete:

1. The route does not proceed to activation.
2. The missing business information is identified.
3. Transport Management resolves the route definition.
4. The route returns to operational validation.

### 10.2 Invalid Stop Set

If the proposed stop set is not sufficient for the intended route:

1. Activation is withheld.
2. Transport Management resolves the stop assignment.
3. The route is re-evaluated.

The scenario does not create a new stop lifecycle; stop creation or modification remains separately governed.

### 10.3 Invalid Route Sequence

If the proposed sequence does not satisfy applicable route constraints:

1. The route remains non-operational.
2. The sequence is corrected through Route Management.
3. The route is reviewed again.

### 10.4 Review Not Approved

If operational review does not authorise activation:

1. The route remains outside operational use.
2. The review outcome is recorded.
3. Transport Management resolves the identified issue where appropriate.
4. Activation is reconsidered through the authorised business process.

### 10.5 Route Requirement Withdrawn

If the institutional requirement for the new route is withdrawn before activation:

1. Activation does not occur.
2. The route remains non-operational.
3. The business outcome is closed according to the applicable route-management process.

No unsupported lifecycle state name is introduced for this closure.

## 11. Failure, Interruption and Business Recovery

Failure and recovery are expressed only at the business level.

| Business Situation | Consequence | Business Recovery | Owner |
|---|---|---|---|
| Required route information is incomplete | Route cannot be activated | Correct the route definition before activation | G04 |
| Required stop assignment is incomplete | Operational route cannot be completed | Complete the required stop assignment | G04 |
| Route sequence is not acceptable | Route cannot be activated | Correct route sequencing and re-evaluate | G04 |
| Operational review is unresolved | Activation cannot be authorised | Complete the configured review/decision process | G04 |
| Route requirement is no longer valid | New route should not become active | Close the proposed business outcome without activation | G04 |

No technical retries, queues, rollback, exception stacks, server failures or infrastructure recovery are described.

## 12. Business Rules and Decision Constraints

The following constraints are grounded in L2-G04:

| Rule / Constraint | Owner | Effect |
|---|---|---|
| Transport Management owns transport operations. | G04 | Route creation and activation remain G04-owned. |
| Transport Route owns route definition. | G04 | Route definition is not delegated to another domain. |
| Transport Route owns route sequencing. | G04 | Sequence is part of the route aggregate responsibility. |
| Transport Route owns assigned stops. | G04 | Stop association remains within the route boundary. |
| Transport Route owns operational status and route availability. | G04 | Activation establishes the route's operational availability. |
| Transport routes and policies are configuration driven. | G04 | Institutional transport configuration governs operational conditions. |
| Significant transport activity is event driven and auditable. | G04 | Route activation must remain traceable. |
| Historical transport operations are never deleted. | G04 | Route history is retained. |
| Vehicle ownership remains outside Transport Management. | G04 | Route activation does not create vehicle ownership. |
| Maintenance execution remains with G01. | G01 | Route activation does not create maintenance responsibility. |

## 13. State and Lifecycle Impact

The locked G04 architecture provides a Route lifecycle overview containing:

`Draft → Configured → Active → Suspended → Retired`

For this scenario, the relevant business progression is:

`Draft → Configured → Active`

The scenario must not invent additional states between these canonical route states.

| Lifecycle Stage | Business Meaning |
|---|---|
| Draft | Route is being established and is not yet an operational route. |
| Configured | Route definition and operational configuration have been established sufficiently for review/activation. |
| Active | Route is available for operational transport use. |

The transition to **Active** is the scenario's completion boundary.

Suspension, retirement and subsequent route changes belong to separate lifecycle outcomes.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

Locked L2-G04 defines the canonical Route events `RouteCreated` and `RouteActivated`. This scenario concludes with activation, so the completion event is:

| Business Event | Canonical Publisher | Consumer |
|---|---|---|
| `RouteActivated` | G04 Transport Management | Authorised downstream transport consumers |

`RouteCreated` represents the earlier route-creation fact. It is not renamed or republished by another domain.

### 14.2 Synchronous Business Contracts / Hand-offs

The frozen Stage 3 participating-domain set is G only.

Accordingly, this scenario does not introduce a cross-domain synchronous business contract as part of its completion.

Downstream consumers may consume the authoritative activated-route fact, but:

- they do not own Transport Route;
- they do not directly manipulate route state;
- they do not determine route activation.

This preserves the G04 canonical ownership boundary.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

Transport Management owns the business decision to activate a route.

Where configured transport governance requires supervisory review, the Transport Supervisor participates in that operational review.

### 15.2 Separation of Duties

The locked G04 source establishes operational review/approval as part of route management but does not define a dedicated route-creation separation-of-duty rule.

No additional SoD rule is invented.

### 15.3 Audit Obligations

The following business evidence must remain historically traceable:

- route definition;
- assigned stops;
- route sequence;
- operational review;
- activation decision;
- resulting operational status;
- relevant route history.

The locked G04 architecture explicitly requires immutable operational audit history and retention of historical transport operations.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

The scenario is complete only when:

- the Transport Route is sufficiently defined;
- required stops are assigned;
- route sequencing is established and validated;
- configured operational review is complete;
- activation is authorised;
- the route is Active and available for transport operations;
- `RouteActivated` is published by G04;
- the activation and route history are auditable.

### 16.2 Postconditions

- The Transport Route is an active operational route.
- Its assigned stops and sequence form part of the authoritative route definition.
- Route availability reflects the active operational outcome.
- Historical route information remains retained.
- No student, employee, vehicle, maintenance or financial ownership has transferred to G04.
- Subsequent allocation and trip operations remain separate canonical scenarios.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No unresolved event-name, lifecycle-state, owner, or publisher issue remains for this scenario after direct reconciliation with locked L2-G04.

### 17.2 Assumptions

- Any route approval requirement is governed by the configured transport workflow.
- Route-specific operational constraints are those established by authoritative G04 configuration and rules.
- No route optimisation rule is assumed unless defined upstream.

### 17.3 Deferred Questions

Any additional institution-specific route approval matrix remains configuration governed and is outside the canonical L3 scenario.
