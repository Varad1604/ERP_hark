# Vehicle Availability to Operational Vehicle Assignment

**Scenario ID:** L3-G-108  
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
| Final L3 ID | L3-G-108 |
| Canonical Title | Vehicle Availability to Operational Vehicle Assignment |
| Primary Owner | Transport Management |
| Participating Domains | G, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
This scenario governs selection and operational assignment of an available vehicle to a transport Trip. G04 explicitly distinguishes operational vehicle assignment from vehicle asset ownership and defines the Vehicle Assignment aggregate as owning operational assignment, duration, availability and replacement history.

## 3. Business Outcome
An operationally available vehicle is validated and assigned to the required transport Trip, with the assignment becoming effective and historically traceable.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A Trip requires an operational vehicle assignment and an available vehicle is identified for consideration.
### 4.2 End Boundary
The selected vehicle reaches the operationally assigned condition for the Trip.
### 4.3 In Scope
- Identify the Trip requiring a vehicle.
- Identify an operationally available vehicle.
- Validate the vehicle against configured operational requirements.
- Establish the Vehicle Assignment.
- Record assignment duration/context.
- Preserve assignment history.
- Make the operational assignment available to the Trip workflow.
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
| Transport Administrator / Coordinator | Selects and coordinates operational vehicle assignment. |
| Transport Supervisor / configured authority | Reviews assignment where required. |
| Vehicle operational authority | Provides/maintains authoritative operational availability information. |
| H downstream consumer | May consume operational vehicle information. |

## 7. Preconditions and Entry Criteria
- A Trip exists that requires a vehicle.
- An operational vehicle availability context exists.
- Vehicle category/capacity and other applicable operational constraints can be evaluated.

## 8. Trigger
A transport Trip requires an operational vehicle assignment.

## 9. Main Business Flow
1. Identify the Trip and its operational requirements.
2. Identify vehicles currently available for operational assignment.
3. Evaluate the candidate vehicle against applicable configurable constraints, including vehicle category and operational suitability.
4. Select an appropriate available vehicle.
5. Establish the Vehicle Assignment for the Trip.
6. Record the assignment duration and operational context.
7. Preserve the assignment history.
8. Confirm the vehicle assignment as part of the Trip's operational readiness.

## 10. Alternate Business Flows
- No suitable vehicle is available: the Trip cannot complete vehicle assignment until an operational alternative is identified.
- Candidate vehicle fails operational validation: do not assign it; evaluate another suitable available vehicle.
- A vehicle is already assigned elsewhere: it is not treated as available for this Trip.
- An operational problem appears after assignment: use the Vehicle Breakdown/Replacement or Vehicle Operational Issue workflow as applicable.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| No suitable available vehicle | Assignment cannot complete | Identify an operational alternative |
| Candidate fails validation | Vehicle remains unassigned | Select another suitable vehicle |
| Vehicle already committed | Candidate is unavailable for this Trip | Evaluate another available vehicle |
| Issue arises after assignment | Existing assignment may no longer support operation | Apply replacement or operational-issue workflow |

Recovery is business-level only. No technical retries, queues, rollback or infrastructure failure mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule | Effect |
|---|---|
| Vehicle Assignment owns operational vehicle assignment, duration, availability and replacement history. | The assignment remains G04-owned. |
| Vehicle ownership remains outside G04. | Assignment does not create asset ownership. |
| Operational status includes Available, Assigned, In Trip, Restricted, Replacement Assigned and Unavailable. | Candidate selection respects authoritative operational status. |
| Assignment validation is configuration driven. | No institution-specific constraint is invented. |
| Historical assignment information remains available. | Assignment changes remain traceable. |

## 13. State and Lifecycle Impact
The authoritative Vehicle Operational Management model exposes operational states including `Available`, `Assigned`, `In Trip`, `Restricted`, `Replacement Assigned` and `Unavailable`. The workflow is `Vehicle Available → Trip Assignment → Operational Validation → Vehicle Assigned`. This scenario covers the Available-to-Assigned business outcome.

No lifecycle state is invented beyond the authoritative G04 terminology.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
G04 requires significant transport activities to publish immutable domain events, but the retrieved G04 event catalogue did not expose a canonical event name specifically for Vehicle Assignment. No `VehicleAssigned` event is invented.
### 14.2 Synchronous Business Collaboration / Hand-offs
The frozen participants are G and H. G04 owns operational assignment; H may consume operational vehicle information. Vehicle asset ownership remains external and is not transferred through participation.

## 15. Business Authorization, Approval and Audit Obligations
Operational assignment remains a G04 transport responsibility. Configured operational review/approval applies where required. Vehicle, Trip, assignment context, effective assignment and history must remain auditable.

G04 requires immutable operational audit history and historical operational information is not deleted.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Trip requiring vehicle identified.
- Suitable available vehicle identified.
- Operational validation completed.
- Vehicle Assignment established.
- Assignment duration/context recorded.
- Assignment history preserved.
### 16.2 Postconditions
- The named operational vehicle outcome is authoritative within G04.
- Assignment/replacement/recommendation history remains traceable.
- Vehicle asset ownership remains outside G04.
- Maintenance execution remains outside G04.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. The exact canonical Vehicle Assignment event name is not exposed in the retrieved L2 event material; confirm it before Architecture Approval/Lock rather than inventing one.
### 17.2 Assumptions
- Vehicle operational policies remain configuration driven.
- G04 performs operational decisions and recommendations only.
- No actor, rule, state, event, publisher, owner or dependency is invented.
### 17.3 Deferred Questions
Material unresolved event vocabulary must be confirmed through architecture governance before Architecture Approval/Lock.
