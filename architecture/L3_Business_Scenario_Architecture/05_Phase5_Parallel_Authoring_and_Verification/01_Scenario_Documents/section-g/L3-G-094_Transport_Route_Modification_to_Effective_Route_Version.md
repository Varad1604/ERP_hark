# Transport Route Modification to Effective Route Version

**Scenario ID:** L3-G-094  
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
| Final L3 ID | L3-G-094 |
| Canonical Title | Transport Route Modification to Effective Route Version |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs an authorised modification to an existing operational Transport Route.

The purpose is to ensure that a route change becomes an effective operational outcome only after the route definition, stops, sequencing and operational implications have been reviewed under the Transport Management business process.

The scenario preserves historical route information and does not absorb separate canonical outcomes such as stop creation, route diversion, transport allocation or trip planning.

## 3. Business Outcome

An authorised modification to an existing Transport Route becomes the effective operational route definition, while the prior route context remains historically traceable.

The effective route remains wholly owned by Transport Management.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a business requirement exists to modify an existing Transport Route.

### 4.2 End Boundary

The scenario ends when the authorised revised route becomes the effective operational route.

### 4.3 In Scope

- Identification of the existing route.
- Definition of the proposed route modification.
- Review of affected route definition, stops and sequence.
- Operational review.
- Authorisation of the modification.
- Making the revised route effective.
- Preservation of historical route context.

### 4.4 Out of Scope

- Creation of a new route as a separate business outcome.
- Creation of a new Transport Stop.
- Student/staff/guest allocation.
- Trip execution.
- Vehicle ownership or maintenance.
- Technical versioning mechanisms.

### 4.5 Related Scenarios

- Transport Stop Creation to Active Pickup/Drop Location.
- Route Sequencing Change to Operational Route Update.
- Route Diversion Request to Emergency Operational Route.
- Permanent Route Revision to New Operational Route.

## 5. Ownership and Participating Domains

Transport Management is the sole canonical owner.

The Transport Route aggregate owns:

- route definition;
- route sequencing;
- operational status;
- assigned stops;
- route availability.

No participating domain outside G is introduced.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Transport Administrator | Initiates, evaluates and manages the proposed route modification. |
| Transport Supervisor | Performs configured operational review or approval where required. |
| Downstream route consumers | Consume the effective route outcome without changing route ownership or state. |

## 7. Preconditions and Entry Criteria

- An existing Transport Route is within G04 scope.
- The route has an authoritative current operational definition.
- A legitimate business reason for modification exists.
- The proposed modification can be evaluated against route and operational constraints.
- Existing route history remains available for traceability.

## 8. Trigger

A modification to an existing Transport Route is proposed.

## 9. Main Business Flow

### 9.1 Identify the Existing Route

Transport Management identifies the authoritative Transport Route affected by the proposed change.

The current route remains authoritative until the revised route is approved and made effective.

### 9.2 Define the Proposed Modification

The requested change is translated into an updated route definition.

Depending on the business requirement, this may affect:

- route characteristics;
- assigned stops;
- stop ordering;
- operational availability.

The scenario does not silently create a separate stop lifecycle.

### 9.3 Evaluate Route Integrity

Transport Management evaluates whether the proposed route remains operationally coherent.

The evaluation considers the authoritative route and sequencing responsibilities of G04.

### 9.4 Review the Operational Impact

The proposed modification is subjected to the configured transport operational review.

The purpose is to determine whether the modified route is suitable for operational use.

### 9.5 Authorise the Modification

The authorised transport authority decides whether the revised route may become effective.

If not authorised, the existing route remains effective.

### 9.6 Make the Revised Route Effective

When authorised, Transport Management makes the revised route definition the effective operational route.

The new effective route remains a G04-owned Transport Route outcome.

### 9.7 Preserve Prior Route Context

The prior route context is retained as historical transport information.

The locked G04 architecture states that historical transport operations shall never be deleted.

### 9.8 End Boundary

The scenario ends when the revised route is effective.

Subsequent trips or allocations use their own canonical workflows.

## 10. Alternate Business Flows

### 10.1 Modification Rejected

If the proposed route modification is not approved:

- the current route remains effective;
- the modification does not become operational;
- the review outcome remains auditable.

