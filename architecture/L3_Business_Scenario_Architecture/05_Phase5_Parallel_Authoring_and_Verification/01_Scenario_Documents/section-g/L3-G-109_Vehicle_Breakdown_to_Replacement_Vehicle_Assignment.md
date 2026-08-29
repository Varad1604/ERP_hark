# Vehicle Breakdown to Replacement Vehicle Assignment

**Scenario ID:** L3-G-109  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, H  
**Scenario Type:** Critical  
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
| Final L3 ID | L3-G-109 |
| Canonical Title | Vehicle Breakdown to Replacement Vehicle Assignment |
| Primary Owner | Transport Management |
| Participating Domains | G, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
This scenario governs the operational response when a vehicle supporting a transport operation breaks down and a suitable replacement vehicle is assigned. G04 explicitly identifies breakdown as a reason for vehicle replacement while keeping maintenance execution outside Transport Management.

## 3. Business Outcome
The affected transport operation receives an operational replacement vehicle assignment and the replacement history is preserved, while any maintenance need is kept separate from the replacement assignment outcome.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An operationally assigned vehicle becomes unavailable because of a breakdown during the transport operation or before the relevant trip can proceed.
### 4.2 End Boundary
A suitable replacement vehicle is operationally assigned to the affected Trip/transport operation.
### 4.3 In Scope
- Identify the affected Trip and vehicle.
- Establish the breakdown condition.
- Determine operational impact.
- Identify a suitable replacement vehicle.
- Validate and assign the replacement.
- Preserve replacement history.
- Separate maintenance recommendation/execution from operational replacement.
### 4.4 Out of Scope
- Vehicle asset ownership, procurement, registration, disposal or depreciation.
- Maintenance execution/work orders/vendor repair.
- Driver/personnel master ownership.
- Financial accounting.
- Technical APIs, databases, services, queues, deployment, infrastructure, UI or IAM/RBAC.
- Independent notification delivery.
- Any later business outcome not required to reach the frozen scenario outcome.

## 5. Ownership and Participating Domains
Transport Management (G04) owns the operational vehicle assignment outcome. Vehicle ownership remains outside G04 and maintenance execution remains with G01.

The frozen participating-domain set is **G, H**. Participation does not transfer canonical ownership.

G04 owns operational vehicle assignment, availability and replacement history; vehicle asset ownership remains outside Transport Management. Maintenance execution remains with G01. Transport Management may recommend maintenance but does not execute it.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Coordinator / Supervisor | Coordinates the replacement decision. |
| Assigned Driver / operational workforce | Provides operational information about the vehicle condition where applicable. |
| Transport Administrator / configured authority | Establishes the replacement assignment. |
| G01 maintenance authority | Receives maintenance recommendation/service need; owns maintenance execution. |
| H downstream consumer | May consume the operational replacement information. |

## 7. Preconditions and Entry Criteria
- A vehicle is assigned to or supporting a Trip.
- A breakdown condition is identified.
- Vehicle availability information can be evaluated for replacement selection.

## 8. Trigger
An assigned operational vehicle breaks down and can no longer support the affected transport operation.

## 9. Main Business Flow
1. Identify the affected Trip and vehicle assignment.
2. Establish the breakdown as an operational condition.
3. Determine whether the current vehicle can continue; if not, treat it as unavailable for the affected operation.
4. Identify available replacement vehicles.
5. Validate the replacement candidate against operational requirements.
6. Establish the replacement Vehicle Assignment.
7. Record the replacement and preserve prior assignment history.
8. Where the breakdown indicates maintenance need, create the business maintenance recommendation outcome without executing maintenance in G04.
9. Confirm the affected transport operation has an operational vehicle assignment.

## 10. Alternate Business Flows
- No replacement vehicle is available: the affected Trip cannot regain vehicle readiness through this scenario alone.
- Candidate replacement is unsuitable: evaluate another available candidate.
- Breakdown requires maintenance: G04 recommends/raises the maintenance need; G01 owns execution.
- The route or Trip itself must change: use the appropriate Trip/Route workflow rather than hiding that outcome inside vehicle replacement.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| No replacement available | Operational vehicle assignment cannot be restored | Resolve through authorised transport operational decision |
| Replacement candidate unsuitable | Candidate is not assigned | Select another suitable vehicle |
| Breakdown requires maintenance | Vehicle remains outside normal operation | Provide maintenance recommendation/service request to G01 |
| Trip impact exceeds vehicle replacement | Replacement alone is insufficient | Apply the authoritative Trip/Route operational workflow |

Recovery is business-level only. No technical retries, queues, rollback or infrastructure failure mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule | Effect |
|---|---|
| Vehicle Replacement is supported due to Breakdown, Maintenance Recommendation, Administrative Decision or Emergency Requirement. | Breakdown is an authoritative replacement trigger. |
| Vehicle Assignment owns replacement history. | Previous and replacement assignments remain traceable. |
| Vehicle ownership remains outside G04. | Replacement does not alter asset ownership. |
| Maintenance execution remains within G01. | G04 cannot perform repairs/work orders. |
| Operational vehicle states are configurable and include `Unavailable` and `Replacement Assigned`. | Breakdown can make the original vehicle unavailable and replacement operationally assigned. |

## 13. State and Lifecycle Impact
The vehicle operational model includes `Available`, `Assigned`, `In Trip`, `Restricted`, `Replacement Assigned` and `Unavailable`. Breakdown causes the original operational availability to become unsuitable for the affected operation; the business outcome is replacement assignment. No additional lifecycle state is invented.

No lifecycle state is invented beyond the authoritative G04 terminology.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
The retrieved G04 material establishes immutable domain events but does not expose a canonical breakdown/replacement event name. No `VehicleBreakdown` or `ReplacementVehicleAssigned` event is fabricated.
### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participants are G and H. G04 owns the replacement assignment. H may consume the operational outcome. G01 is a maintenance owner/dependency but is not a frozen participating domain for this scenario; maintenance execution remains outside this L3 outcome.

## 15. Business Authorization, Approval and Audit Obligations
Transport operational authority determines replacement according to configurable policy. Breakdown evidence, affected vehicle, replacement vehicle, effective assignment and history must remain auditable. Any maintenance recommendation is separately traceable.

G04 requires immutable operational audit history and historical operational information is not deleted.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Breakdown identified.
- Affected assignment evaluated.
- Replacement vehicle identified and validated.
- Replacement Vehicle Assignment becomes effective.
- Replacement history preserved.
- Any maintenance need is separated from the operational replacement outcome.
### 16.2 Postconditions
- The named operational vehicle outcome is authoritative within G04.
- Assignment/replacement/recommendation history remains traceable.
- Vehicle asset ownership remains outside G04.
- Maintenance execution remains outside G04.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. Exact canonical breakdown/replacement event names are not exposed in retrieved G04 event material and must not be invented.
### 17.2 Assumptions
- Vehicle operational policies remain configuration driven.
- G04 performs operational decisions and recommendations only.
- No actor, rule, state, event, publisher, owner or dependency is invented.
### 17.3 Deferred Questions
Material unresolved event vocabulary must be confirmed through architecture governance before Architecture Approval/Lock.