### 10.2 Invalid Route Sequence

If the modified sequence is not valid:

- the revised route remains non-effective;
- the sequence is corrected;
- the route is re-evaluated.

### 10.3 Stop-Specific Change

If the requested change is actually a new stop or independent stop lifecycle event, the change remains within the appropriate Transport Stop scenario rather than expanding this route-modification scenario.

### 10.4 Temporary Diversion

If the requirement is temporary rather than a permanent/effective route modification, the appropriate Route Diversion scenario remains authoritative.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Consequence | Business Resolution | Owner |
|---|---|---|---|
| Proposed route definition is invalid | Modification cannot become effective | Correct the proposed route | G04 |
| Modified stop sequence is invalid | Revised route cannot become effective | Correct route sequencing | G04 |
| Review is not completed | Effective route cannot change | Complete business review | G04 |
| Modification is rejected | Current route remains authoritative | Resolve or close the proposed change | G04 |
| Historical context would be lost | Auditability would be impaired | Preserve the prior route context | G04 |

No technical rollback is described. The business rule is simply that an unapproved modification does not replace the current effective route.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Transport Route owns route definition. | G04 | Route modification remains G04-owned. |
| Transport Route owns route sequencing. | G04 | Sequence changes remain part of route management. |
| Transport Route owns assigned stops. | G04 | Route modification controls the route's assigned-stop relationship. |
| Transport Route owns operational status and availability. | G04 | Only an authorised modification can change the effective route outcome. |
| Historical transport operations are never deleted. | G04 | Prior route context remains traceable. |
| Transport Management does not own vehicle assets. | G04 | Route changes do not create vehicle ownership. |
| Maintenance execution remains G01-owned. | G01 | Route modification cannot become a maintenance workflow. |
| Downstream consumers do not manipulate transport state directly. | G04 | Effective route remains centrally owned. |

## 13. State and Lifecycle Impact

L2-G04 provides the Route lifecycle:

`Draft → Configured → Active → Suspended → Retired`

For an already active route, this scenario changes the effective route definition while preserving the route's operational lifecycle ownership.

The relevant business outcome is:

`Existing Active Route → Authorised Effective Route Modification`

No additional state is invented for “versioned” route history because the authoritative source does not expose such a dedicated lifecycle state.

Historical route information remains part of the retained transport history.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G04 requires significant transport activity to publish immutable domain events and identifies Route Events.

The exact canonical event name for effective route modification is not exposed in the retrieved authoritative material.

| Business Fact | Canonical Publisher | Consumer |
|---|---|---|
| Effective Transport Route modification | G04 | Authorised downstream transport consumers |

No event name is invented.

### 14.2 Synchronous Business Contracts / Hand-offs

Participating-domain set is G only.

No cross-domain contract is required to complete this scenario.

Downstream consumption cannot transfer route ownership.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Transport Management owns the route modification decision.

The configured Transport Supervisor review/approval path applies where required.

### 15.2 Separation of Duties

No dedicated route-modification SoD rule is exposed in the authoritative material retrieved.

No additional rule is invented.

### 15.3 Audit Evidence

The following must remain traceable:

- existing route;
- proposed modification;
- review;
- decision;
- effective revised route;
- prior route history.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Existing route identified.
- Proposed modification defined.
- Route integrity and sequencing evaluated.
- Required operational review completed.
- Modification authorised.
- Revised route becomes effective.
- Prior route context remains historically traceable.

### 16.2 Postconditions

- The revised route is the effective Transport Route.
- The prior route context remains retained.
- No vehicle, maintenance, student, workforce or financial ownership changes.
- Subsequent transport allocation/trip workflows consume the revised route according to their own scenarios.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical route-modification event name is not exposed in the retrieved G04 source. | Exact event naming cannot be asserted safely. | Confirm G04 Route Event catalogue before Architecture Approval/Lock. |
| The source does not expose a separate route-version lifecycle state. | A synthetic “version” state would be architectural invention. | Treat the modification as an effective route-definition outcome and retain history without inventing a lifecycle state. |

### 17.2 Assumptions

The configured transport workflow determines whether supervisory review is required.

### 17.3 Deferred Questions

Exact route-modification event naming remains a source-resolution item.
